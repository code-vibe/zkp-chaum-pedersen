pub mod zkp_auth{
    include!("./zkp_auth.rs");
}

use std::str::FromStr;
use num_bigint::BigUint;
use std::{collections::HashMap, sync::Mutex};
use tonic::{Request, Response, Status};
use tonic::transport::Server;
use zkp_auth::auth_server::{AuthServer, Auth};
use crate::zkp_auth::{AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest, AuthenticationChallengeResponse, RegisterRequest, RegisterResponse};

#[derive(Debug,Default)]
struct  AuthImpl {
    pub user_info: Mutex<HashMap<String, UserInfo>>,
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
        todo!()
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