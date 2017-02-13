use std::path::PathBuf;

use rsh::State;
use rsh::utils;

pub fn cd(s: &mut State) -> i32 {
    let mut new_path: PathBuf;

    match s.argv.get(1) {
        Some(x) => new_path = PathBuf::from(x),
        None => {
            new_path = PathBuf::from(s.variables
                .get("HOME")
                .unwrap_or(&"".to_string()))
        }
    };

    if !new_path.has_root() {
        match utils::make_absolute(new_path) {
            Ok(p) => new_path = p,
            Err(e) => {
                println!("cd: {}", e);
                return 1;
            }
        };
    }

    if !new_path.exists() {
        println!("cd: no such file or directory");
        return 1;
    }


    s.cwd = new_path;

    0
}
