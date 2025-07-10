use std::fs;
use std::io;
use std::path::Path;
use std::time::UNIX_EPOCH;

#[tauri::command]
pub fn get_files_and_details_as_vector(path: String) -> Vec<(String, u64, String, String)> {
    match get_files_and_details(path) {
        Ok(data) => return data,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Vec::new();
        }
    }
}

fn get_files_and_details<P: AsRef<Path>>(
    path: P,
) -> io::Result<Vec<(String, u64, String, String)>> {
    let mut result = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() || path.is_dir() {
            let metadata = fs::metadata(&path)?;
            let file_name = match path.file_name() {
                Some(name) => name.to_string_lossy().into_owned(),
                None => continue,
            };
            let size = metadata.len();
            let creation_date = metadata
                .created()?
                .duration_since(UNIX_EPOCH)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
                .as_secs();
            let extension = path
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();
            result.push((file_name, size, creation_date.to_string(), extension));
        }
    }

    Ok(result)
}
