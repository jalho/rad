pub fn get_age(pid: u32) -> std::time::Duration {
    todo!();
}

pub enum ProcStatus {
    TERMINATED,
    RUNNING(u32),
}
pub fn get_pid(executable_name: &str) -> std::result::Result<ProcStatus, std::io::Error> {
    let mut proc: std::process::Child;
    match std::process::Command::new("pgrep")
        .arg(executable_name)
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Ok(n) => {
            proc = n;
        }
        Err(err_io) => return std::result::Result::Err(err_io),
    }
    let stdout: std::process::ChildStdout;
    match proc.stdout.take() {
        Some(n) => {
            stdout = n;
        }
        /*
            We asked for STDOUT but the OS didn't give it so I guess we panic!
        */
        None => {
            eprintln!("Didn't get STDOUT handle!");
            todo!();
        }
    }
    let mut reader = std::io::BufReader::new(stdout);

    let pgrep_exit: std::process::ExitStatus;
    match proc.wait() {
        Ok(n) => {
            pgrep_exit = n;
        }
        Err(err_io) => return std::result::Result::Err(err_io),
    }
    if !pgrep_exit.success() {
        return std::result::Result::Ok(ProcStatus::TERMINATED);
    }

    let pid: u32;
    let mut line = String::new();
    match std::io::BufRead::read_line(&mut reader, &mut line) {
        Ok(_) => {}
        /*
            "pgrep" exited with successful status but we couldn't get its STDOUT
            so I guess we panic!
        */
        Err(err_read) => {
            eprintln!("Could not read STDOUT: {:#?}", err_read);
            todo!()
        }
    }
    let line = line.trim();
    match line.parse::<u32>() {
        Ok(n) => {
            pid = n;
        }
        /*
            "pgrep" exited with successful status, yet STDOUT wasn't parseable
            as integer so I guess we panic!
        */
        Err(err_parse) => {
            eprintln!(
                "Could not parse STDOUT as integer: '{}': {:#?}",
                line, err_parse
            );
            todo!();
        }
    }
    return std::result::Result::Ok(ProcStatus::RUNNING(pid));
}

#[derive(Debug)]
/// Errors related to forking a thing (RustDedicated game server) into an
/// independent process.
pub enum ForkError {
    TransmitPID,
    Spawn,
}
impl From<ForkError> for crate::FatalError {
    fn from(_value: ForkError) -> Self {
        return Self::ForkError;
    }
}
impl From<std::io::Error> for crate::FatalError {
    fn from(_value: std::io::Error) -> Self {
        return Self::ExtCommandError;
    }
}

pub struct Fork {
    /// Not the JoinHandle of the forked process, but of the OS thread that
    /// created that process.
    pub jh: std::thread::JoinHandle<()>,

    /// PID of the forked process.
    pub pid: u32,
}

/// Launch a thing into an independent process.
pub fn launch_fork(executable_path: &'static str) -> std::result::Result<Fork, ForkError> {
    let (tx, rx) = std::sync::mpsc::channel::<std::result::Result<u32, ForkError>>();

    let jh = std::thread::spawn(move || {
        /*
          PID of the resulting forked process.
        */
        let pid: u32;

        let mut command = std::process::Command::new(executable_path);

        /*
          We don't want to call process.wait() because that would make the
          parent process wait until the child has exited before continuing and
          likewise terminate the child upon termination of parent. What we want
          is separate lifecycles of the processes so one can continue operation
          while the other is taken down and perhaps restarted.
        */
        let process: std::process::Child;
        match command.spawn() {
            Ok(n) => {
                process = n;
            }
            Err(_) => {
                send_result(tx, std::result::Result::Err(ForkError::Spawn));
                return;
            }
        }
        pid = process.id();
        send_result(tx, std::result::Result::Ok(pid));
        return;
    });

    match rx.recv() {
        Ok(n) => match n {
            Ok(pid) => {
                return std::result::Result::Ok(Fork { jh, pid });
            }
            Err(err_fork) => {
                return std::result::Result::Err(err_fork);
            }
        },
        Err(_) => {
            return std::result::Result::Err(ForkError::TransmitPID);
        }
    }
}

fn send_result<T>(sender: std::sync::mpsc::Sender<T>, result: T) {
    match sender.send(result) {
        Ok(_) => {}
        Err(err) => {
            /*
              If we can't use a results channel to send results, then we can
              only panic.
            */
            panic!("{:#?}", err);
        }
    }
}
