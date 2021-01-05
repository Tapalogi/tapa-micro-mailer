use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait IJsonSerializable<T = Self>
where
    Self: DeserializeOwned + Serialize + Clone + Send + Sized,
{
    fn from_slice(json_slice: &[u8]) -> Option<Self> {
        serde_json::from_slice::<Self>(json_slice).ok()
    }

    fn from_json(json_string: &str) -> Option<Self> {
        serde_json::from_str::<Self>(json_string).ok()
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_pretty_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
