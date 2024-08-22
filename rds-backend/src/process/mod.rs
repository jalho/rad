/// Get the age of a process by PID.
pub fn get_age(pid: u64) -> std::result::Result<std::time::Duration, ProcessError> {
    let arg: &str = &pid.to_string();
    let n = get_integer_value_with("ps", vec!["-o", "etimes=", "-p", arg])?;
    let elapsed_secs = std::time::Duration::from_secs(n);
    return std::result::Result::Ok(elapsed_secs);
}

pub enum ProcStatus {
    Terminated,
    Running(u64), // PID
}
/// Get process status by executable name.
pub fn get_pid(seekable: &str) -> std::result::Result<ProcStatus, ProcessError> {
    let pid: u64;
    match get_integer_value_with("pgrep", vec![seekable]) {
        Ok(n) => pid = n,
        Err(err) => match err {
            ProcessError::ErrorExitStatus { .. } => {
                return std::result::Result::Ok(ProcStatus::Terminated)
            }
            _ => return std::result::Result::Err(err),
        },
    }
    return std::result::Result::Ok(ProcStatus::Running(pid));
}

fn get_integer_value_with(
    executable: &str,
    args: Vec<&str>,
) -> std::result::Result<u64, ProcessError> {
    let mut child: std::process::Child;
    let executable_path = std::path::PathBuf::from(executable);
    match std::process::Command::new(&executable_path)
        .args(&args)
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Ok(n) => child = n,
        Err(err_io) => {
            return std::result::Result::Err(ProcessError::CannotSpawn {
                cause: err_io,
                executable_path,
            });
        }
    }
    let stdout: std::process::ChildStdout;
    match child.stdout.take() {
        Some(n) => stdout = n,
        None => {
            return std::result::Result::Err(ProcessError::CannotGetStdoutHandle {
                executable_path,
            });
        }
    }
    let mut reader = std::io::BufReader::new(stdout);

    let status: std::process::ExitStatus;
    match child.wait() {
        Ok(n) => status = n,
        Err(err_io) => {
            return std::result::Result::Err(ProcessError::CannotWait {
                cause: err_io,
                executable_path,
            });
        }
    }
    if !status.success() {
        return std::result::Result::Err(ProcessError::ErrorExitStatus {
            executable_path,
            input: args.join(" "),
        });
    }

    let integer_parsed: u64;
    let mut line = String::new();
    match std::io::BufRead::read_line(&mut reader, &mut line) {
        Ok(_) => {}
        Err(err_read) => {
            return std::result::Result::Err(ProcessError::CannotReadStdout {
                executable_path,
                cause: err_read,
            })
        }
    }
    let line = line.trim();
    match line.parse::<u64>() {
        Ok(n) => integer_parsed = n,
        Err(err_parse) => {
            return std::result::Result::Err(ProcessError::CannotParseStdout {
                executable_path,
                input: line.into(),
                cause: err_parse,
            })
        }
    }
    return std::result::Result::Ok(integer_parsed);
}

/// Error variants related to process lifecycles because the standard library
/// considers everything just an IO error.
#[derive(Debug)]
pub enum ProcessError {
    CannotSpawn {
        executable_path: std::path::PathBuf,
        cause: std::io::Error,
    },
    CannotWait {
        executable_path: std::path::PathBuf,
        cause: std::io::Error,
    },
    CannotGetStdoutHandle {
        executable_path: std::path::PathBuf,
    },
    CannotReadStdout {
        executable_path: std::path::PathBuf,
        cause: std::io::Error,
    },
    CannotParseStdout {
        executable_path: std::path::PathBuf,
        cause: std::num::ParseIntError,
        input: String,
    },
    ErrorExitStatus {
        executable_path: std::path::PathBuf,
        input: String,
    },
}
impl std::error::Error for ProcessError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ProcessError::CannotSpawn {
                cause: ref error, ..
            } => Some(error),
            ProcessError::CannotWait {
                cause: ref error, ..
            } => Some(error),
            ProcessError::CannotGetStdoutHandle { .. } => None,
            ProcessError::CannotReadStdout {
                cause: ref error, ..
            } => Some(error),
            ProcessError::CannotParseStdout {
                cause: ref error, ..
            } => Some(error),
            ProcessError::ErrorExitStatus { .. } => None,
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
                write!(f, "cannot spawn process from executable {:?}", path)
            }
            ProcessError::CannotWait {
                executable_path: ref path,
                ..
            } => {
                write!(f, "failed to wait process from executable {:?}", path)
            }
            ProcessError::CannotGetStdoutHandle {
                executable_path: ref path,
            } => write!(
                f,
                "cannot get stdout handle of process from executable {:?}",
                path
            ),
            ProcessError::CannotReadStdout {
                executable_path: ref path,
                ..
            } => write!(
                f,
                "cannot read stdout of process from executable {:?}",
                path
            ),
            ProcessError::CannotParseStdout {
                executable_path: ref path,
                ref input,
                ..
            } => write!(
                f,
                "cannot parse integer from stdout of process from executable {:?}: \"{}\"",
                path, input
            ),
            ProcessError::ErrorExitStatus {
                executable_path: ref path,
                ref input,
            } => write!(
                f,
                "exited with error status from process from executable {:?}, input: \"{}\"",
                path, input
            ),
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
                        cause: err_io,
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
