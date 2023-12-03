use crate::TZ;
use chrono::Datelike;
use chrono::Utc;
use clap::Parser;
use colored::*;
use reqwest::Client;
use scraper::Html;
use scraper::Selector;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
#[derive(Parser)]
pub struct Args {
    #[clap(long, env = "year")]
    year: Option<i32>,
    #[clap(long, env = "day")]
    day: Option<u32>,
    #[clap(long, env = "session")]
    session: String,
    level: i32,
    answer: String,
}
pub async fn run(args: Args) -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let attempts = File::open("attempts.txt").unwrap();
    for line in BufReader::new(attempts).lines() {
        if line? == args.answer {
            println!("{} has been submitted before", args.answer.magenta());
            return Ok(());
        }
    }
    let date = Utc::now().with_timezone(&*TZ);
    let year = args.year.unwrap_or(date.year());
    let day = args.day.unwrap_or(date.day());
    let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);
    println!("{}", url);
    let client = Client::new();
    let response = client
        .post(&url)
        .header("Cookie", format!("session={}", args.session))
        .form(&[
            ("level", args.level.to_string()),
            ("answer", args.answer.to_string()),
        ])
        .send()
        .await?;
    let text = response.text().await?;
    let document = Html::parse_document(&text);
    let selector = Selector::parse("main article p").unwrap();
    let content = document
        .select(&selector)
        .map(|element| element.text().collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join("\n");
    let content_colored = if content.contains("That's not the right answer.") {
        content.red()
    } else if content.contains("You gave an answer too recently;") {
        content.yellow()
    } else {
        content.green()
    };
    println!("{}", content_colored);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("attempts.txt")
        .unwrap();
    writeln!(file, "{}", args.answer).unwrap();
    Ok(())
}
