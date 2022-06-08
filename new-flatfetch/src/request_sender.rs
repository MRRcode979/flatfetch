extern crate clap;
extern crate tokio;
extern crate reqwest;
extern crate owo_colors;

use owo_colors::OwoColorize;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn fetch_url(url: &str) -> Result<()> {    
    let bar = ProgressBar::new(5);
    bar.set_style(ProgressStyle::default_bar()
    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
    .progress_chars("█ "));
    let res = Client::new()
    .get(url)
    .send()
    .await
    .or(Err(format!("Failed to GET from '{}'", &url)))?;
    bar.inc(1);
    let file_name = url.split('/').last().unwrap();
    bar.inc(1);
    let mut file = std::fs::File::create(file_name)?;
    bar.inc(1);
    let mut content = std::io::Cursor::new(res.bytes().await?);
    bar.inc(1);
    std::io::copy(&mut content, &mut file)?;
    bar.inc(1);
    bar.finish();
    println!("{}", "All done!".blue());
    Ok(())
}
