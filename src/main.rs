use tracing::{info, warn, error};
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    // Initialize the tracing subscriber with an explicit EnvFilter
    let filter = EnvFilter::new("info");
    
    fmt()
        .with_env_filter(filter)
        .init();
    
    info!("Starting Clawbote... v2");
    warn!("This is a warning example v2");
    error!("This is an error example v2");
}
