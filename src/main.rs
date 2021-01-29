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
        match client_data.get_joke() {
            Ok(joke) => println!("{}", joke.text),
            Err(err) => println!("{:?}", err)
        };
    } else {
        println!("use `tamasha --fetch` to fetch a joke");
    }
}
