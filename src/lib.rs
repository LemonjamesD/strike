#![feature(async_closure)]

pub mod events;
pub mod heartbeat;
pub mod prelude;
#[cfg(debug_assertions)]
pub mod secrets;
pub mod types;

use crate::events::GetGateway;
use crate::events::{get_gateway, read_gateway};
use crate::prelude::*;
use anyhow::{anyhow, Result};
use deref_derive::Deref;
use events::EventsPlugin;
use regex::Regex;
use std::fmt::Display;
use tokio::runtime::Runtime;

#[derive(Resource, Deref)]
pub struct TokioRuntime(Runtime);
impl Default for TokioRuntime {
    fn default() -> Self {
        Self(Runtime::new().unwrap())
    }
}

pub struct DiscordAppPlugin {
    token: String,
}

impl DiscordAppPlugin {
    pub fn new<T: GetDiscordToken>(token: T) -> Self {
        Self {
            token: token.get_token().unwrap(),
        }
    }
}

impl Plugin for DiscordAppPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TokioRuntime>()
            .add_plugins(EventsPlugin);
        app.update();
    }
}

pub trait GetDiscordToken {
    /// Provided Methods
    fn get_token(&self) -> Result<String> {
        match self.valid() {
            Ok(_) => Ok(self.to_token_string()?),
            Err(why) => Err(why),
        }
    }
    fn valid(&self) -> Result<()> {
        let regex = Regex::new(r"[\w-]{24}\.[\w-]{6}\.[\w-]{27}")?;
        let stringed = self.to_token_string()?;
        match regex.is_match(&stringed) {
            true => Ok(()),
            false => Err(anyhow!("Failed to validate token!")),
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
            Err(why) => Err(anyhow!(format!("Error getting `TOKEN` env var: {why:?}"))),
        }
    }
}
