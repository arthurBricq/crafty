use clap::Parser;

const ABOUT: &str = r#"
   Welcome to the Crafty Game
   ==========================
 
It is a minecraft clone, fully coded in Rust

"#;

/// Arguments of the client
#[derive(Parser, Debug)]
#[command(version = "0.1", about = ABOUT, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = String::from("localhost"))]
    pub server: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = String::from("3333"))]
    pub port: String,
}

impl Args {
    pub fn from_args() -> Self {
        Args::parse()
    }
    
    pub fn url(&self) -> String {
        self.server.clone() + ":" + self.port.as_str()
    }
}