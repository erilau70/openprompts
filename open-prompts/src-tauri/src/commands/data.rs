use crate::error::AppResult;
use crate::models::prompt::{Prompt, PromptIndex, PromptMetadata};
use crate::services::{index_service, prompt_service, search_service};
use crate::state::AppState;

#[tauri::command]
pub fn get_index(state: tauri::State<'_, AppState>) -> AppResult<PromptIndex> {
    index_service::load_index(&state.paths)
}

#[tauri::command]
pub fn get_folders(state: tauri::State<'_, AppState>) -> AppResult<Vec<String>> {
    let index = index_service::load_index(&state.paths)?;
    Ok(index.folders)
}

#[tauri::command]
pub fn get_prompt(state: tauri::State<'_, AppState>, id: String) -> AppResult<Prompt> {
    let index = index_service::load_index(&state.paths)?;
    prompt_service::load_prompt(&state.paths, &index, &id)
}

#[tauri::command]
pub fn save_prompt(
    state: tauri::State<'_, AppState>,
    prompt: Prompt,
) -> AppResult<PromptMetadata> {
    let mut index = index_service::load_index(&state.paths)?;
    let meta = prompt_service::save_prompt(&state.paths, &mut index, prompt)?;
    index_service::save_index(&state.paths, &index)?;
    Ok(meta)
}

#[tauri::command]
pub fn delete_prompt(state: tauri::State<'_, AppState>, id: String) -> AppResult<()> {
    let mut index = index_service::load_index(&state.paths)?;
    prompt_service::delete_prompt(&state.paths, &mut index, &id)?;
    index_service::save_index(&state.paths, &index)?;
    Ok(())
}

#[tauri::command]
pub fn add_folder(state: tauri::State<'_, AppState>, name: String) -> AppResult<Vec<String>> {
    let mut index = index_service::load_index(&state.paths)?;
    prompt_service::create_folder(&state.paths, &name)?;
    index_service::add_folder(&mut index, name)?;
    index_service::save_index(&state.paths, &index)?;
    Ok(index.folders)
}

#[tauri::command]
pub fn rename_folder(
    state: tauri::State<'_, AppState>,
    old: String,
    new: String,
) -> AppResult<Vec<String>> {
    let mut index = index_service::load_index(&state.paths)?;
    prompt_service::rename_folder(&state.paths, &mut index, &old, &new)?;
    index_service::save_index(&state.paths, &index)?;
    Ok(index.folders)
}

#[tauri::command]
pub fn delete_folder(state: tauri::State<'_, AppState>, name: String) -> AppResult<Vec<String>> {
    let mut index = index_service::load_index(&state.paths)?;
    prompt_service::delete_folder(&state.paths, &mut index, &name)?;
    index_service::save_index(&state.paths, &index)?;
    Ok(index.folders)
}

#[tauri::command]
pub fn search_prompts(
    state: tauri::State<'_, AppState>,
    query: String,
) -> AppResult<Vec<PromptMetadata>> {
    let index = index_service::load_index(&state.paths)?;
    Ok(search_service::search_prompts(&index.prompts, &query))
}

#[tauri::command]
pub fn record_usage(state: tauri::State<'_, AppState>, id: String) -> AppResult<()> {
    let mut index = index_service::load_index(&state.paths)?;
    prompt_service::record_usage(&mut index, &id)?;
    index_service::save_index(&state.paths, &index)?;
    Ok(())
}
