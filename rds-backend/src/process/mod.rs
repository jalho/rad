pub fn get_age(pid: u32) -> std::time::Duration {
    todo!();
}

pub enum ProcStatus {
    Terminated,
    Running(u32),
}
pub fn get_pid(executable_name: &str) -> std::result::Result<ProcStatus, ProcessError> {
    let mut proc: std::process::Child;
    let executable_path = std::path::PathBuf::from("pgrep");
    match std::process::Command::new(&executable_path)
        .arg(executable_name)
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Ok(n) => {
            proc = n;
        }
        Err(err_io) => {
            return std::result::Result::Err(ProcessError::CannotSpawn {
                error: err_io,
                executable_path,
            })
        }
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

    let status: std::process::ExitStatus;
    match proc.wait() {
        Ok(n) => {
            status = n;
        }
        Err(err_io) => {
            return std::result::Result::Err(ProcessError::CannotWait {
                error: err_io,
                executable_path,
            })
        }
    }
    if !status.success() {
        return std::result::Result::Ok(ProcStatus::Terminated);
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
    return std::result::Result::Ok(ProcStatus::Running(pid));
}

/// Error variants related to process lifecycles because the standard library
/// considers everything just an IO error.
/// TODO: Add information about what was attempted somehow: at least the path of
///       the executable!
#[derive(Debug)]
pub enum ProcessError {
    CannotSpawn {
        executable_path: std::path::PathBuf,
        error: std::io::Error,
    },
    CannotWait {
        executable_path: std::path::PathBuf,
        error: std::io::Error,
    },
}
impl std::error::Error for ProcessError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ProcessError::CannotSpawn { ref error, .. } => Some(error),
            ProcessError::CannotWait { ref error, .. } => Some(error),
        }
    }
}
impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ProcessError::CannotSpawn {
                executable_path: ref path,
                ..
            } => {
                return write!(f, "CannotSpawn {:?}", path);
            }
            ProcessError::CannotWait {
                executable_path: ref path,
                ..
            } => {
                return write!(f, "CannotWait {:?}", path);
            }
        }
    }
}
impl From<ProcessError> for crate::FatalError {
    fn from(value: ProcessError) -> Self {
        return Self::ExternalCommandError(value);
    }
}

/// Errors related to forking a thing (RustDedicated game server) into an
/// independent process.
#[derive(Debug)]
pub enum ForkError {
    CannotReceivePID(std::sync::mpsc::RecvError),
    Other(ProcessError),
}
impl std::error::Error for ForkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ForkError::CannotReceivePID(ref e) => Some(e),
            ForkError::Other(ref e) => Some(e),
        }
    }
}
impl std::fmt::Display for ForkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ForkError::CannotReceivePID(_) => {
                return write!(f, "CannotTransmitPID");
            }
            ForkError::Other(_) => {
                return write!(f, "Other");
            }
        }
    }
}
impl From<ForkError> for crate::FatalError {
    fn from(value: ForkError) -> Self {
        return Self::GameServerForkError(value);
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
pub fn launch_fork(executable_path: std::path::PathBuf) -> std::result::Result<Fork, ForkError> {
    let (tx, rx) = std::sync::mpsc::channel::<std::result::Result<u32, ForkError>>();

    let jh = std::thread::spawn(move || {
        /*
          PID of the resulting forked process.
        */
        let pid: u32;

        let mut command = std::process::Command::new(&executable_path);

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
            Err(err_io) => {
                send_result(
                    tx,
                    std::result::Result::Err(ForkError::Other(ProcessError::CannotSpawn {
                        error: err_io,
                        executable_path,
                    })),
                );
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
        Err(err_recv) => {
            return std::result::Result::Err(ForkError::CannotReceivePID(err_recv));
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
