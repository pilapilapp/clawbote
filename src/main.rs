use ethers::{
    prelude::*,
    providers::{Provider, Http},
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to a local Ethereum node (like Ganache) or a testnet
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    
    // Get the latest block number
    let block = provider.get_block_number().await?;
    println!("Current block: {}", block);

    Ok(())
}
