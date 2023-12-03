use crate::TZ;
use chrono::DateTime;
use chrono::Datelike;
use chrono::Duration;
use chrono::FixedOffset;
use chrono::NaiveTime;
use chrono::Utc;
use clap::Parser;
use reqwest::Client;
use std::error::Error;
use std::fs::File;
use std::io::Write;
#[derive(Parser)]
pub struct Args {
    #[clap(long, env = "year")]
    year: Option<i32>,
    #[clap(long, env = "day")]
    day: Option<u32>,
    #[clap(long, env = "session")]
    session: String,
    #[clap(long, env, default_value = "00:10:00")]
    sleep: NaiveTime,
}
pub async fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let sleep = args.sleep.signed_duration_since(NaiveTime::default());
    let date = countdown(sleep).await;
    let year = args.year.unwrap_or(date.year());
    let day = args.day.unwrap_or(date.day());
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    println!("{}", url);
    let client = Client::new();
    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", args.session))
        .send()
        .await?;
    let mut file = File::create("input.txt")?;
    file.write_all(&response.bytes().await?)?;
    Ok(())
}
async fn countdown(sleep: Duration) -> DateTime<FixedOffset> {
    fn duration_until_midnight(date: DateTime<FixedOffset>) -> Duration {
        let next_midnight = (date + Duration::days(1))
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(*TZ)
            .unwrap();
        next_midnight - date
    }
    let mut date = Utc::now().with_timezone(&*TZ);
    loop {
        let duration = duration_until_midnight(date);
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;
        let seconds = duration.num_seconds() % 60;
        println!("{:02}:{:02}:{:02}", hours, minutes, seconds);
        if duration > sleep || duration < Duration::zero() {
            return date;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        date = Utc::now().with_timezone(&*TZ);
    }
}
