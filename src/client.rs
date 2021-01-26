use anyhow::Result;

// change format to json to get metadata
pub struct Joke {
    pub text: String,
}
pub struct JokeAPIClient {
    url: String,
    category: String,
    format: String,
}

impl JokeAPIClient {
    pub fn default(url: String, category: String, format: String) -> Self {
        Self {
            url,
            category,
            format,
        }
    }

    pub fn get_joke(&self) -> Result<Joke, anyhow::Error> {
        let url = format!("{}/{}?format={}", self.url, self.category, self.format);
        let joke = Joke {
            text: reqwest::blocking::get(&url)?.text()?,
        };
        Ok(joke)
    }
}
