use eyre::Result;
use crate::service::AccountService;

pub struct AccountController {
    service: AccountService,
}

impl AccountController {
    pub fn new() -> Result<Self> {
        let service = AccountService::new()?;
        Ok(Self { service })
    }

    pub async fn get_balance(&self, address: &str) -> Result<f64> {
        self.service.get_balance(address).await
    }
} 