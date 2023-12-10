use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args[1].as_ref() {
        "paths" => print_paths(),
        "find" => find(&args[1..]),
        _ => todo!(),
    }
}

fn find(to_find: &[String]) {
    for path in SYSTEMD_PATHS.lines() {
        for file in WalkDir::new(path).into_iter().filter_map(|entry| {
            if let Ok(entry) = entry {
                if entry.file_type().is_file() {
                    return Some(entry);
                }
            }
            None
        }) {
            let mut found = false;
            let contents = std::fs::read_to_string(file.path());
            if let Ok(contents) = contents {
                for query in to_find {
                    if contents.contains(query) {
                        found = true
                    }
                }
                if found {
                    println!("{}", file.path().display());
                }
            }
        }
    }
}

fn print_paths() {
    for systemd_directory in SYSTEMD_PATHS.lines() {
        for path in WalkDir::new(systemd_directory)
            .into_iter()
            .filter_map(|entry| {
                if let Ok(entry) = entry {
                    return Some(entry);
                };
                None
            })
        {
            println!("{}", path.path().display());
        }
    }
}

// output of `systemd-analyze list-paths`
static SYSTEMD_PATHS: &str = r"
/etc/systemd/system.control
/run/systemd/system.control
/run/systemd/transient
/run/systemd/generator.early
/etc/systemd/system
/etc/systemd/system.attached
/run/systemd/system
/run/systemd/system.attached
/run/systemd/generator
/usr/local/lib/systemd/system
/usr/lib/systemd/system
/run/systemd/generator.late
";
