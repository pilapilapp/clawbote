use eyre::Result;
use crate::service::AccountService;

pub struct AccountController {
    service: AccountService,
}

#[allow(dead_code)]
impl AccountController {
    pub fn new() -> Result<Self> {
        let service = AccountService::new()?;
        Ok(Self { service })
    }

    pub async fn get_balance(&self, address: &str) -> Result<f64> {
        self.service.get_balance(address).await
    }

    pub async fn get_transaction_count(&self, address: &str) -> Result<u64> {
        self.service.get_transaction_count(address).await
    }

    pub async fn get_code_size(&self, address: &str) -> Result<usize> {
        self.service.get_code_size(address).await
    }
} 