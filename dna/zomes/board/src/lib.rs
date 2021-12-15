use hdk::prelude::*;

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

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct CreateColumnInput {
    column: Column,
    board: EntryHashB64,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct CreateTaskInput {
    task: Task,
    board: EntryHashB64,
    column: EntryHashB64,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct GetBoardOutput {
    columns: Vec<Column>,
    tasks: Vec<(Task, Column)>,
}

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
    let CreateTaskInput { task, board, column } = input;
    create_entry(&task)?;
    let task_entry_hash = hash_entry(&task)?;

    create_link(EntryHash::from(board), task_entry_hash.clone(), LinkTag::new("has_task"))?;
    create_link(task_entry_hash.clone(), EntryHash::from(column), LinkTag::new("belongs_to_column"))?;

    Ok(EntryHashB64::from(task_entry_hash))
}

#[hdk_extern]
pub fn get_board(input: EntryHashB64) -> ExternResult<GetBoardOutput> {
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
    
    let tasks = get_links(board_entry_hash, Some(LinkTag::new("has_task")))?
        .into_iter()
        .map(|link| {
            let task_element = get(link.target.clone(), GetOptions::default())?
                .ok_or(WasmError::Guest(String::from("Entry not found")))?;
            let task_entry = task_element.entry().to_app_option::<Task>()?
                .ok_or(WasmError::Guest(String::from("The targeted entry is not a task")))?;
            let column_entry_hash = get_links(link.target, Some(LinkTag::new("belongs_to_column")))?
                .first()
                .ok_or(WasmError::Guest(String::from("The task is not belong to a column")))?
                .target.clone();
            let column_element = get(column_entry_hash, GetOptions::default())?
                .ok_or(WasmError::Guest(String::from("Entry not found")))?;
            let column_entry = column_element.entry().to_app_option::<Column>()?
                .ok_or(WasmError::Guest(String::from("The targeted entry is not a column")))?;
            Ok((task_entry, column_entry))
        })
        .collect::<Result<Vec<(Task, Column)>, WasmError>>()?;

    Ok(GetBoardOutput {
        columns: columns,
        tasks: tasks,
    })
}
