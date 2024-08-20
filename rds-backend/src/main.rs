fn main() {
    // TODO: Define a config struct
    let config: String;
    match get_config() {
        Some(n) => {
            config = n;
        }
        None => {
            log(LogLevel::INFO, format_args!("Nothing to do!"));
            return;
        }
    }

    log(LogLevel::INFO, format_args!("Launching RustDedicated..."));
    exec_observable(
        "./RustDedicated",
        [
            "-batchmode",
            "-logfile rds.log",
            "+server.identity instance0",
            "+rcon.port 28016",
            "+rcon.web 1",
            &format!("+rcon.password {}", config),
        ],
    );
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
                format_args!("Couldn't get env var {}: {:?}", env_rcon_password, e),
            );
            return None;
        }
    }
}

/*
    TODO:
    - Document.
    - Add writers for STDOUT, STDERR: The writers should be capable of writing
      to log file(s) and of sending to a number of authorized WebSockets.
    - Add support for collecting stats: E.g. how long did it take for
      RustDedicated to start accepting TCP at RCON port or for BradleyAPC to
      spawn or for Steam server to connect etc. The collector's should be able
      to read any given STDOUT and STDERR, parse them and then pass what was
      read to some writer (that may e.g. insert to a database).
*/
fn exec_observable<I, S>(executable_path: S, args: I) -> std::process::ExitStatus
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let mut cmd: std::process::Command = std::process::Command::new(executable_path);
    let mut cmd_populated: &mut std::process::Command = cmd.args(args);
    cmd_populated = cmd_populated.stdout(std::process::Stdio::piped());

    let mut cmd_process: std::process::Child;
    match cmd_populated.spawn() {
        Ok(n) => {
            cmd_process = n;
        }
        Err(_) => todo!(),
    }

    // TODO: Handle STDERR too
    match cmd_process.stdout.take() {
        Some(n) => {
            let reader = std::io::BufReader::new(n);
            for line in std::io::BufRead::lines(reader) {
                match line {
                    Ok(n) => {
                        // TODO: Pass read line to given writers and exec stats collector
                        println!("{}", n);
                    }
                    Err(_) => todo!(),
                }
            }
        }
        None => todo!(),
    }

    match cmd_process.wait() {
        Ok(n) => {
            return n;
        }
        Err(_) => todo!(),
    }
}

enum LogLevel {
    INFO,
    ERROR,
}
fn log(level: LogLevel, message: std::fmt::Arguments) {
    match level {
        LogLevel::INFO => {
            println!("[INFO] - {}", message)
        }
        LogLevel::ERROR => {
            println!("[ERROR] - {}", message)
        }
    }
}
