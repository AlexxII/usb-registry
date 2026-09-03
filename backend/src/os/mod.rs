use std::path::PathBuf;

pub fn database_path() -> PathBuf {
    PathBuf::from("./app.db")
}
