fn main() {
    // TODO: Define a config struct instead of just one string
    let config: String;
    /*
        There's no inherent reason for why the config must be Arc; it's just to
        communicate the idea to the compiler... The configuration is only ever
        intended to be used (read) by one single thread at a time (the one that
        spawns the RustDedicated instance). But because it's spawned in a loop,
        it needs to be a thread-safe pointer because the compiler can't realize
        the logic that the previous thread will be completed before starting the
        next one...
    */
    let config_arc: std::sync::Arc<String>;
    match get_config() {
        Some(n) => {
            config = n;
            config_arc = std::sync::Arc::new(config);
        }
        None => {
            log(
                LogLevel::INFO,
                LogCategory::INIT,
                format_args!("Nothing to do!"),
            );
            return;
        }
    }

    // top loop: Install, update, restart RustDedicated if terminated
    loop {
        let config_arc2: std::sync::Arc<String>;
        config_arc2 = config_arc.clone();

        // TODO: Install & update SteamCMD if necessary
        // TODO: Install & update RustDedicated if necessary
        // TODO: Install & update Carbon if necessary

        let (tx, rx) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            let mut child = std::process::Command::new("./RustDedicated")
                .args([
                    "-batchmode",
                    "-logfile rds.log",
                    "+server.identity instance0",
                    "+rcon.port 28016",
                    "+rcon.web 1",
                    &format!("+rcon.password {}", config_arc2),
                ])
                .stdout(std::process::Stdio::piped())
                .spawn()
                .expect("Failed to spawn child process");
            log(
                LogLevel::INFO,
                LogCategory::INIT,
                format_args!("Launched RustDedicated with PID {}", child.id()),
            );

            let stdout = child.stdout.take().expect("Failed to take stdout");
            let reader = std::io::BufReader::new(stdout);
            for line in std::io::BufRead::lines(reader) {
                match line {
                    Ok(line) => {
                        if tx.send(line).is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading stdout: {}", e);
                        break;
                    }
                }
            }

            let _ = child.wait();
        });
        let rds_start_instant = std::time::Instant::now();
        // the grace period over which (after launch) health checks are not done
        let rds_grace_period = std::time::Duration::from_millis(3000); // TODO: Remove fixed length grace period (3 seconds is obviously not even enough in real use)

        // inner loop: Process RustDedicated STDOUT, STDERR and check its health (may hang and not terminate)
        loop {
            match rx.try_recv() {
                Ok(_line) => {
                    /*
                        TODO: Process STDOUT, STDERR of RustDedicated: Send to
                              authorized WebSocket clients, write to some log
                              file?

                              Also, collect process startup stats to some
                              persistent storage (add a database?). For example:
                              how long did it take from launching RustDedicated
                              till it logging something like "Steam Server
                              connected" or "Bradley spawned" or some other
                              indicators of readiness...
                    */
                    // println!("Child process STDOUT:\n\t{}", _line)
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    log(
                        LogLevel::INFO,
                        LogCategory::CLEANUP,
                        format_args!("Child process has finished."),
                    );
                    break;
                }
            }

            /*
                TODO: Replace fixed length grace period with dynamically
                      determined startup phase: Process STDOUT and STDERR of
                      RustDedicated to determine whether it's likely done with
                      its startup... (It logs stuff like "Steam Server
                      connected" etc.)
            */
            let elapsed = rds_start_instant.elapsed();
            if elapsed > rds_grace_period {
                let check_status = rds_running_check_health();
                if check_status.success() {
                    log(
                        LogLevel::DEBUG,
                        LogCategory::HEALTHCHECK,
                        format_args!("Healthy!"),
                    );
                } else {
                    log(
                        LogLevel::INFO,
                        LogCategory::HEALTHCHECK,
                        format_args!("Unhealthy!"),
                    );
                    rds_hung_kill();
                    log(
                        LogLevel::INFO,
                        LogCategory::HEALTHCHECK,
                        format_args!("Killed unhealthy process"),
                    );
                    break;
                }
            } else {
                log(
                    LogLevel::DEBUG,
                    LogCategory::HEALTHCHECK,
                    format_args!(
                        "Skipped during startup grace period ({:?}): {:?} elapsed since start",
                        rds_grace_period, elapsed
                    ),
                );
            }
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}

