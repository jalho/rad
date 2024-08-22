mod log;
mod process;
mod rds;

use crate::log::SourceLoggable;

/// First, the program checks if the Rust game server (executable name
/// _RustDedicated_) is running, and starts it if not. If a new start is
/// required, any available updates are installed too. The started process is
/// forked into independent process to allow separate lifecycles, and its STDOUT
/// and STDERR are directed to system shared locations to allow reading them
/// from later processes. Then, while the game server is up or starting up, this
/// program proceeds with two threads of execution:
///
/// 1. Health monitoring of the game server that is running in an independent
///    process.
///
/// 2. HTTP and WebSocket server integrating with the game server and its system
///    shared resources (STDOUT, STDERR).
///
/// ### Thread 1: "Health monitoring"
///
/// ```pseudocode
/// loop {
///     if !rds::is_healthy() {
///         rds::terminate_fork();
///         rds::install_updates();
///         rds::launch_fork();
///     }
///     sleep(briefly);
/// }
/// ```
///
/// ### Thread 2: "HTTP and WebSocket server"
///
/// Some single threaded async runtime like `tokio` passing messages both ways
/// between authorized WebSocket clients and the RCON endpoint of the forked
/// game server process.
///
/// Separate thread so the whole code base wouldn't be infected with whatever
/// async runtime framework is used here: I want to keep async Rust to the
/// minimum where it provides significant value.
///
/// ```pseudocode
/// let rcon_duplex = rds::RconDuplex::new();
/// continuous task {
///     server.drop_dead_connections();
///     server.accept_connection();
/// }
/// continuous task {
///     for connection in server.connections {
///         rcon_duplex.write(connection);
///         connection.write(rcon_duplex);
///     }
/// }
/// ```
fn main() -> std::result::Result<(), FatalError> {
    /*
        PID of the process running the game server.
    */
    let pid_rds: u32;

    match process::get_pid("RustDedicated") {
        Ok(process::ProcStatus::Terminated) => {
            let fork: process::Fork = process::launch_fork("./RustDedicated".into())?;
            _ = fork.jh.join();
            println!(
                "[INFO] - Launched game server in an independent process with PID {}",
                fork.pid
            );
            pid_rds = fork.pid;
        }
        Ok(process::ProcStatus::Running(pid)) => {
            println!(
                "[INFO] - Detected existing game server process with PID {}",
                pid
            );
            pid_rds = pid;
        }
        Err(err_get_pid) => {
            let error: FatalError = FatalError::from(err_get_pid);
            error.log();
            return std::result::Result::Err(error);
        }
    }

    return std::result::Result::Ok(());
}

/// The errors we may return with from main that we can't recover from.
#[derive(Debug)]
enum FatalError {
    GameServerForkError(process::ForkError),
    ExternalCommandError(process::ProcessError),
}
impl std::error::Error for FatalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            FatalError::GameServerForkError(ref e) => Some(e),
            FatalError::ExternalCommandError(ref e) => Some(e),
        }
    }
}
impl std::fmt::Display for FatalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FatalError::GameServerForkError(_) => {
                return write!(
                    f,
                    "could not launch game server into an independent process"
                );
            }
            FatalError::ExternalCommandError(_) => {
                return write!(f, "failed to execute external command");
            }
        }
    }
}
impl crate::log::SourceLoggable for crate::FatalError {}
