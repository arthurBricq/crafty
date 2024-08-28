use std::sync::{Arc, Mutex};
use crafty::camera::Camera;
use crafty::tcp_proxy::TcpProxy;
use crafty::world::World;
use crafty::world_renderer::WorldRenderer;


pub fn main() {
    // The proxy currently holds the server,
    let proxy = TcpProxy::new("localhost:3333");

    // The client is initialized with an empty world, as it will be the responsibility of the server
    // to provide it with the chunks.
    // Currently, the client 'owns' the proxy, this is really the part that sucks for now.
    let mut client = WorldRenderer::new(proxy, World::empty(), Camera::new());
    client.login();
    
    // This is blocking so we can't really do it...
    client.run();
}

