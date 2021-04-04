use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "kyoto-server", version = env!("CARGO_PKG_VERSION"),
            author = env!("CARGO_PKG_AUTHORS"), about = "kyoto server")]
pub struct Params {
    #[structopt(name = "port", long = "--port")]
    pub port: Option<String>,

    #[structopt(name = "input buffer size", long = "--input-buffer-size")]
    pub input_buffer_size: Option<String>,
}