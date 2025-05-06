use num_bigint::BigUint;
use zkp_chaum_pedersen::ZKP;
use crate::zkp_auth::auth_client::AuthClient;
use crate::zkp_auth::RegisterRequest;

pub mod zkp_auth{
    include!("./zkp_auth.rs");
}
#[tokio::main]
async fn main() {
    let mut buf = String::new();
    let (alpha, beta,p,q) = ZKP::get_constants();
    let zkp = ZKP {
        p: p.clone(),
        q: q.clone(),
        alpha: alpha.clone(),
        beta : beta.clone(),

    };
    let mut client = AuthClient::connect("http://127.0.0.1:8080").await.expect("Failed to connect to server");
    println!("Connected to the server");

    println!("Please provide your username");
    std::io::stdin().read_line(&mut buf).expect("Could not get the username from the stdin");
    let username = buf.trim().to_string();
    buf.clear();

    println!("Please provide your password");
    std::io::stdin().read_line(&mut buf).expect("Could not get the password from the stdin");
    let password =BigUint::from_bytes_be(buf.trim().as_bytes()) ;
    buf.clear();

    let (y1,y2) = zkp.compute_pair(&password);

    let request = RegisterRequest {
        user : username.clone(),
        y1 : y1.to_bytes_be(),
        y2 : y2.to_bytes_be(),
    };
    let _response = client
        .register(request)
        .await
        .expect("Could not register in server");

    println!("✅ Registration was successful");
}