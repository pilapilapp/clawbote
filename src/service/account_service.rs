use ethers::{
    providers::{Provider, Http},
    utils::format_units,
    middleware::Middleware,
    types::Address,
};
use eyre::Result;
use std::str::FromStr;

pub struct AccountService {
    provider: Provider<Http>,
}

impl AccountService {
    pub fn new() -> Result<Self> {
        let provider = Provider::<Http>::try_from("http://localhost:8545")?;
        Ok(Self { provider })
    }

    pub async fn get_balance(&self, address: &str) -> Result<f64> {
        let address = Address::from_str(address)?;
        let balance = self.provider.get_balance(address, None).await?;
        let balance_eth = format_units(balance, "ether")?.parse()?;
        Ok(balance_eth)
    }
} 