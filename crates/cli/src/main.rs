use fs_core::filesystem::{Filesystem, FsConfig, FsError};
use fs_memory::{InMemoryContentStore, InMemoryMetadataStore};
use std::io::{self, Write};

fn main() {
    let mut fs = Filesystem::new(
        InMemoryMetadataStore::new(),
        InMemoryContentStore::new(),
        FsConfig { chunk_size: 4096 },
    );

    println!("MiniFS ready. Commands:");
    println!("  put <path> <text>   # create file");
    println!("  get <path>           # read file");
    println!("  ls [path]            # list directory (default /)");
    println!("  exit");

    loop {
        print!("> ");
        io::stdout().flush().expect("flush");

        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("read line");

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line == "exit" || line == "quit" {
            break;
        }

        let mut parts = line.splitn(3, ' ');
        let cmd = parts.next().unwrap_or("");

        match cmd {
            "put" => {
                let path = parts.next();
                let text = parts.next();
                match (path, text) {
                    (Some(path), Some(text)) => match fs.create_file(path, text.as_bytes()) {
                        Ok(()) => println!("ok"),
                        Err(err) => println!("error: {}", describe_error(&err)),
                    },
                    _ => println!("usage: put <path> <text>"),
                }
            }
            "get" => {
                let path = parts.next();
                match path {
                    Some(path) => match fs.read_file(path) {
                        Ok(data) => match String::from_utf8(data) {
                            Ok(text) => println!("{}", text),
                            Err(_) => println!("(binary data)"),
                        },
                        Err(err) => println!("error: {}", describe_error(&err)),
                    },
                    None => println!("usage: get <path>"),
                }
            }
            "ls" => {
                let path = parts.next().unwrap_or("/");
                match fs.list_dir(path) {
                    Ok(entries) => {
                        if entries.is_empty() {
                            println!("(empty)");
                        } else {
                            for entry in entries {
                                println!("{}", entry.name());
                            }
                        }
                    }
                    Err(err) => println!("error: {}", describe_error(&err)),
                }
            }
            _ => println!("unknown command"),
        }
    }
}

fn describe_error(err: &FsError) -> &'static str {
    match err {
        FsError::InvalidPath => "invalid path",
        FsError::AlreadyExists => "already exists",
        FsError::NotADirectory => "parent is not a directory",
        FsError::NotAFile => "path is a directory",
        FsError::MissingNode => "node not found",
    }
}
