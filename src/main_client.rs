use crafty::args::Args;
use crafty::network::proxy::Proxy;
use crafty::network::tcp_proxy::TcpProxy;
use crafty::player::Player;
use crafty::world::World;
use crafty::world_renderer::WorldRenderer;

pub fn main() {
    let args = Args::from_args();
    let url = args.url();

    // The proxy currently holds the server,
    let proxy = TcpProxy::new(&url);
    proxy.lock().unwrap().login(args.name);

    // The client is initialized with an empty world, as it will be the responsibility of the server
    // to provide it with the chunks.
    // Currently, the client 'owns' the proxy, this is really the part that sucks for now.
    let mut client = WorldRenderer::new(proxy, World::empty(), Player::new());

    // This is blocking so we can't really do it...
    client.run();
}
