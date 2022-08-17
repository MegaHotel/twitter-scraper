# twitter-scraper
## **FOR EDUCATIONAL PURPOSES ONLY** 

### Usage example:
```rust
use twitter_scraper::{run, TwitterResults};

fn main() {
    let query: String = "#Trump".to_string();
    let results: TwitterResults = run(query, None, None, None).unwrap();
    println!("{:?}", results);
}
```

## **FOR EDUCATIONAL PURPOSES ONLY** 

