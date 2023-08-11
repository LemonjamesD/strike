use bevy_log::LogPlugin;
use strike::prelude::*;
use strike::secrets::TOKEN;

fn main() {
    App::new()
        .add_plugins(LogPlugin {
            level: Level::DEBUG,
            filter: String::new(),
        })
        .add_plugins(DiscordAppPlugin::new(TOKEN))
        .run();
}
