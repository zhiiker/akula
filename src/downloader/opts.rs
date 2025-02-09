use crate::downloader::sentry_address::SentryAddress;
use anyhow::anyhow;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "ddl", about = "test sentry client")]
pub struct Opts {
    #[structopt(
        long = "sentry.api.addr",
        help = "Sentry GRPC service URL as 'http://host:port'",
        default_value = "http://localhost:8000"
    )]
    pub sentry_api_addr: SentryAddress,
    #[structopt(
        long = "chain",
        help = "Name of the testnet to join",
        default_value = "mainnet"
    )]
    pub chain_name: String,
}

impl Opts {
    pub fn new(chain_names: &[&str]) -> anyhow::Result<Self> {
        let instance: Opts = Opts::from_args();

        if !chain_names.contains(&instance.chain_name.as_str()) {
            return Err(anyhow!("unknown chain '{}'", instance.chain_name));
        }

        Ok(instance)
    }
}
