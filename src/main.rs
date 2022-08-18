use home;
use std::path::Path;
use std::fs;

// Move the most recently download file in ~/Downloads
// into the current directory.
fn main() {
    let homedir = match home::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("unable to figure out home directory");
            std::process::exit(1);
        }
    };

    let downloads_dir = Path::new(&homedir).join("Downloads");
    if !downloads_dir.exists() {
        eprintln!("downloads directory doesn't exist: {}", downloads_dir.display());
        std::process::exit(1);
    }

    let mut dir_entries = Vec::new();
    match fs::read_dir(&downloads_dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(ent) => {
                        dir_entries.push(ent.path());
                    },
                    Err(err) => {
                        eprintln!("error accessing: {} {}", downloads_dir.display(), err);
                        std::process::exit(1);
                    }
                };
            }
        },
        Err(err) => {
            eprintln!("unable access the downloads directory: {:?}", err);
            std::process::exit(1);
        }
    }

    if dir_entries.is_empty() {
        eprintln!("nothing in downloads");
        std::process::exit(1);
    }

    dir_entries.sort_by_key(|entry| entry.metadata().unwrap().modified().unwrap());
    dir_entries.reverse();


    let most_recent = &dir_entries[0];

    let rename_as = Path::new(most_recent.file_name().unwrap());
    if rename_as.exists() {
        eprintln!("./{} already exists", rename_as.display());
        std::process::exit(1);
    }

    fs::rename(most_recent, rename_as).unwrap();
    println!("moved {} to .", most_recent.display());
}
