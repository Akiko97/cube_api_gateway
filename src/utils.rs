use serde::{Serialize, de::DeserializeOwned};

pub fn read_json<T: DeserializeOwned>(json: &str) -> serde_json::error::Result<T> {
    serde_json::from_str::<T>(json)
}

pub fn to_json<T: Serialize>(value: &T) -> serde_json::error::Result<String> {
    serde_json::to_string::<T>(value)
}
