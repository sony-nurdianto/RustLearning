use leptos::RwSignal;

#[derive(Debug, Clone)]
pub struct DatabaseEntry {
    pub key: String,
    pub value: i32,
}

#[derive(Debug, Clone)]
pub struct DatabaseEntryRwS {
    pub key: String,
    pub value: RwSignal<i32>,
}
