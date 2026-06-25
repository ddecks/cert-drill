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
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("cert-drill")
        .join("data")
}

/// Migrate data from old binary-relative location to the new standard location.
/// Called once at startup. Moves files if old dir exists and new dir doesn't have them yet.
pub fn migrate_data() {
    let old_data = base_dir().join("data");
    let new_data = data_dir();

    if !old_data.exists() || old_data == new_data {
        return;
    }

    let entries = match std::fs::read_dir(&old_data) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        if !entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            continue;
        }
        let exam_id = entry.file_name();
        let old_exam_dir = old_data.join(&exam_id);
        let new_exam_dir = new_data.join(&exam_id);

        if let Err(_) = std::fs::create_dir_all(&new_exam_dir) {
            continue;
        }

        let files = match std::fs::read_dir(&old_exam_dir) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let mut moved_all = true;
        for file in files.flatten() {
            let fname = file.file_name();
            let dest = new_exam_dir.join(&fname);
            if !dest.exists() {
                if std::fs::rename(file.path(), &dest).is_err() {
                    // Fall back to copy+delete for cross-device moves
                    if std::fs::copy(file.path(), &dest).is_ok() {
                        let _ = std::fs::remove_file(file.path());
                    } else {
                        moved_all = false;
                    }
                }
            }
        }

        if moved_all {
            let _ = std::fs::remove_dir(&old_exam_dir);
        }
    }

    // Remove old data dir if empty
    let _ = std::fs::remove_dir(&old_data);

    eprintln!(
        "Migrated data to {}",
        new_data.display()
    );
}
