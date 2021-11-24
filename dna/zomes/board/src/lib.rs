use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

#[hdk_entry(id = "board")]
pub struct Board {
    name: String,
    // description: Option<String>,
}

#[hdk_entry(id = "column")]
pub struct Column {
    title: String,
}

#[hdk_entry(id = "task")]
pub struct Task {
    description: String,
}

entry_defs![
    Board::entry_def(),
    Column::entry_def(),
    Task::entry_def()
];

pub fn create_board(board: Board) -> ExternResult<EntryHashB64> {
    create_entry(&board)?;
    let hash = hash_entry(&board)?;

    Ok(EntryHashB64::from(hash))
}

pub fn create_column(column: Column, board: EntryHashB64) -> ExternResult<EntryHashB64> {
    create_entry(&column)?;
    let column_entry_hash = hash_entry(&column)?;

    let board_entry_hash = EntryHash::from(board);
    create_link(board_entry_hash, column_entry_hash.clone(), LinkTag::new("has_member"))?;
    
    Ok(EntryHashB64::from(column_entry_hash))
}

pub fn create_task(task: Task, board: EntryHashB64, column: EntryHashB64) -> ExternResult<EntryHashB64> {
    create_entry(&task)?;
    let task_entry_hash = hash_entry(&task)?;

    create_link(EntryHash::from(board), task_entry_hash.clone(), LinkTag::new("has_task"))?;
    create_link(task_entry_hash.clone(), EntryHash::from(column), LinkTag::new("belongs_to_column"))?;

    Ok(EntryHashB64::from(task_entry_hash))
}
