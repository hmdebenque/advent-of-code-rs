use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use clap::Parser;
use reqwest::ClientBuilder;

mod aoc_2023;
mod aoc_2024;
mod aoc_2025;

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// Learn Rust through AOC
struct Args {
    #[clap(short, long)]
    /// AOC authentication
    auth: String,

    #[clap(short, long)]
    /// Year to solve
    year: u16,

    #[clap(short, long)]
    /// Day to solve
    day: u8,

    #[clap(short, long)]
    verbosity: Option<usize>,
}

#[tokio::main]
async fn main() -> Result<()> {
    log::info!("Initialization");
    env_logger::init();
    let args = Args::parse();
    log::warn!("Starting AOC app with parameters {:?}", args);

    let day = args.day;
    let input = download_input(args.auth, args.year, day).await?;

    if args.year == 2023 {
        let (part1, part2) = match args.day {
            1 => (aoc_2023::day1::day1(&input), aoc_2023::day1::day1_2(&input)),
            2 => (aoc_2023::day2::day2(&input), aoc_2023::day2::day2_2(&input)),
            3 => (aoc_2023::day3::day3(&input), aoc_2023::day3::day3_2(&input)),
            other => {
                return Err(anyhow!("Cannot handle day {other}"));
            }
        };
        log::info!("Result for day {} part 1 = {}", args.day, part1);
        log::info!("Result for day {} part 2 = {}", args.day, part2);
    } else if args.year == 2024 {
        let (part1, part2) = match args.day {
            1 => (aoc_2024::day1::day1(&input), aoc_2024::day1::day1_2(&input)),
            2 => (aoc_2024::day2::day2(&input), aoc_2024::day2::day2_2(&input)),
            3 => (aoc_2024::day3::day3(&input), aoc_2024::day3::day3(&input)),
            4 => (aoc_2024::day4::day4(&input), aoc_2024::day4::day4(&input)),
            5 => (aoc_2024::day5::day5(&input), aoc_2024::day5::day5_2(&input)),
            6 => (
                with_timer("day 6 part 1", &|| aoc_2024::day6::day6(&input)),
                with_timer("day 6 part 2", &|| aoc_2024::day6::day6_2(&input)),
            ),
            7 => (
                with_timer("day 7 part 1", &|| aoc_2024::day7::day7(&input)),
                with_timer("day 7 part 2", &|| aoc_2024::day7::day7_2(&input)),
            ),
            8 => (
                with_timer("day 8 part 1", &|| aoc_2024::day8::day8(&input)),
                with_timer("day 8 part 2", &|| aoc_2024::day8::day8_2(&input)),
            ),
            9 => (
                with_timer("day 9 part 1", &|| aoc_2024::day9::day9(&input)),
                with_timer("day 9 part 2", &|| aoc_2024::day9::day9_2(&input)),
            ),
            10 => (
                with_timer("day 10 part 1", &|| aoc_2024::day10::day10(&input)),
                with_timer("day 10 part 2", &|| aoc_2024::day10::day10_2(&input)),
            ),
            12 => (
                aoc_2024::day12::day12(&input),
                aoc_2024::day12::day12_2(&input),
            ),
            13 => (
                aoc_2024::day13::day13(&input),
                aoc_2024::day13::day13_2(&input),
            ),
            other => {
                return Err(anyhow!("Cannot handle day {other}"));
            }
        };
        log::info!("Result 2024 for day {} part 1 = {}", args.day, part1);
        log::info!("Result 2024 for day {} part 2 = {}", args.day, part2);
    } else if args.year == 2025 {
        let (part1, part2) = match args.day {
            1 => (aoc_2025::day1::day1(&input), aoc_2025::day1::day1_2(&input)),
            2 => (aoc_2025::day2::day2(&input), aoc_2025::day2::day2_2(&input)),
            3 => (aoc_2025::day3::day3(&input), aoc_2025::day3::day3_2(&input)),
            4 => (aoc_2025::day4::day4(&input), aoc_2025::day4::day4_2(&input)),
            5 => (aoc_2025::day5::day5(&input), aoc_2025::day5::day5_2(&input)),
            6 => (aoc_2025::day6::day6(&input), aoc_2025::day6::day6_2(&input)),
            7 => (aoc_2025::day7::day7(&input), aoc_2025::day7::day7_2(&input)),
            8 => (
                aoc_2025::day8::day8(&input, 1000),
                aoc_2025::day8::day8_2(&input),
            ),
            // 11 => (aoc_2025::day11::day11(&input), aoc_2025::day11::day11_2(&input)),
            other => {
                return Err(anyhow!("Cannot handle day {other}"));
            }
        };
        log::info!("Result 2025 for day {} part 1 = {}", args.day, part1);
        log::info!("Result 2025 for day {} part 2 = {}", args.day, part2);
    }

    Ok(())
}

const TIMEOUT: Duration = Duration::from_secs(10);

async fn download_input(auth_cookie: String, year: u16, day: u8) -> Result<String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let client = ClientBuilder::new().timeout(TIMEOUT).build().unwrap();

    let request = client
        .get(url)
        .header("Cookie", format!("session={auth}", auth = auth_cookie));
    log::info!("Sending request {:?}", request);
    let res = request.send().await?;

    let body = res.text().await?;

    log::info!("response: {}", body);
    Ok(body)
}

fn with_timer<T>(name: &str, function: &dyn Fn() -> T) -> T {
    let now = Instant::now();
    let result = function();
    println!(
        "Execution time of {}, {}ms",
        name,
        now.elapsed().as_millis()
    );
    result
}
