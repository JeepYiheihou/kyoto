use osaka::data::Server;
use osaka::network::Listener;

/* Main function for osaka.
 * Start a webserver to listen to given port and accept new connections. */
pub fn main() -> osaka::Result<()> {
    /* Enable logging diagnostics. */
    tracing_subscriber::fmt::try_init()?;

    let server = Server::new();
    let mut listener = Listener::new(server);
    listener.run()?;
    Ok(())
}