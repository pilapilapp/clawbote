mod controller;
mod service;

use controller::AccountController;
use eyre::Result;

fn print_usage(program: &str) {
    println!("Usage: {} <command> [arguments...]", program);
    println!("Available commands:");
    println!("  balance <address>     - Get account balance in ETH");
    println!("  nonce <address>       - Get account transaction count (nonce)");
    println!("  code-size <address>   - Get contract code size (0 for EOA)");
    println!("  help                  - Show this help message");
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let program = &args[0];

    if args.len() < 2 {
        print_usage(program);
        return Ok(());
    }

    let controller = AccountController::new()?;
    
    match args[1].as_str() {
        "balance" => {
            if args.len() < 3 {
                println!("Usage: {} balance <address>", program);
                return Ok(());
            }
            let address = &args[2];
            let balance = controller.get_balance(address).await?;
            println!("Balance: {:.6} ETH", balance);
        }
        "nonce" => {
            if args.len() < 3 {
                println!("Usage: {} nonce <address>", program);
                return Ok(());
            }
            let address = &args[2];
            let nonce = controller.get_transaction_count(address).await?;
            println!("Nonce: {}", nonce);
        }
        "code-size" => {
            if args.len() < 3 {
                println!("Usage: {} code-size <address>", program);
                return Ok(());
            }
            let address = &args[2];
            let size = controller.get_code_size(address).await?;
            println!("Code size: {} bytes", size);
        }
        "help" => {
            print_usage(program);
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            print_usage(program);
        }
    }

    Ok(())
}
