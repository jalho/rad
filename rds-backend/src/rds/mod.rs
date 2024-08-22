#[derive(Debug)]
pub enum ForkError {
    A(std::io::Error),
    B(std::sync::mpsc::RecvTimeoutError),
}

/// Launch RustDedicated game server in an independent process.
pub fn rds_launch() -> std::result::Result<libc::pid_t, ForkError> {
    let (tx, rx) = std::sync::mpsc::channel::<Result<libc::pid_t, ForkError>>();

    std::thread::spawn(move || {
        let mut rds_command = std::process::Command::new("./RustDedicated");
        let mut rds_thread: std::process::Child;

        match rds_command.spawn() {
            Ok(n) => {
                rds_thread = n;
            }
            Err(err) => {
                _ = tx.send(std::result::Result::Err(ForkError::A(err)));
                return;
            }
        }

        let pid: libc::pid_t;
        unsafe {
            /*
              Duplicate current process into an independent process.
            */
            pid = libc::fork();
        }

        // case "child": Run the spawned RustDedicated game server process to termination
        if pid == 0 {
            _ = rds_thread.wait();
            return;
        }
        // case "parent": Kill the excess duplicate thread.
        else {
            match rds_thread.kill() {
                Ok(_) => {
                    _ = tx.send(std::result::Result::Ok(pid));
                    return;
                }
                Err(err) => {
                    _ = tx.send(std::result::Result::Err(ForkError::A(err)));
                    return;
                }
            }
        }
    });

    let timeout = std::time::Duration::from_millis(10);
    match rx.recv_timeout(timeout) {
        Ok(result) => {
            return result;
        }
        Err(err) => {
            return std::result::Result::Err(ForkError::B(err));
        }
    }
}
