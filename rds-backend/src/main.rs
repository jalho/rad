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
fn main() {
    _ = rds::rds_launch_fork().join();
}
