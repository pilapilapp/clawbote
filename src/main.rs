mod controller;
mod service;

use controller::AccountController;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let controller = AccountController::new()?;
    let balance = controller.get_balance("0xfdDae01FD29B7b7AD169F9f2557071AA40A3A134").await?;
    println!("Balance: {:.2} ETH", balance);
    Ok(())
}
