use anyhow::Result;

const BASE_URL: &str = "https://v2.jokeapi.dev/joke";

// change format to json to get metadata
pub struct Joke {
    pub text: String,
}
pub struct JokeAPIClient {
    category: String,
    format: String,
}

impl JokeAPIClient {
    pub fn default(category: String, format: String) -> Self {
        Self {
            category,
            format,
        }
    }

    pub fn get_joke(&self) -> Result<Joke, anyhow::Error> {
        let url = format!("{}/{}?format={}", BASE_URL, self.category, self.format);
        let joke = Joke {
            text: reqwest::blocking::get(&url)?.text()?,
        };
        Ok(joke)
    }
}
