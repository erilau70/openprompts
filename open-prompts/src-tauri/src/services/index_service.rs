use crate::error::{map_err, AppResult};
use crate::models::prompt::PromptIndex;
use crate::services::storage::{atomic_write, StoragePaths};
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub fn load_index(paths: &StoragePaths) -> AppResult<PromptIndex> {
    let mut index = if !paths.index_path.exists() {
        PromptIndex::default()
    } else {
        let data = fs::read_to_string(&paths.index_path).map_err(map_err)?;
        match serde_json::from_str::<PromptIndex>(&data) {
            Ok(index) => index,
            Err(e) => {
                // Rename corrupt file, create fresh
                let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
                let corrupt_path = paths
                    .index_path
                    .with_extension(format!("corrupt.{}", timestamp));
                let _ = fs::rename(&paths.index_path, &corrupt_path);
                eprintln!("Corrupt index.json renamed to {:?}: {}", corrupt_path, e);
                PromptIndex::default()
            }
        }
    };

    let changed = sync_index_with_filesystem(paths, &mut index)?;
    if changed || !paths.index_path.exists() {
        save_index(paths, &index)?;
    }

    Ok(index)
}

pub fn save_index(paths: &StoragePaths, index: &PromptIndex) -> AppResult<()> {
    let json = serde_json::to_string_pretty(index).map_err(map_err)?;
    atomic_write(&paths.index_path, json.as_bytes())
}

fn sync_index_with_filesystem(paths: &StoragePaths, index: &mut PromptIndex) -> AppResult<bool> {
    let mut changed = false;
    let existing_len = index.prompts.len();

    let mut existing_by_key: HashMap<(String, String), crate::models::prompt::PromptMetadata> =
        HashMap::new();
    for meta in index.prompts.drain(..) {
        existing_by_key.insert((meta.folder.clone(), meta.filename.clone()), meta);
    }

    let discovered_files = scan_prompt_files(&paths.prompts_dir)?;
    let mut rebuilt_prompts = Vec::with_capacity(discovered_files.len());

    for (folder, filename, file_path) in discovered_files {
        let key = (folder.clone(), filename.clone());
        let file_timestamp = file_timestamp_rfc3339(&file_path);

        if let Some(mut existing) = existing_by_key.remove(&key) {
            if let Some(ts) = file_timestamp {
                if existing.updated != ts {
                    existing.updated = ts;
                    changed = true;
                }
            }
            rebuilt_prompts.push(existing);
            continue;
        }

        changed = true;
        let now = chrono::Utc::now().to_rfc3339();
        let ts = file_timestamp.unwrap_or_else(|| now.clone());

        rebuilt_prompts.push(crate::models::prompt::PromptMetadata {
            id: uuid::Uuid::new_v4().to_string(),
            name: filename_to_title(&filename),
            folder,
            description: String::new(),
            filename,
            use_count: 0,
            last_used: None,
            created: ts.clone(),
            updated: ts,
            icon: None,
            color: None,
        });
    }

    if !existing_by_key.is_empty() {
        changed = true;
    }

    index.prompts = rebuilt_prompts;
    if index.prompts.len() != existing_len {
        changed = true;
    }

    let discovered_folders: HashSet<String> = index
        .prompts
        .iter()
        .filter_map(|p| {
            if p.folder.is_empty() {
                None
            } else {
                Some(p.folder.clone())
            }
        })
        .collect();

    for folder in &discovered_folders {
        if !index.folders.contains(folder) {
            index.folders.push(folder.clone());
            changed = true;
        }
    }

    // Dedupe while preserving order
    let mut seen = HashSet::new();
    let before = index.folders.len();
    index.folders.retain(|folder| seen.insert(folder.clone()));
    if index.folders.len() != before {
        changed = true;
    }

    Ok(changed)
}

