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
            let auth_id = "skdjfsk".to_string();

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
    async fn verify_authentication(&self, request: Request<AuthenticationAnswerRequest>) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        todo!()
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