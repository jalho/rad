pub struct Fork {
    /// Not the JoinHandle for the RustDedicated game server process, but for
    /// the OS thread that created that process.
    pub jh: std::thread::JoinHandle<()>,

    /// PID of the process running the RustDedicated game server.
    pub pid: u32,
}

/// Launch RustDedicated game server in an independent process.
pub fn rds_launch_fork() -> Fork {
    let (tx, rx) = std::sync::mpsc::channel::<std::result::Result<u32, std::io::Error>>();

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
        match rds_command.spawn() {
            Ok(n) => {
                rds_process = n;
            }
            Err(err) => {
                send_result(tx, std::result::Result::Err(err));
                return;
            }
        }
        pid = rds_process.id();
        send_result(tx, std::result::Result::Ok(pid));
        return;
    });

    let pid = rx.recv().unwrap().unwrap(); // TODO: Handle properly!
    return Fork { jh, pid };
}

fn send_result(
    sender: std::sync::mpsc::Sender<Result<u32, std::io::Error>>,
    result: std::result::Result<u32, std::io::Error>,
) {
    /*
      If we can't use a results channel to send results, then we can only panic.
    */
    sender.send(result).unwrap();
}
