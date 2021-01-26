use clap::Clap;

/// Tamasha - A cli to fetch a joke from JokeAPI
#[derive(Clap)]
#[clap(version = "0.1.0", about = "Fetch me a joke!")]
pub struct JokeCliClient {
    #[clap(long)]
    pub fetch: bool,
    #[clap(short, long, default_value = "any")]
    pub category: String,
    #[clap(short, long, default_value = "txt")]
    pub format: String,
}
