


fn file_size(path: &str) -> i32 {
    let metadata = fs::metadata(path)?;
}