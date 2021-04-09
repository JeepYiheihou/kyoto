use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "kyoto-benchmark", version = env!("CARGO_PKG_VERSION"),
            author = env!("CARGO_PKG_AUTHORS"), about = "kyoto benchmark")]
pub struct Params {
    #[structopt(name = "addr", long = "--addr")]
    pub addr: Option<String>,

    #[structopt(name = "port", long = "--port")]
    pub port: Option<u16>,

    #[structopt(name = "clients", long = "--clients")]
    pub clients: Option<u16>,

    #[structopt(name = "key-space", long = "--key-space")]
    pub key_space: Option<u32>,

    #[structopt(name = "get-set-ratio", long = "--get-set-ratio")]
    pub get_set_ratio: Option<u8>,

    #[structopt(name = "time", long = "--time")]
    pub time: Option<u16>,
}