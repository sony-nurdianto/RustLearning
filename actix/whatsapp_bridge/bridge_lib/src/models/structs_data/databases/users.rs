use surrealdb::{Datetime, RecordId};

pub struct Users {
    id: Option<RecordId>,
    username: String,
    organization_id: Option<u32>,
    organization_name: Option<String>,
    client_media: String,
    country_code: u8,
    project_id: Option<u32>,
    project_name: Option<String>,
    is_clietn: bool,
    whatsapp_number: u8,
    created_at: Datetime,
    updated_at: Datetime,
}
