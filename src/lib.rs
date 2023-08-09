#![feature(async_closure)]

pub mod heartbeat;
pub mod prelude;
#[cfg(debug_assertions)]
pub mod secrets;
pub mod types;

use crate::heartbeat::{get_gateway, read_gateway};
use anyhow::{anyhow, Result};
use bevy_ecs::prelude::*;
use deref_derive::Deref;
use regex::Regex;
use std::fmt::Display;
use tokio::runtime::Runtime;

#[derive(Resource, Default)]
pub(crate) struct GetGateway(String);
#[derive(Resource, Deref)]
pub struct TokioRuntime(Runtime);
impl Default for TokioRuntime {
    fn default() -> Self {
        Self(Runtime::new().unwrap())
    }
}

#[repr(usize)]
pub enum Stage {
    PreStartup = 0,
    Startup = 1,
    PostStartup = 2,
    PreUpdate = 3,
    Update = 4,
    PostUpdate = 5,
}

pub struct DiscordApp {
    token: String,
    world: World,
    schedules: [Schedule; 6],
}

impl DiscordApp {
    /// Create the struct and add the token
    pub fn new<T: GetDiscordToken>(token: T) -> Self {
        Self {
            token: token.get_token().unwrap(),
            world: World::new(),
            // [PreStartup, Startup, PostStartup, PreUpdate, Update, PostUpdate]
            schedules: [
                Schedule::default(),
                Schedule::default(),
                Schedule::default(),
                Schedule::default(),
                Schedule::default(),
                Schedule::default(),
            ],
        }
    }
    /// Add system(s)
    pub fn add_systems<M>(
        &mut self,
        stage: Stage,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        self.schedules[stage as usize].add_systems(systems);
        self
    }
    /// Run the App
    pub fn run(&mut self) {
        // Setup some internal stuff
        self.setup();
        // Run the startup systems
        self.schedules[0].run(&mut self.world);
        self.schedules[1].run(&mut self.world);
        self.schedules[2].run(&mut self.world);
        // Run the update systems forever
        loop {
            self.schedules[3].run(&mut self.world);
            self.schedules[4].run(&mut self.world);
            self.schedules[5].run(&mut self.world);
        }
    }
    /// Create the heartbeat loop and all that
    pub(crate) fn setup(&mut self) {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::TRACE)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
        self.world.init_resource::<GetGateway>();
        self.world.init_resource::<TokioRuntime>();

        self.add_systems(Stage::PreStartup, (get_gateway, read_gateway).chain());
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
