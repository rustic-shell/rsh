use std::path::PathBuf;
use std::collections::HashMap;
use std::slice::Iter;

use rsh::utils;
use rsh::State;

pub type Builtin = fn(State) -> State;

pub fn load() -> HashMap<String, Builtin> {
    let mut h = HashMap::new();

    h.insert("cd".to_string(), cd as fn(State) -> State);
    h.insert("ls".to_string(), ls as fn(State) -> State);
    h.insert("echo".to_string(), echo as fn(State) -> State);

    h
}

fn cd(s: State) -> State {
    match s.argv.get(1) {
        Some(x) => {
            let mut new_state = s.clone();
            let mut new_path = PathBuf::from(x);

            if new_path.has_root() {
                new_state.cwd = new_path;
                return new_state;
            }


            match utils::make_absolute(new_path) {
                Ok(p) => new_state.cwd = p,
                Err(e) => {
                    println!("cd: {}", e);
                    new_state.cwd = s.cwd;
                }
            };


            new_state
        }
        None => s.clone(),
    }
}

fn ls(s: State) -> State {
    if s.argv.len() == 1 {
        list_dir(&s.cwd);
        return s;
    }

    for d in s.argv.iter() {
        let mut p = PathBuf::from(d);
        list_dir(&p);
    }

    s
}

fn list_dir(p: &PathBuf) {
    // Cheking if file so we don't do extra processing
    if p.is_file() {
        println!("FILE: {}", p.to_str().unwrap_or("WTF"));
        return;
    }

    // Unwrapping because we know it's a dir, not a file
    for entry in p.read_dir().unwrap() {
        match entry {
            Ok(e) => {
                // TODO replace this unwrap to something safer
                print!("{} ", e.file_name().into_string().unwrap());
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    print!("\n");
}

pub fn echo(s: State) -> State {
    if s.argv[1] == "-n" {
        let strings = &s.argv[2..s.argv.len()].join(" ");
        print!("{}", strings);
    } else {
        let strings = &s.argv[1..s.argv.len()].join(" ");
        println!("{}", strings);
    }

    s
}
