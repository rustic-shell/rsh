mod rsh;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut argv = env::args();

    argv.next();
    
    let mut path: PathBuf;

    match argv.next() {
        Some(arg) => path = PathBuf::from(arg),
        None => path = env::current_dir().unwrap(),
    };

    let mut s = rsh::State::new(path);
    rsh::run(s)
}
