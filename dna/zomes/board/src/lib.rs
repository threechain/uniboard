use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

mod entries;
mod params;

use entries::*;
use params::*;

entry_defs![
    Board::entry_def(),
    Column::entry_def(),
    Task::entry_def()
];

#[hdk_extern]
pub fn create_board(board: Board) -> ExternResult<EntryHashB64> {
    create_entry(&board)?;
    let hash = hash_entry(&board)?;

    Ok(EntryHashB64::from(hash))
}

#[hdk_extern]
pub fn create_column(input: CreateColumnInput) -> ExternResult<EntryHashB64> {
    let CreateColumnInput { column, board } = input;
    create_entry(&column)?;
    let column_entry_hash = hash_entry(&column)?;

    let board_entry_hash = EntryHash::from(board);
    create_link(board_entry_hash, column_entry_hash.clone(), LinkTag::new("has_column"))?;
    
    Ok(EntryHashB64::from(column_entry_hash))
}

// TODO content should be unique for each new item
#[hdk_extern]
pub fn create_task(input: CreateTaskInput) -> ExternResult<EntryHashB64> {
    let CreateTaskInput { task, board: _, column } = input;
    create_entry(&task)?;
    let task_entry_hash = hash_entry(&task)?;

    create_link(EntryHash::from(column), task_entry_hash.clone(), LinkTag::new("has_task"))?;
    // create_link(task_entry_hash.clone(), EntryHash::from(column), LinkTag::new("belongs_to_column"))?;

    Ok(EntryHashB64::from(task_entry_hash))
}

#[hdk_extern]
pub fn get_board(input: EntryHashB64) -> ExternResult<Vec<GetBoardColumnOutput>> {
    let board_entry_hash = EntryHash::from(input);
    let columns = get_links(board_entry_hash.clone(), Some(LinkTag::new("has_column")))?
        .into_iter()
        .map(|link| {
            let element = get(link.target, GetOptions::default())?
                .ok_or(WasmError::Guest(String::from("Entry not found")))?;
            let entry = element.entry().to_app_option::<Column>()?
                .ok_or(WasmError::Guest(String::from("The targeted entry is not a column")))?;
            Ok(entry)
        })
        .collect::<Result<Vec<Column>, WasmError>>()?;
    
    let tasks = columns.into_iter()
        .map(|col| {
            let col_entry_hash = hash_entry(&col)?;
            let tasks = get_links(col_entry_hash, Some(LinkTag::new("has_task")))?
                .into_iter()
                .map(|link| {
                    let element = get(link.target, GetOptions::default())?
                        .ok_or(WasmError::Guest(String::from("Entry not found")))?;
                    let entry = element.entry().to_app_option::<Task>()?
                        .ok_or(WasmError::Guest(String::from("The targeted entry is not a task")))?;
                    Ok(entry)
                })
                .collect::<Result<Vec<Task>, WasmError>>()?;
            
            Ok(GetBoardColumnOutput { title: col.title, tasks: tasks })
        })
        .collect::<Result<Vec<GetBoardColumnOutput>, WasmError>>()?;
    

    Ok(tasks)
}
