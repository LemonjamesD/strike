use crate::prelude::*;

#[derive(Resource)]
pub struct HeartbeatInterval(usize);
#[derive(Resource)]
pub struct HeartbeatMarker;

pub(crate) struct HeartbeatPlugin;
impl Plugin for HeartbeatPlugin {
    fn build(&self, app: &mut App) {}
}
