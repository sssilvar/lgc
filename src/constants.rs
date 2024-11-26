pub const API_URL: &str = "https://orthographe.reverso.net/api/v1/Spelling/";
pub const HEADERS: [(&str, &str); 13] = [
    ("accept", "text/json"),
    ("accept-language", "en-GB,en;q=0.9,fr-FR;q=0.8,fr;q=0.7,en-US;q=0.6"),
    ("content-type", "application/*+json"),
    ("origin", "https://www.reverso.net"),
    ("priority", "u=1, i"),
    ("referer", "https://www.reverso.net/"),
    ("sec-ch-ua", "\"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\""),
    ("sec-ch-ua-mobile", "?0"),
    ("sec-ch-ua-platform", "\"macOS\""),
    ("sec-fetch-dest", "empty"),
    ("sec-fetch-mode", "cors"),
    ("sec-fetch-site", "same-site"),
    ("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"),

];
