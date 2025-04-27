use alloy_chains::NamedChain;

#[tokio::main]
async fn main() {
    scanner::bootstrap(NamedChain::Base).await.unwrap();
}
