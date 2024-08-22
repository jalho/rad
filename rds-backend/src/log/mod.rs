pub trait SourceLoggable: std::error::Error {
    fn log(&self) {
        eprintln!("[ERROR] - {}", self);
        match self.source() {
            Some(cause) => {
                for (_, e) in std::iter::successors(Some(cause), |e| e.source()).enumerate() {
                    eprintln!("        ^-- {}", e); // align with the "[ERROR] -" prefix
                }
            }
            None => {}
        }
    }
}
