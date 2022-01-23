use hdk::prelude::*;

#[hdk_entry(id = "board")]
pub struct Board {
    pub name: String,
    // description: Option<String>,
}

#[hdk_entry(id = "column")]
pub struct Column {
    pub title: String,
}

#[hdk_entry(id = "task")]
pub struct Task {
    pub id: u8,
    pub description: String,
}