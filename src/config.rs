use std::sync::LazyLock;

use serde::Deserialize;

const CONFIG_STR: &str = include_str!("../config.toml");

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| toml::from_str(CONFIG_STR).unwrap());

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Config {
    pub title: String,
    pub name: String,
    pub email: String,
    pub prompt: Prompt,
    pub github: Github,
    pub linkedin: Linkedin,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Prompt {
    pub hostname: String,
    pub username: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Github {
    pub username: String,
    pub repos: Vec<String>,
    pub in_progress: Vec<InProgress>,
}

impl Github {
    pub fn url(&self) -> String {
        format!("https://github.com/{}", self.username)
    }

    pub fn short_url(&self) -> String {
        format!("github.com/{}", self.username)
    }

    pub fn api_url(&self) -> String {
        format!("https://api.github.com/users/{}/repos", self.username)
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct InProgress {
    pub name: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Linkedin {
    pub username: String,
}

impl Linkedin {
    pub fn url(&self) -> String {
        format!("https://linkedin.com/in/{}", self.username)
    }

    pub fn short_url(&self) -> String {
        format!("linkedin.com/in/{}", self.username)
    }
}
