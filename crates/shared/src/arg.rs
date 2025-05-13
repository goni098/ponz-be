use alloy_chains::NamedChain;

pub fn parse_chain_arg() -> NamedChain {
    let args: Vec<String> = std::env::args().collect();

    let chain_id = args
        .get(2)
        .expect(format!("Missing chain_id arg, actual args: {:?}", args).as_str())
        .parse::<u64>()
        .expect("Expected u64 chain_id");

    NamedChain::try_from(chain_id).expect("Invlaid chain_id arg")
}
