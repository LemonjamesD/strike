pub mod op_codes;
pub use op_codes::*;
use serde::de::Deserialize;
use serde::ser::Serialize;

pub const API_URL: &'static str = "https://discord.com/api/v10";

pub trait SimpleSerialize {
    fn serialize(&self) -> String;
}

impl<T: Serialize> SimpleSerialize for T {
    fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub trait SimpleDeserialize {
    fn deserialize<'a, T: Deserialize<'a>>(&self) -> T;
}

impl<T: ToString> SimpleDeserialize for T {
    fn deserialize<'a, R: Deserialize<'a>>(&self) -> R {
        serde_json::from_str::<R>(&self.to_string()).unwrap()
    }
}
