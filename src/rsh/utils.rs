use std::path::PathBuf;
use std::fs;
use std::io::Error;

// TODO rewrite this using platform specific kernel calls so it respects
// symlinks
pub fn make_absolute(p: PathBuf) -> Result<PathBuf, Error> {
    fs::canonicalize(p)
}
