use std::io::stdin;
use num_bigint::BigUint;

pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use zkp_auth::{auth_client::AuthClient, RegisterRequest};
use chaum_pedersen::ZKP;

use crate::zkp_auth::{AuthenticationAnswerRequest, AuthenticationChallengeRequest};

#[tokio::main]
async fn main() {
    let mut buf = String::new();
    let (alpha, beta, p, q) = ZKP::get_constants();
    let zkp = ZKP {alpha: alpha.clone(), beta: beta.clone(), p: p.clone(), q: q.clone()};

    let mut client = AuthClient::connect("http://127.0.0.1:50051").await.expect("Could not connect to server");
    println!("✅ Connected to server!");

    println!("Please provide your username:");
    stdin().read_line(&mut buf).expect("Could not get username");
    let user_name = buf.trim().to_string(); 
    buf.clear();

    println!("Please provide your password:");
    stdin().read_line(&mut buf).expect("Could not get password");
    let password = BigUint::from_bytes_be(buf.trim().as_bytes());
    buf.clear();

    let y1 = ZKP::exponentiate(&alpha, &password, &p);
    let y2 = ZKP::exponentiate(&beta, &password, &p);

    let request = RegisterRequest {
        user: user_name.clone(),
        y1: y1.to_bytes_be(),
        y2: y2.to_bytes_be(),
    };
    let _response = client.register(request).await.expect("Could not register user");
    println!("{:?}", _response);

    println!("Please provide your password to login:");
    stdin().read_line(&mut buf).expect("Could not get password");
    let password = BigUint::from_bytes_be(buf.trim().as_bytes());
    buf.clear();

    let k = ZKP::generate_random_number_below(&q);
    let r1 = ZKP::exponentiate(&alpha, &k, &p);
    let r2 = ZKP::exponentiate(&beta, &k, &p);

    let request = AuthenticationChallengeRequest {
        user: user_name,
        r1: r1.to_bytes_be(),
        r2: r2.to_bytes_be(),
    };

    let response = client.authentication_challenge(request).await.expect("Could not request challenge to server").into_inner();

    let auth_id = response.auth_id;
    let c = BigUint::from_bytes_be(&response.c);

    let s = zkp.solve(&k, &c, &password);

    let request = AuthenticationAnswerRequest {
        auth_id,
        s: s.to_bytes_be(),
    };
    let response = client.verify_authentication(request).await.expect("Could not verify authentication in server").into_inner();
    println!("You logged in! Session ID: {}", response.session_id);
}
