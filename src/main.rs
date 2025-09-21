use std::{fs, path::PathBuf};

fn main() {
    loop {
        let files: Vec<PathBuf> = rfd::FileDialog::new()
        .set_title("Choose files to move")
        .pick_files()
        .unwrap_or_default();

        if files.is_empty() {
            println!("No files selected.");
            break;
            // return;
        }

        let base_dir = files[0].parent().unwrap();

        let mut index = 0;
        let mut target_dir = base_dir.join(format!("pack_{}", index));
        while target_dir.exists() {
            index += 1;
            target_dir = base_dir.join(format!("pack_{}", index));
        }

        fs::create_dir_all(&target_dir).unwrap();

        for file in &files {
            let dest = target_dir.join(file.file_name().unwrap());
            fs::rename(file, dest).unwrap();
        }
    }
}
