use axum::{extract::State, routing::get, Router};
use ethers::prelude::*;

#[tokio::main]
async fn main() {
    let rpc_url = "https://mainnet.infura.io/v3/"; //declare our rpc url
    let provider = Provider::try_from(rpc_url).unwrap(); // initiates the provider no error handleing error prone

    let app = Router::new().route("/", get(handler)).with_state(provider); //with state adding our provider above so we can share it and use it in other fucntions;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
//gets the current block number on the ethereum blockchain
async fn handler(State(state): State<Provider<Http>>) -> String {
    let blk = state.get_block_number().await.ok(); // no error handeling asumes its ok so not blcoking io
    format!("this is it: {}", blk.unwrap())
}
