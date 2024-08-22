pub struct Fork {
    pub jh: std::thread::JoinHandle<()>,
    pub pid: u32,
}

/// Launch RustDedicated game server in an independent process.
pub fn rds_launch_fork() -> Fork {
    let (tx, rx) = std::sync::mpsc::channel::<u32>();

    /*
      We're calling libc::fork here which duplicates the current thread, so we
      need a clean thread, therefore std::thread::spawn.
    */
    let jh = std::thread::spawn(move || {
        /*
          PID of the forked process that creates yet another process that in
          turn runs the RustDedicated game server.
        */
        let pid_intermediate: libc::pid_t;

        /*
          PID of the RustDedicated game server process.
        */
        let pid_final: u32;

        let mut rds_command = std::process::Command::new("./RustDedicated");
        let rds_process: std::process::Child;
        rds_process = rds_command.spawn().unwrap();
        pid_final = rds_process.id();
        _ = tx.send(pid_final).unwrap(); // TODO: Handle properly!

        unsafe {
            /*
              Duplicate current process into an independent process.
            */
            pid_intermediate = libc::fork();
        }

        // case "child": Nothing to do!
        if pid_intermediate == 0 {
        }
        // case "parent": Nothing to do!
        else {
        }
    });

    let pid_final = rx
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap(); // TODO: Handle properly!
    return Fork { jh, pid: pid_final };
}
