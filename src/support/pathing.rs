use std::path::Path;

pub fn collapse_posix(path_like: &str) -> String {
    let mut parts: Vec<String> = Vec::new();
    for part in path_like.replace('\\', "/").split('/') {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." {
            parts.pop();
            continue;
        }
        parts.push(part.to_string());
    }
    parts.join("/")
}

pub fn relative_posix(path: &Path, root: &Path) -> String {
    match path.strip_prefix(root) {
        Ok(value) => normalize_path_string(value.to_string_lossy().as_ref()),
        Err(_) => normalize_path_string(path.to_string_lossy().as_ref()),
    }
}

pub fn normalize_path_string(value: &str) -> String {
    value.replace('\\', "/")
}
