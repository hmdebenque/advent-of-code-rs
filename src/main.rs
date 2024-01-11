use std::time::Duration;

use clap::Parser;
use error_chain::error_chain;
use reqwest::ClientBuilder;

use crate::solutions::day1::{day1, day1_2};
use crate::solutions::day2::{day2, day2_2};

mod solutions;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// Learn Rust through AOC
struct Args {
    #[clap(short, long)]
    /// AOC authentication
    auth: String,

    #[clap(short, long)]
    /// Day to solve
    day: u8,

    #[clap(short, long)]
    verbosity: Option<usize>,
}

#[tokio::main]
async fn main() -> std::result::Result<(), Error> {
    log::info!("Initialization");
    env_logger::init();
    let args = Args::parse();
    log::info!("Starting AOC app with parameters {:?}", args);

    let day = args.day;
    let input = download_input(args.auth, day).await?;

    let (part1, part2) = match args.day {
        1 => (day1(&input), day1_2(&input)),
        2 => (day2(&input), day2_2(&input)),
        other => {
            return Err(Error::from(format!("Cannot handle day {other}")));
        }
    };

    log::info!("Result for day {} part 1 = {}", args.day, part1);
    log::info!("Result for day {} part 2 = {}", args.day, part2);
    Ok(())
}

const TIMEOUT: Duration = Duration::from_secs(10);

async fn download_input(auth_cookie: String, day: u8) -> Result<String> {
    let url = format!("https://adventofcode.com/2023/day/{}/input", day);

    let client = ClientBuilder::new().timeout(TIMEOUT).build().unwrap();

    let request = client
        .get(url)
        .header("Cookie", format!("session={auth}", auth = auth_cookie));
    log::info!("Sending resquest {:?}", request);
    let res = request.send().await?;

    let body = res.text().await?;

    log::info!("response: {}", body);
    return Ok(body);
}
