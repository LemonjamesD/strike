use strike::prelude::*;
use strike::secrets::TOKEN;

fn main() {
    App::new().add_plugins(DiscordAppPlugin::new(TOKEN)).run();
}