fn scan_prompt_files(prompts_dir: &Path) -> AppResult<Vec<(String, String, PathBuf)>> {
    let mut files = Vec::new();

    if !prompts_dir.exists() {
        return Ok(files);
    }

    scan_prompt_files_recursive(prompts_dir, prompts_dir, &mut files)?;
    files.sort_by(|a, b| {
        let left = format!("{}/{}", a.0, a.1);
        let right = format!("{}/{}", b.0, b.1);
        left.cmp(&right)
    });

    Ok(files)
}

fn scan_prompt_files_recursive(
    root: &Path,
    current: &Path,
    acc: &mut Vec<(String, String, PathBuf)>,
) -> AppResult<()> {
    for entry in fs::read_dir(current).map_err(map_err)? {
        let entry = entry.map_err(map_err)?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(map_err)?;

        if file_type.is_dir() {
            scan_prompt_files_recursive(root, &path, acc)?;
            continue;
        }

        if !file_type.is_file() {
            continue;
        }

        let is_markdown = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("md"))
            .unwrap_or(false);

        if !is_markdown {
            continue;
        }

        let filename = match path.file_name().and_then(|name| name.to_str()) {
            Some(name) => name.to_string(),
            None => continue,
        };

        let folder = match path.parent().and_then(|parent| parent.strip_prefix(root).ok()) {
            Some(rel) if !rel.as_os_str().is_empty() => normalize_folder(rel),
            _ => String::new(),
        };

        acc.push((folder, filename, path));
    }

    Ok(())
}

fn normalize_folder(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<String>>()
        .join("/")
}

fn file_timestamp_rfc3339(path: &Path) -> Option<String> {
    let modified = fs::metadata(path).ok()?.modified().ok()?;
    let dt: DateTime<Utc> = DateTime::<Utc>::from(system_time_or_now(modified));
    Some(dt.to_rfc3339())
}

fn system_time_or_now(time: SystemTime) -> SystemTime {
    if time.duration_since(SystemTime::UNIX_EPOCH).is_ok() {
        time
    } else {
        SystemTime::now()
    }
}

fn filename_to_title(filename: &str) -> String {
    let stem = Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("untitled");
    let cleaned = stem.replace(['_', '-'], " ");
    if cleaned.trim().is_empty() {
        "Untitled".to_string()
    } else {
        cleaned
    }
}

pub fn add_folder(index: &mut PromptIndex, name: String) -> AppResult<()> {
    if index.folders.contains(&name) {
        return Err(format!("Folder '{}' already exists", name));
    }
    index.folders.push(name);
    Ok(())
}

pub fn rename_folder(index: &mut PromptIndex, old_name: &str, new_name: &str) -> AppResult<()> {
    if index.folders.contains(&new_name.to_string()) {
        return Err(format!("Folder '{}' already exists", new_name));
    }

    let pos = index
        .folders
        .iter()
        .position(|f| f == old_name)
        .ok_or_else(|| format!("Folder '{}' not found", old_name))?;
    index.folders[pos] = new_name.to_string();

    // Update all prompt folder references
    for prompt in &mut index.prompts {
        if prompt.folder == old_name {
            prompt.folder = new_name.to_string();
        }
    }

    // Update folder_meta if present
    if let Some(ref mut meta) = index.folder_meta {
        if let Some(folder_meta) = meta.remove(old_name) {
            let mut updated = folder_meta;
            updated.name = new_name.to_string();
            meta.insert(new_name.to_string(), updated);
        }
    }

    Ok(())
}

pub fn delete_folder(index: &mut PromptIndex, name: &str) -> AppResult<()> {
    index.folders.retain(|f| f != name);

    // Move prompts to uncategorized
    for prompt in &mut index.prompts {
        if prompt.folder == name {
            prompt.folder = String::new();
        }
    }

    // Remove from folder_meta if present
    if let Some(ref mut meta) = index.folder_meta {
        meta.remove(name);
    }

    Ok(())
}
