use osaka::network;

/* Main function for osaka.
 * Start a webserver to listen to given port and accept new connections. */
pub fn main() -> osaka::Result<()> {
    /* Enable logging diagnostics. */
    tracing_subscriber::fmt::try_init()?;

    let port: u32 = 9736;
    let mut server = network::server::Server::new(port);
    server.run()?;
    Ok(())
}