mod asset;

use clap::Parser;

fn main() {
    let cli = asset::Asset::parse();

    if let Err(e) = cli.execute() {
        println!("{:?}", e);
    };
}
