pub mod prelude;
#[cfg(debug_assertions)]
pub mod secrets;

use anyhow::{Result, anyhow};
use regex::Regex;
use std::fmt::Display;

pub struct DiscordApp {
    token: String
}

impl DiscordApp {
    pub fn new<T: GetDiscordToken>(token: T) -> Self {
        Self {
            token: token.get_token().unwrap()
        }
    }
}

pub trait GetDiscordToken {
    /// Provided Methods
    fn get_token(&self) -> Result<String> {
        match self.valid() {
            Ok(_) => Ok(self.to_token_string()?),
            Err(why) => Err(why)
        }
    }
    fn valid(&self) -> Result<()> {
        let regex = Regex::new(r"[\w-]{24}\.[\w-]{6}\.[\w-]{27}")?;
        let stringed = self.to_token_string()?;
        match regex.is_match(&stringed) {
            true => Ok(()),
            false => Err(anyhow!("Failed to validate token!"))
        }
    }

    // Required Method
    fn to_token_string(&self) -> Result<String>;
}

impl<T: Display> GetDiscordToken for T {
    fn to_token_string(&self) -> Result<String> {
        Ok(format!("{self}"))
    }
    
}

/// A marker struct that implements [`GetDiscordToken`] to tell it to use env
pub struct UseEnv;

impl GetDiscordToken for UseEnv {
    fn to_token_string(&self) -> Result<String> {
        let token = std::env::var("TOKEN");
        match token {
            Ok(token) => Ok(token),
            Err(why) => Err(anyhow!(format!("Error getting `TOKEN` env var: {why:?}")))
        }
    }
}