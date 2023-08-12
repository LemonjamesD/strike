use crate::prelude::*;
use crate::types::OpCode;
use crate::types::API_URL;
use anyhow::Result;
use bevy_ecs::prelude::*;
use bevy_ecs::reflect::ReflectResourceFns;
use bevy_tasks::IoTaskPool;
use bevy_tasks::TaskPool;
use deref_derive::Deref;
use serde_json::Value;
use std::net::TcpStream;
use std::rc::Rc;
use std::sync::Arc;
use std::thread::spawn;
use tungstenite::connect;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::Message;
use tungstenite::WebSocket;
use url::Url;

#[derive(Resource, Deref, Default)]
pub(crate) struct GetGateway(String);
#[derive(Resource, Deref)]
pub(crate) struct WebsocketStream(WebSocket<MaybeTlsStream<TcpStream>>);

pub(crate) fn get_gateway(mut get_gateway: ResMut<GetGateway>) {
    let mut gotten_gateway = reqwest::blocking::get(format!("{}/gateway", API_URL))
        .expect("Failed to get")
        .json::<Value>()
        .expect("Failed to parse json into value")["url"]
        .clone()
        .to_string();
    gotten_gateway.remove(0);
    gotten_gateway.remove(gotten_gateway.len() - 1);

    get_gateway.0 = gotten_gateway;
}

pub(crate) fn connect_to_gateway(mut commands: Commands, get_gateway: Res<GetGateway>) {
    info!("Gateway URL: {}", get_gateway.0);
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
    let pool = IoTaskPool::init(|| TaskPool::new());
    let results = pool.scope(move |s| {
        s.spawn(async move {
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
        });
    });
}

pub(crate) struct EventsPlugin;
impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GetGateway>()
            .add_event::<OpEvents>()
            .add_systems(
                Startup,
                (get_gateway, connect_to_gateway, apply_deferred, read_stream).chain(),
            );
    }
}
