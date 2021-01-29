mod cli;
mod client;

use clap::Clap;

use cli::JokeCliClient;
use client::JokeAPIClient;

fn main() {
    let cli = JokeCliClient::parse();
    if cli.fetch {
        let client_data = JokeAPIClient::default(
            cli.category,
            cli.format,
        );
        match client_data.get_joke() {
            Ok(joke) => println!("{}", joke.text),
            Err(err) => println!("{:?}", err)
        };
    } else {
        println!("use `tamasha --fetch` to fetch a joke");
    }
}
