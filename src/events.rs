use crate::types::OpCode;
use crate::types::API_URL;
use crate::{prelude::*, TokioRuntime};
use anyhow::Result;
use bevy_ecs::prelude::*;
use bevy_ecs::reflect::ReflectResourceFns;
use deref_derive::Deref;
use futures::StreamExt;
use serde_json::Value;
use std::net::TcpStream;
use std::rc::Rc;
use std::sync::Arc;
use tungstenite::connect;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::Message;
use tungstenite::WebSocket;
use url::Url;

#[derive(Resource, Deref, Default)]
pub(crate) struct GetGateway(String);
#[derive(Resource, Deref)]
pub(crate) struct WebsocketStream(WebSocket<MaybeTlsStream<TcpStream>>);

pub(crate) fn get_gateway(mut get_gateway: ResMut<GetGateway>, tokio_runtime: Res<TokioRuntime>) {
    let handle = tokio_runtime.handle();
    let _ = handle.enter();
    tokio_runtime.block_on(get_gateway_url(&mut get_gateway));
}

async fn get_gateway_url(get_gateway: &mut GetGateway) {
    let mut gotten_gateway = reqwest::get(format!("{}/gateway", API_URL))
        .await
        .expect("Failed to get")
        .json::<Value>()
        .await
        .expect("Failed to parse json into value")["url"]
        .clone()
        .to_string();
    gotten_gateway.remove(0);
    gotten_gateway.remove(gotten_gateway.len() - 1);

    get_gateway.0 = gotten_gateway;
}

pub(crate) fn read_gateway(
    mut commands: Commands,
    get_gateway: Res<GetGateway>,
    tokio_runtime: Res<TokioRuntime>,
) {
    info!("{}", get_gateway.0);
    // Insert it into the resource
    let socket = connect(&get_gateway.0).unwrap();
    commands.insert_resource(WebsocketStream(socket.0));
    info!("Connected Succesfully to discord");
}

#[derive(Event, Debug)]
pub enum OpEvents {
    Op10(OpCode),
}

pub(crate) fn read_stream(
    mut op_events: EventWriter<OpEvents>,
    mut stream: ResMut<WebsocketStream>,
) {
    let read = stream.0.read();
    if read.is_err() {
        return;
    }
    match read.unwrap() {
        Message::Text(text) => {
            let valued: Value =
                serde_json::from_str(&text).expect("Failed to unwrap Text event into JSON");
            let op_code: Result<OpCode, _> = serde_json::from_str(&text);
            if let Ok(op_code) = op_code {
                let value = match op_code.op {
                    10 => OpEvents::Op10(op_code),
                    _ => return,
                };
                op_events.send(value);
            }
        }
        _ => {}
    }
}

pub(crate) struct EventsPlugin;
impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GetGateway>()
            .add_event::<OpEvents>()
            .add_systems(Startup, (get_gateway, read_gateway).chain())
            .add_systems(Update, read_stream);
    }
}
