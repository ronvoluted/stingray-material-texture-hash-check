use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs;
use std::io;

const TEXTURE: [u8; 8] = u64::to_le_bytes(0xcd4238c6a0c69e32);

fn main() {
    let hashes = match fs::read("hashes.bin") {
        Ok(content) => {
            println!("Found hashes.bin\n");
            content
        },
        Err(_) => {
            println!("'hashes.bin' not found in current directory");
            wait_before_exit();
            return;
        }
    };

    let mut textures = HashSet::new();
    for chunk in hashes.chunks(16) {
        let (ext, name) = chunk.split_at(8);
        if ext == TEXTURE {
            textures.insert(name);
        }
    }

    let entries = fs::read_dir(".").expect("Unable to read current directory");
    let mut found_any_files = false;

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "material" {
                found_any_files = true;
                let material_name = path.file_name().unwrap().to_string_lossy();
                println!("{}:", material_name);

                let material = match fs::read(&path) {
                    Ok(content) => content,
                    Err(_) => {
                        println!("  Failed to read '{}'", material_name);
                        continue;
                    }
                };

                let mut found_any_matches = false;
                for check in material.windows(8) {
                    if textures.contains(&check) {
                        let pretty = u64::from_le_bytes(<[u8; 8]>::try_from(check).unwrap());
                        println!("  {:016x}", pretty);
                        found_any_matches = true;
                    }
                }

                if !found_any_matches {
                    println!("  No texture matches found");
                }
            }
        }
    }

    if !found_any_files {
        println!("No '.material' files found in current directory.");
    }

    wait_before_exit();
}

fn wait_before_exit() {
    println!("\nPress Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
