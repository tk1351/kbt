use std::{
    fs::{self, DirEntry},
    io,
    path::PathBuf,
};

pub fn process_path(path: &str) -> Option<PathBuf> {
    if let Some(stripped) = path.strip_prefix("~") {
        let home_dir = dirs::home_dir()?;
        Some(home_dir.join(stripped.trim_start_matches("/")))
    } else {
        Some(PathBuf::from(path))
    }
}

pub fn get_dir_entries(dir: &PathBuf) -> Result<Vec<DirEntry>, io::Error> {
    if !dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("{} does not exist", dir.display()),
        ));
    }

    if !dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} is not a directory", dir.display()),
        ));
    }

    let mut entries: Vec<DirEntry> = fs::read_dir(dir)?.filter_map(|f| f.ok()).collect();
    entries.sort_by_key(|e| e.file_name());
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn process_tilde_path() {
        env::set_var("HOME", "/home/testuser");

        let result = process_path("~/documents");
        assert_eq!(result, Some(PathBuf::from("/home/testuser/documents")));
    }

    #[test]
    fn process_regular_path() {
        let result = process_path("test");
        assert_eq!(result, Some(PathBuf::from("test")));
    }

    #[test]
    fn process_absolute_path() {
        let result = process_path("/usr/local/bin");
        assert_eq!(result, Some(PathBuf::from("/usr/local/bin")));
    }

    #[test]
    fn get_dir_entries_success() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().to_path_buf();

        File::create(temp_path.join("file1.txt")).unwrap();
        File::create(temp_path.join("file2.txt")).unwrap();
        fs::create_dir(temp_path.join("subdir")).unwrap();

        let result = get_dir_entries(&temp_path);
        assert!(result.is_ok());

        let entries = result.unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].file_name(), "file1.txt");
        assert_eq!(entries[1].file_name(), "file2.txt");
        assert_eq!(entries[2].file_name(), "subdir");
    }

    #[test]
    fn get_dir_entries_non_existent() {
        let non_existent_path = PathBuf::from("/path/that/does/not/exist");
        let result = get_dir_entries(&non_existent_path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn get_dir_entries_not_a_directory() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("file.txt");
        File::create(&file_path).unwrap();

        let result = get_dir_entries(&file_path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidInput);
    }
}
