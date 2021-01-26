mod cli;
mod client;

use clap::Clap;

use cli::JokeCliClient;
use client::JokeAPIClient;

const BASE_URL: &str = "https://v2.jokeapi.dev/joke";

fn main() {
    let cli = JokeCliClient::parse();
    if cli.fetch {
        let client_data = JokeAPIClient::default(
            String::from(BASE_URL),
            String::from(cli.category),
            String::from(cli.format),
        );
        let joke = client_data.get_joke().unwrap();
        println!("\n{}", joke.text);
    } else {
        println!("\nuse `tamasha --fetch` to fetch a joke");
    }
}
