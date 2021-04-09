use kyoto_benchmark::data::params::Params;
use kyoto_benchmark::network::socket_io::start_benchmark;

use structopt::StructOpt;

pub fn main() -> kyoto_benchmark::Result<()> {
    let params = Params::from_args();
    start_benchmark(params)
}