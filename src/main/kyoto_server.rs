use kyoto::data::{ Server, Params };
use kyoto::network::listen::listen;

use structopt::StructOpt;

/* Main function for kyoto.
 * Start a webserver to listen to given port and accept new connections. */
pub fn main() -> kyoto::Result<()> {
    /* Enable logging diagnostics. */
    tracing_subscriber::fmt::try_init()?;

    let params = Params::from_args();
    let server = Server::new(params);
    listen(server)?;
    Ok(())
}