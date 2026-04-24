use std::path::PathBuf;

fn base_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

pub fn exams_dir() -> PathBuf {
    base_dir().join("exams")
}

pub fn data_dir() -> PathBuf {
    base_dir().join("data")
}
