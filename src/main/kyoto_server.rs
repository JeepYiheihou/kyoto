use kyoto::data::Server;
use kyoto::network::Listener;

/* Main function for kyoto.
 * Start a webserver to listen to given port and accept new connections. */
pub fn main() -> kyoto::Result<()> {
    /* Enable logging diagnostics. */
    tracing_subscriber::fmt::try_init()?;

    let server = Server::new();
    let mut listener = Listener::new(server);
    listener.run()?;
    Ok(())
}