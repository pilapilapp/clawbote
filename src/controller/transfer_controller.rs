use eyre::Result;

pub struct TransferController {
    // We'll add service later
}

#[allow(dead_code)]
impl TransferController {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn send_eth(&self, _from: &str, _to: &str, _amount: f64) -> Result<String> {
        // Temporary implementation
        Ok("0x".to_string())
    }

    pub async fn check_tx_status(&self, _tx_hash: &str) -> Result<String> {
        // Temporary implementation
        Ok("pending".to_string())
    }
} 