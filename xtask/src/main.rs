use khonsu_tools::{
    universal::{anyhow, clap::Parser, DefaultConfig},
    Commands,
};

fn main() -> anyhow::Result<()> {
    let command = Commands::parse();
    command.execute::<Config>()
}

enum Config {}

impl khonsu_tools::Config for Config {
    type Publish = Self;
    type Universal = DefaultConfig;
}

impl khonsu_tools::publish::Config for Config {
    fn paths() -> Vec<String> {
        vec![
            String::from("crates/transmog"),
            String::from("crates/transmog-bincode"),
            String::from("crates/transmog-cbor"),
            String::from("crates/transmog-json"),
            String::from("crates/transmog-pot"),
            String::from("crates/transmog-async"),
            String::from("crates/transmog-versions"),
        ]
    }
}
