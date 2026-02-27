use crate::error::{map_err, AppResult};
use crate::models::prompt::{Prompt, PromptIndex, PromptMetadata};
use crate::services::storage::{atomic_write, StoragePaths};
use std::fs;
use std::path::PathBuf;

/// Get the filesystem path for a prompt's .md file
pub fn get_prompt_path(paths: &StoragePaths, folder: &str, filename: &str) -> PathBuf {
    if folder.is_empty() {
        paths.prompts_dir.join(filename)
    } else {
        paths.prompts_dir.join(folder).join(filename)
    }
}

/// Sanitize a string into a valid Windows filename
pub fn sanitize_filename(input: &str) -> String {
    let reserved = [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6",
        "COM7", "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7",
        "LPT8", "LPT9",
    ];

    let sanitized: String = input
        .chars()
        .filter(|c| !matches!(c, '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*'))
        .filter(|c| !c.is_control())
        .collect();

    let sanitized = sanitized
        .trim_end_matches(|c: char| c == '.' || c == ' ')
        .to_string();

    if sanitized.is_empty() {
        return "untitled".to_string();
    }

    let stem = sanitized.split('.').next().unwrap_or("");
    let sanitized = if reserved.iter().any(|r| r.eq_ignore_ascii_case(stem)) {
        format!("_{}", sanitized)
    } else {
        sanitized
    };

    // Truncate to reasonable length
    if sanitized.len() > 200 {
        sanitized[..200].to_string()
    } else {
        sanitized
    }
}

/// Generate a unique filename in the target folder, appending -N if needed
pub fn ensure_unique_filename(paths: &StoragePaths, folder: &str, base: &str) -> String {
    let sanitized = sanitize_filename(base);
    let md_name = format!("{}.md", sanitized);
    let target_path = get_prompt_path(paths, folder, &md_name);

    if !target_path.exists() {
        return md_name;
    }

    for i in 1..1000 {
        let candidate = format!("{}-{}.md", sanitized, i);
        let candidate_path = get_prompt_path(paths, folder, &candidate);
        if !candidate_path.exists() {
            return candidate;
        }
    }

    // Fallback: use UUID
    format!("{}-{}.md", sanitized, uuid::Uuid::new_v4())
}

/// Load a prompt's content from disk, combining metadata from index with .md file
pub fn load_prompt(paths: &StoragePaths, index: &PromptIndex, id: &str) -> AppResult<Prompt> {
    let meta = index
        .prompts
        .iter()
        .find(|p| p.id == id)
        .ok_or_else(|| format!("Prompt '{}' not found", id))?;

    let file_path = get_prompt_path(paths, &meta.folder, &meta.filename);
    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Could not read prompt file {:?}: {}", file_path, e))?;

    Ok(Prompt {
        meta: meta.clone(),
        content,
    })
}

/// Save a prompt: create or update. Write .md first, then update index.
/// Returns the saved metadata.
pub fn save_prompt(
    paths: &StoragePaths,
    index: &mut PromptIndex,
    prompt: Prompt,
) -> AppResult<PromptMetadata> {
    let now = chrono::Utc::now().to_rfc3339();

    // Check if this is an update (id exists in index)
    if let Some(existing) = index.prompts.iter_mut().find(|p| p.id == prompt.meta.id) {
        // Update existing prompt
        let file_path = get_prompt_path(paths, &existing.folder, &existing.filename);

        // If folder changed, need to move the file
        if existing.folder != prompt.meta.folder {
            let new_filename =
                ensure_unique_filename(paths, &prompt.meta.folder, &prompt.meta.name);
            let new_dir = if prompt.meta.folder.is_empty() {
                paths.prompts_dir.clone()
            } else {
                paths.prompts_dir.join(&prompt.meta.folder)
            };
            fs::create_dir_all(&new_dir).map_err(map_err)?;
            let new_path = new_dir.join(&new_filename);

            // Write new file first
            atomic_write(&new_path, prompt.content.as_bytes())?;
            // Delete old file
            let _ = fs::remove_file(&file_path);

            existing.filename = new_filename;
        } else {
            // Same folder, just overwrite content
            atomic_write(&file_path, prompt.content.as_bytes())?;
        }

        existing.name = prompt.meta.name.clone();
        existing.folder = prompt.meta.folder.clone();
        existing.description = prompt.meta.description.clone();
        existing.icon = prompt.meta.icon.clone();
        existing.color = prompt.meta.color.clone();
        existing.updated = now;

        return Ok(existing.clone());
    }

    // Create new prompt
    let id = if prompt.meta.id.is_empty() {
        uuid::Uuid::new_v4().to_string()
    } else {
        prompt.meta.id.clone()
    };

    let folder = &prompt.meta.folder;
    let filename = ensure_unique_filename(paths, folder, &prompt.meta.name);

    // Ensure folder directory exists
    let dir = if folder.is_empty() {
        paths.prompts_dir.clone()
    } else {
        paths.prompts_dir.join(folder)
    };
    fs::create_dir_all(&dir).map_err(map_err)?;

    // Write .md file FIRST (crash safety: orphan file is harmless)
    let file_path = dir.join(&filename);
    atomic_write(&file_path, prompt.content.as_bytes())?;

    // Ensure folder exists in index
    if !folder.is_empty() && !index.folders.contains(folder) {
        index.folders.push(folder.clone());
    }

    let meta = PromptMetadata {
        id,
        name: prompt.meta.name,
        folder: prompt.meta.folder,
        description: prompt.meta.description,
        filename,
        use_count: 0,
        last_used: None,
        created: now.clone(),
        updated: now,
        icon: prompt.meta.icon,
        color: prompt.meta.color,
    };

    index.prompts.push(meta.clone());
    Ok(meta)
}

