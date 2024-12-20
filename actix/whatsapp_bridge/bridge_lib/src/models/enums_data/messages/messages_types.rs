use serde::Deserialize;

use crate::models::constants_data::common::{AUDIO, DOCUMENT, IMAGE, TEMPLATE, TEXT};

#[derive(Deserialize)]
#[serde(untagged)]
pub enum MessagesTypes {
    Text(String),
    Template(String),
    Image(String),
    Audio(String),
    Document(String),
}

impl MessagesTypes {
    fn get_value(&self) -> String {
        match self {
            Self::Text(v) => String::from(v),
            Self::Template(v) => String::from(v),
            Self::Image(v) => String::from(v),
            Self::Audio(v) => String::from(v),
            Self::Document(v) => String::from(v),
        }
    }

    pub fn text() -> String {
        let m: Self = Self::Text(String::from(TEXT));
        m.get_value()
    }

    pub fn template() -> String {
        let m: Self = Self::Template(String::from(TEMPLATE));
        m.get_value()
    }

    pub fn image() -> String {
        let m: Self = Self::Image(String::from(IMAGE));
        m.get_value()
    }

    pub fn audio() -> String {
        let m: Self = Self::Audio(String::from(AUDIO));
        m.get_value()
    }

    pub fn document() -> String {
        let m: Self = Self::Document(String::from(DOCUMENT));
        m.get_value()
    }
}
