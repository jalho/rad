mod process;
mod rds;

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
fn main() -> std::result::Result<(), rds::ForkError> {
    /*
        PID of the process running RustDedicated game server.
    */
    let pid_rds: u32;

    match process::get_pid("RustDedicated") {
        Ok(process::ProcStatus::TERMINATED) => {
            let fork: rds::Fork = rds::rds_launch_fork()?;
            _ = fork.jh.join();
            println!(
                "[INFO] - Launched RustDedicated in an independent process with PID {}",
                fork.pid
            );
            pid_rds = fork.pid;
        }
        Ok(process::ProcStatus::RUNNING(pid)) => {
            println!(
                "[INFO] - Detected existing RustDedicated process with PID {}",
                pid
            );
            pid_rds = pid;
        }
        Err(_) => todo!(),
    }

    return std::result::Result::Ok(());
}
