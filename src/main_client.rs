use clap::Parser;
use crafty::camera::Camera;
use crafty::network::tcp_proxy::TcpProxy;
use crafty::world::World;
use crafty::world_renderer::WorldRenderer;

const ABOUT: &str = r#"
Client of the crafty game

Allows to connect to the server and play with your friends.
"#;

/// Arguments of the client
#[derive(Parser, Debug)]
#[command(version = "0.1", about = ABOUT, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = String::from("localhost"))]
    server: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = String::from("3333"))]
    port: String,
}

pub fn main() {
    let args = Args::parse();

    let server = args.server;
    let port = args.port;
    let url  = server + port.as_str();

    // The proxy currently holds the server,
    let proxy = TcpProxy::new(&url);

    // The client is initialized with an empty world, as it will be the responsibility of the server
    // to provide it with the chunks.
    // Currently, the client 'owns' the proxy, this is really the part that sucks for now.
    let mut client = WorldRenderer::new(proxy, World::empty(), Camera::new());
    client.login();

    // This is blocking so we can't really do it...
    client.run();
}

