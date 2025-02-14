mod fetcher;
mod scrapper;
mod storage;

fn main() {
    let res = fetcher::fetch::fetch("https://www.rust-lang.org/");
    match res {
        Ok(res) => println!("{}", res),
        Err(e) => println!("Error: {}", e),
    }
}
