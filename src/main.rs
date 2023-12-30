use clap::Parser;
use error_chain::error_chain;
use reqwest::ClientBuilder;
use std::time::Duration;

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
    log::info!("Initilization");
    env_logger::init();
    let args = Args::parse();
    log::info!("Starting AOC app with parameters {:?}", args);

    let day = args.day;
    let input = download_input(args.auth, day).await?;

    let day1_result = solutions::day1::day1(&input);
    log::info!("Result day 1 = {}", day1_result);

    let day1_result_2 = solutions::day1::day1_2(&input);
    log::info!("Result day 1 part 2 = {}", day1_result_2);

    return Ok(());
}

const TIMEOUT: Duration = Duration::from_secs(2);

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
    return Result::Ok(body);
}
