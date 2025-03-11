use std::path::PathBuf;

use wax::Glob;

pub fn scan_csproj(path: &str) -> Vec<PathBuf> {
    let mut paths = vec![];
    let glob = Glob::new("**/*.csproj").unwrap();
    for entry in glob.walk(path) {
        let entry = entry.unwrap();
        let path = entry.path().to_path_buf();
        paths.push(path);
    }
    paths
}
