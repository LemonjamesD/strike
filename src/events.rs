use crate::types::API_URL;
use crate::GetGateway;
use crate::{prelude::*, TokioRuntime};
use bevy_ecs::prelude::*;
use serde_json::Value;
use tokio::runtime::Handle;
use tokio::task::block_in_place;

pub(crate) fn get_gateway(mut get_gateway: ResMut<GetGateway>, tokio_runtime: Res<TokioRuntime>) {
    let handle = tokio_runtime.handle();
    let _ = handle.enter();
    let task = async move || {
        let gotten_gateway = reqwest::get(format!("{}/gateway", API_URL))
            .await
            .expect("Failed to get")
            .json::<Value>()
            .await
            .expect("Failed to parse json into value")["url"]
            .clone();

        get_gateway.0 = gotten_gateway.to_string();
    };
    tokio_runtime.block_on(task());
}

pub(crate) fn read_gateway(get_gateway: Res<GetGateway>) {
    info!("{}", get_gateway.0);
}
