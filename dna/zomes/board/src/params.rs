use hdk::prelude::*;
use hdk::prelude::holo_hash::*;
use crate::entries::*;

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct CreateColumnInput {
    pub column: Column,
    pub board: EntryHashB64,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct CreateTaskInput {
    pub task: Task,
    pub board: EntryHashB64,
    pub column: EntryHashB64,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct DeleteTaskInput {
    pub task: EntryHashB64,
    pub column: EntryHashB64,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct UpdateTaskInput {
    pub task: EntryHashB64,
    pub column: EntryHashB64,
    pub new_task: Task,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct GetBoardColumnOutput {
    pub title: String,
    pub tasks: Vec<Task>,
}