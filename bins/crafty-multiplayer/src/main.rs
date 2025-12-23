use graphics::player::world_renderer::WorldRenderer;
use model::args::Args;
use model::game::player::Player;
use model::world::world::World;
use network::proxy::Proxy;
use network::tcp_proxy::TcpProxy;

pub fn main() {
    tracing_subscriber::fmt::init();
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
    client.run::<graphics_wgpu::runtime::WgpuRenderer>();
}
