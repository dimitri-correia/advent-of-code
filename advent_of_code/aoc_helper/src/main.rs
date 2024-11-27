use reqwest::Error;

mod fetcher;
mod scores;
mod utils;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    scores::fetch_scores().await?;
    fetcher::fetch_challenges().await?;
    Ok(())
}
