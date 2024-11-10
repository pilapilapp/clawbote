use ethers::{
    providers::{Provider, Http},
    utils::format_units,
    types::{Address, U256},
    middleware::Middleware,
};
use eyre::{Result, eyre};
use std::str::FromStr;

pub struct AccountService {
    provider: Provider<Http>,
}

const DEFAULT_NODE_URL: &str = "https://rpc.ankr.com/eth";

type EthereumAddress = Address;
type Wei = U256;

#[allow(dead_code)]
impl AccountService {
    pub fn new() -> Result<Self> {
        let provider = Provider::<Http>::try_from(DEFAULT_NODE_URL)?;
        Ok(Self { provider })
    }

    pub async fn get_balance(&self, address: &str) -> Result<f64> {
        let address: EthereumAddress = Address::from_str(address)
            .map_err(|e| eyre!("Invalid Ethereum address: {}", e))?;
        let balance: Wei = self.provider.get_balance(address, None).await?;
        let balance_eth = format_units(balance, "ether")?.parse()?;
        Ok(balance_eth)
    }

    pub async fn get_transaction_count(&self, address: &str) -> Result<u64> {
        let address: EthereumAddress = Address::from_str(address)?;
        let count = self.provider.get_transaction_count(address, None).await?;
        Ok(count.as_u64())
    }

    pub async fn get_code(&self, address: &str) -> Result<Vec<u8>> {
        let address: EthereumAddress = Address::from_str(address)?;
        let code = self.provider.get_code(address, None).await?;
        Ok(code.to_vec())
    }

    pub async fn get_code_size(&self, address: &str) -> Result<usize> {
        let address: EthereumAddress = Address::from_str(address)?;
        let code = self.provider.get_code(address, None).await?;
        Ok(code.len())
    }
} 