/*
   Determine whether RustDedicated is running healthily, i.e. the game is
   playable, players can join and everything is as good as it gets!

   Ideally RustDedicated would just terminate if it is unhealthy, but for
   whatever reason it sometimes seems to just hang (case "monthly force wipe
   update" where manual update & restart may be required).

   TODO: Make a more robust health check somehow... Is there some API that
         allows for checking whether a player is allowed to connect? Also,
         consider implementing it in Rust (instead of relying on system provided
         "netcat" utility for example). Current implementation just checks
         whether the default RCON port (28016) is open for TCP using netcat...
*/
fn rds_running_check_health() -> std::process::ExitStatus {
    let mut cmd: std::process::Command = std::process::Command::new("nc");
    let mut cmd_populated: &mut std::process::Command = cmd.args(["-z", "127.0.0.1", "28016"]);
    cmd_populated = cmd_populated.stdout(std::process::Stdio::piped());

    let mut cmd_process: std::process::Child;
    match cmd_populated.spawn() {
        Ok(n) => {
            cmd_process = n;
        }
        Err(_) => todo!(),
    }

    let n = cmd_process.wait().unwrap();
    return n;
}

/*
    Find process ID of RustDedicated and force it to terminate. This function
    does not do any health checks, but instead assumes that it has been
    previously determined that the process must be forced to terminate.

    This doesn't seem like a very clean solution but idk if there is a better
    way. I only care about Debian anyway so relying on pgrep and kill is fine!

    TODO: Validate commands' outputs: Don't try to proceed if something goes wrong!
          Here "pgrep" assumes RustDedicated is running and "kill" assumes
          specific STDOUT from "pgrep"...
*/
fn rds_hung_kill() {
    let mut cmd: std::process::Command = std::process::Command::new("pgrep");
    let mut cmd_populated: &mut std::process::Command = cmd.args(["RustDedicated"]);
    cmd_populated = cmd_populated.stdout(std::process::Stdio::piped());

    let mut cmd_process: std::process::Child;
    match cmd_populated.spawn() {
        Ok(n) => {
            cmd_process = n;
        }
        Err(_) => todo!(),
    }

    match cmd_process.stdout.take() {
        Some(n) => {
            let reader = std::io::BufReader::new(n);
            for line in std::io::BufRead::lines(reader) {
                match line {
                    Ok(pid) => {
                        let mut cmd: std::process::Command = std::process::Command::new("kill");
                        let mut cmd_populated: &mut std::process::Command = cmd.arg(&pid);
                        cmd_populated = cmd_populated.stdout(std::process::Stdio::piped());

                        match cmd_populated.spawn() {
                            Ok(_) => {}
                            Err(_) => todo!(),
                        }
                    }
                    Err(_) => todo!(),
                }
                break; // take 1st (and only) line, i.e. the PID
            }
        }
        None => todo!(),
    }
}

// TODO: Read config from config file or database to some struct
fn get_config() -> Option<String> {
    let env_rcon_password = "RCON_PASSWORD";
    match std::env::var(env_rcon_password) {
        Ok(n) => {
            let rcon_password: String = n;
            return Some(rcon_password);
        }
        Err(e) => {
            log(
                LogLevel::ERROR,
                LogCategory::INIT,
                format_args!("Couldn't get env var {}: {:?}", env_rcon_password, e),
            );
            return None;
        }
    }
}

#[derive(Debug)]
enum LogLevel {
    INFO,
    ERROR,
    DEBUG,
}
#[derive(Debug)]
enum LogCategory {
    INIT,
    HEALTHCHECK,
    CLEANUP,
}
fn log(level: LogLevel, category: LogCategory, message: std::fmt::Arguments) {
    println!("[{:?}] [{:?}] - {}", level, category, message)
}
