use serde::Deserialize;

use crate::models::enums_data::messages::messages_types;

#[derive(Deserialize)]
pub struct ShortMessageRequestBody {
    whatsapp_number: u8,
    filename: Option<String>,
    media_id: Option<String>,
    media_url: Option<String>,
    message_type: messages_types::MessagesTypes,
    message: Option<String>,
    send_to: String,
    receiver_name: String,
    receiver_country_code: String,
}
