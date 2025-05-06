pub mod zkp_auth{
    include!("./zkp_auth.rs");
}

use std::str::FromStr;
use num_bigint::BigUint;
use std::{collections::HashMap, sync::Mutex};
use tonic::{Code, Request, Response, Status};
use tonic::transport::Server;
use zkp_auth::auth_server::{AuthServer, Auth};
use zkp_chaum_pedersen::ZKP;
use crate::zkp_auth::{AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest, AuthenticationChallengeResponse, RegisterRequest, RegisterResponse};

#[derive(Debug,Default)]
struct  AuthImpl {
    pub user_info: Mutex<HashMap<String, UserInfo>>,
    pub auth_id_to_username: Mutex<HashMap<String, String>>,
}
#[derive(Debug,Default)]
pub struct UserInfo {
    //registration
    pub username: String,
    pub y1 : BigUint,
    pub y2 : BigUint,
    //authorization
    pub r1 : BigUint,
    pub r2 : BigUint,
    //verification
    pub c : BigUint,
    pub s : BigUint,
    pub session_id : BigUint,
}
#[tonic::async_trait]
impl Auth for AuthImpl {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        println!("Processing a register request: {:?}", request);
        let request = request.into_inner();

        let mut user_info = UserInfo::default();

        user_info.username = request.user.clone();
        user_info.y1 = BigUint::from_bytes_be(&request.y1);
        user_info.y2 = BigUint::from_bytes_be(&request.y2);

        let user_info_hashmap = &mut self.user_info.lock().unwrap();
        user_info_hashmap.insert(request.user.clone(), user_info);
        Ok(Response::new(RegisterResponse {}))
    }
    async fn create_authentication_challenge(&self, request: Request<AuthenticationChallengeRequest>) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        println!("Processing a create_authentication_challenge request: {:?}", request);

        let request = request.into_inner();
        let username = request.user;
        let  user_info_hashmap = &mut self.user_info.lock().unwrap();

        if let Some(user_info) = user_info_hashmap.get_mut(&username) {
            user_info.r1 = BigUint::from_bytes_be(&request.r1);
            user_info.r2 = BigUint::from_bytes_be(&request.r2);

            let (_,_,_,q) = ZKP::get_constants();
            let c = ZKP::generate_random_below(&q);

            let c = BigUint::from(666u32);
            let auth_id = ZKP::generate_random_string(12);

            let mut auth_id_to_username = &mut self.auth_id_to_username.lock().unwrap();
            auth_id_to_username.insert(auth_id.clone(), user_info.username.clone());
            println!("✅ Successful Challenge Request username: {:?}", user_info.username);

            Ok(Response::new(AuthenticationChallengeResponse {
                auth_id,
                c: c.to_bytes_be(),
            }))
        }else {
            Err(Status::new(
                Code::NotFound,
                format!("User: {} not found in database", username),
            ))
        }
    }
    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        let request = request.into_inner();

        let auth_id = request.auth_id;
        println!("Processing Challenge Solution auth_id: {:?}", auth_id);

        let auth_id_to_user_hashmap = &mut self.auth_id_to_username.lock().unwrap();

        if let Some(user_name) = auth_id_to_user_hashmap.get(&auth_id) {
            let user_info_hashmap = &mut self.user_info.lock().unwrap();
            let user_info = user_info_hashmap
                .get_mut(user_name)
                .expect("AuthId not found on hashmap");

            let s = BigUint::from_bytes_be(&request.s);
            user_info.s = s;

            let (alpha, beta, p, q) = ZKP::get_constants();
            let zkp = ZKP { alpha, beta, p, q };

            let verification = zkp.verify(
                &user_info.r1,
                &user_info.r2,
                &user_info.y1,
                &user_info.y2,
                &user_info.c,
                &user_info.s,
            );

            if verification {
                let session_id = ZKP::generate_random_string(12);

                println!("✅ Correct Challenge Solution username: {:?}", user_name);

                Ok(Response::new(AuthenticationAnswerResponse { session_id }))
            } else {
                println!("❌ Wrong Challenge Solution username: {:?}", user_name);

                Err(Status::new(
                    Code::PermissionDenied,
                    format!("AuthId: {} bad solution to the challenge", auth_id),
                ))
            }
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("AuthId: {} not found in database", auth_id),
            ))
        }
    }
}
#[tokio::main]
async fn main() {
    let address = "127.0.0.1:8080".to_string();
    println!("Running the server on {}", address);
    let auth_impl = AuthImpl::default();
    Server::builder()
        .add_service(AuthServer::new(auth_impl))
        .serve(address.parse().expect("Could not convert address"))
        .await
        .unwrap();
}