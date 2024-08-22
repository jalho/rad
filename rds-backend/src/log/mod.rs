// TODO: Make into trait?
pub fn log_error_with_source<T: std::error::Error + 'static>(err: T) {
    eprintln!("[ERROR] - {}", err);
    match err.source() {
        Some(cause) => {
            for (_, e) in std::iter::successors(Some(cause), |e| e.source()).enumerate() {
                eprintln!("        ^-- {}", e); // align with the "[ERROR] -" prefix
            }
        }
        None => {}
    }
}
