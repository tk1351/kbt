use std::path::PathBuf;

pub fn process_path(path: &str) -> Option<PathBuf> {
    if let Some(stripped) = path.strip_prefix("~") {
        let home_dir = dirs::home_dir()?;
        Some(home_dir.join(stripped.trim_start_matches("/")))
    } else {
        Some(PathBuf::from(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

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
}