/// Delete a prompt. Update index FIRST, then delete .md file.
pub fn delete_prompt(paths: &StoragePaths, index: &mut PromptIndex, id: &str) -> AppResult<()> {
    let meta = index
        .prompts
        .iter()
        .find(|p| p.id == id)
        .ok_or_else(|| format!("Prompt '{}' not found", id))?
        .clone();

    // Remove from index FIRST (crash safety: dangling file is harmless)
    index.prompts.retain(|p| p.id != id);

    // Delete file
    let file_path = get_prompt_path(paths, &meta.folder, &meta.filename);
    let _ = fs::remove_file(&file_path); // Ignore error if already gone

    Ok(())
}

pub fn create_folder(paths: &StoragePaths, name: &str) -> AppResult<()> {
    let folder_path = if name.is_empty() {
        paths.prompts_dir.clone()
    } else {
        paths.prompts_dir.join(name)
    };
    fs::create_dir_all(folder_path).map_err(map_err)
}

pub fn rename_folder(
    paths: &StoragePaths,
    index: &mut PromptIndex,
    old_name: &str,
    new_name: &str,
) -> AppResult<()> {
    if old_name == new_name {
        return Ok(());
    }

    if new_name.is_empty() {
        return Err("New folder name cannot be empty".to_string());
    }

    let old_path = paths.prompts_dir.join(old_name);
    let new_path = paths.prompts_dir.join(new_name);

    if old_path.exists() {
        if new_path.exists() {
            return Err(format!("Folder '{}' already exists", new_name));
        }
        fs::rename(&old_path, &new_path).map_err(map_err)?;
    } else {
        // If source folder doesn't exist (e.g. empty folder tracked in metadata), create target.
        fs::create_dir_all(&new_path).map_err(map_err)?;
    }

    for prompt in &mut index.prompts {
        if prompt.folder == old_name {
            prompt.folder = new_name.to_string();
            prompt.updated = chrono::Utc::now().to_rfc3339();
        }
    }

    if let Some(pos) = index.folders.iter().position(|folder| folder == old_name) {
        index.folders[pos] = new_name.to_string();
    }

    if let Some(ref mut meta) = index.folder_meta {
        if let Some(folder_meta) = meta.remove(old_name) {
            let mut updated = folder_meta;
            updated.name = new_name.to_string();
            meta.insert(new_name.to_string(), updated);
        }
    }

    Ok(())
}

pub fn delete_folder(paths: &StoragePaths, index: &mut PromptIndex, name: &str) -> AppResult<()> {
    if name.is_empty() {
        return Err("Cannot delete the root folder".to_string());
    }

    for prompt in index.prompts.iter_mut().filter(|p| p.folder == name) {
        let old_path = get_prompt_path(paths, &prompt.folder, &prompt.filename);
        let target_filename = if get_prompt_path(paths, "", &prompt.filename).exists() {
            ensure_unique_filename(paths, "", &prompt.name)
        } else {
            prompt.filename.clone()
        };

        let new_path = get_prompt_path(paths, "", &target_filename);
        if old_path.exists() {
            fs::rename(&old_path, &new_path).map_err(map_err)?;
        }

        prompt.folder = String::new();
        prompt.filename = target_filename;
        prompt.updated = chrono::Utc::now().to_rfc3339();
    }

    let folder_path = paths.prompts_dir.join(name);
    if folder_path.exists() {
        let is_empty = fs::read_dir(&folder_path)
            .map_err(map_err)?
            .next()
            .is_none();

        if is_empty {
            fs::remove_dir(&folder_path).map_err(map_err)?;
        }
    }

    index.folders.retain(|folder| folder != name);
    if let Some(ref mut meta) = index.folder_meta {
        meta.remove(name);
    }

    Ok(())
}

/// Increment use_count and set lastUsed timestamp
pub fn record_usage(index: &mut PromptIndex, id: &str) -> AppResult<()> {
    let meta = index
        .prompts
        .iter_mut()
        .find(|p| p.id == id)
        .ok_or_else(|| format!("Prompt '{}' not found", id))?;

    meta.use_count += 1;
    meta.last_used = Some(chrono::Utc::now().to_rfc3339());
    Ok(())
}
