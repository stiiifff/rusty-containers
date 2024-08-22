use anyhow::Error;
use clap::{crate_authors, crate_description, crate_name, crate_version, Arg};
use dotenv::dotenv;
use log::{debug, info};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    pretty_env_logger::init();

    let matches = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("api-url")
                .short("api")
                .long("api-url")
                .value_name("API_URL")
                .help("Sets the URL to the API")
                .takes_value(true),
        )
        .get_matches();

    let api_url: String = matches
        .value_of("api-url")
        .map(|s| s.to_owned())
        .or_else(|| env::var("API_URL").ok())
        // .parse()
        .expect("can't parse API_URL variable");

    debug!("Starting simple-worker ...");

    let rnd = reqwest::get(format!("{}/rand?max=30", api_url))
        .await?
        .json::<u16>()
        .await?;
    
    info!("Counting down from {}", rnd);
    for i in (0..=rnd).rev() {
        println!("{}", i);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    info! ("Worker completed.");

    Ok(())
}
