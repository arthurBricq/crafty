use clap::{Parser, ValueEnum};
use std::env;
use tracing::debug;

#[allow(dead_code)]
#[derive(Debug, Clone, ValueEnum)]
pub enum WorldInitializer {
    RANDOM,
    FLAT,
    DISK,
}

impl WorldInitializer {
    pub fn from_args() -> Self {
        let args: Vec<String> = env::args().collect();
        debug!("args = {args:?}");
        if args.contains(&"--random".to_string()) {
            Self::RANDOM
        } else if args.contains(&"--flat".to_string()) {
            Self::FLAT
        } else if args.contains(&"--disk".to_string()) {
            Self::DISK
        } else {
            Self::RANDOM
        }
    }
}

const ABOUT: &str = r#"

  |==========================|   
  |Welcome to the Crafty Game|  
  |==========================| 
 
It is a minecraft clone, fully coded in Rust.

"#;

/// Arguments of the client
#[derive(Parser, Debug)]
#[command(version = "0.1", about = ABOUT, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = String::from("localhost"), help = "IP address of the server")]
    pub server: String,

    /// Number of times to greet
    #[arg(short, long, help = "IP port of the server", default_value_t = String::from("3333"))]
    pub port: String,

    #[arg(short, long, help = "Name of the player", default_value_t = String::new())]
    pub name: String,

    #[arg(
        value_enum,
        short,
        long,
        help = "How to initialize the world",
        default_value = "random"
    )]
    pub init: WorldInitializer,
}

impl Args {
    pub fn from_args() -> Self {
        Args::parse()
    }

    pub fn url(&self) -> String {
        self.server.clone() + ":" + self.port.as_str()
    }
}
