pub struct Fork {
    /// Not the JoinHandle for the RustDedicated game server process, but for
    /// the OS thread that created that process.
    pub jh: std::thread::JoinHandle<()>,

    /// PID of the process running the RustDedicated game server.
    pub pid: u32,
}

/// Launch RustDedicated game server in an independent process.
pub fn rds_launch_fork() -> Fork {
    let (tx, rx) = std::sync::mpsc::channel::<u32>();

    let jh = std::thread::spawn(move || {
        /*
          PID of the RustDedicated game server process.
        */
        let pid: u32;

        let mut rds_command = std::process::Command::new("./RustDedicated");

        /*
          We don't want to call rds_process.wait() because that would make the
          parent process wait until the child has exited before continuing and
          likewise terminate the child upon termination of parent. What we want
          is separate lifecycles of the processes so one can continue operation
          while the other is taken down and perhaps restarted.
        */
        let rds_process: std::process::Child;
        rds_process = rds_command.spawn().unwrap();
        pid = rds_process.id();

        match tx.send(pid) {
            Ok(_) => {
                // Nothing to do!
            }
            Err(err) => {
                handle_result_channel_err(err);
            }
        }

        return;
    });

    let pid = rx.recv().unwrap(); // TODO: Handle properly!
    return Fork { jh, pid };
}

/// If we can't use a results channel to send results, then we can only panic.
fn handle_result_channel_err<T>(_err: std::sync::mpsc::SendError<T>) {
    panic!();
}
