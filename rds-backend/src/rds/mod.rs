/// Launch RustDedicated game server in an independent process.
pub fn rds_launch_fork() -> std::thread::JoinHandle<()> {
    /*
      We're calling libc::fork here which duplicates the current thread, so we
      need a clean thread, therefore std::thread::spawn.
    */
    let th = std::thread::spawn(move || {
        /*
          PID of the forked process that creates yet another process that in
          turn runs the RustDedicated game server.
        */
        let pid_intermediate: libc::pid_t;

        /*
          PID of the RustDedicated game server process.
        */
        let pid_final: u32;

        unsafe {
            /*
              Duplicate current process into an independent process.
            */
            pid_intermediate = libc::fork();
        }

        // case "child": Run RustDedicated to termination
        if pid_intermediate == 0 {
            let mut rds_command = std::process::Command::new("./RustDedicated");
            let mut rds_process: std::process::Child;
            rds_process = rds_command.spawn().unwrap();
            pid_final = rds_process.id();
            /*
              TODO: Signal the final PID to the parent somehow? Now logged from
                    the forked child process...
            */
            println!("[INFO] - Forked RustDedicated into PID {}", pid_final);
            _ = rds_process.wait();
        }
        // case "parent": Nothing to do!
        else {
            /*
              Would be nice if we could have the value pid_final here, so we
              could return that and e.g. make log about it from within the main
              process.
            */
        }
    });
    return th;
}
