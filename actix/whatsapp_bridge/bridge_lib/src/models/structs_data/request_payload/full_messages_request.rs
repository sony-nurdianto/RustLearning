use serde::Deserialize;

use crate::models::enums_data::messages::messages_types;

#[derive(Deserialize)]
pub struct FullMessageRequestBody {
    username: String,
    whatsapp_number: u8,
    organization_name: Option<String>,
    organization_id: Option<u32>,
    project_id: Option<u32>,
    project_name: String,
    country_code: u8,
    filename: Option<String>,
    media_id: Option<String>,
    media_url: Option<String>,
    message_type: messages_types::MessagesTypes,
    message: Option<String>,
    send_to: String,
    language: String,
    receiver_name: String,
    receiver_country_code: String,
}
