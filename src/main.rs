use twitter_scraper::{run, TwitterResults};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = "#Trump";
    let result: TwitterResults = run(query, None, None, None)?;
    println!("{:?}", result);
    Ok(())
}
