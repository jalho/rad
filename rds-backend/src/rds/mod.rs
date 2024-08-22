/// Launch RustDedicated game server in an independent process.
pub fn rds_launch() {
    let th = std::thread::spawn(move || {
        /*
          PID of the forked OS thread that spawns the thread running
          RustDedicated game server.
        */
        let pid_intermediate: libc::pid_t;

        /*
          PID of the RustDedicated game server process.
        */
        let _pid_final: u32;

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
            println!("[DEBUG] - Spawning RustDedicated process...");
            rds_process = rds_command.spawn().unwrap();
            println!("[DEBUG] - Spawned RustDedicated process!");
            _pid_final = rds_process.id(); // TODO: Signal the final PID to the parent somehow?
            _ = rds_process.wait();
        }
        // case "parent"
        else {
            // here pid_intermediate is the forked child process' PID
        }
    });
    th.join().unwrap();
}
