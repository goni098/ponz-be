use alloy_chains::NamedChain;

#[tokio::main]
async fn main() {
    operator::bootstrap(NamedChain::Sepolia).await.unwrap();
}
