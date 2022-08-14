use twitter_scraper::{run, TwitterResults};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth_token = "Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA";
    let guest_token = "1558597490429100035";
    let result: TwitterResults = run(auth_token, guest_token)?;
    println!("{:?}", result);
    Ok(())
}
