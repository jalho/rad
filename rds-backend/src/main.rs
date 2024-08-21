/// Three threads of execution on the operating system level:
///
/// 1. *RustDedicated* game server
/// 2. Health monitoring of the game server
/// 3. HTTP and WebSocket server integrating with the game server
///
/// ### Thread 1: "RustDedicated game server"
///
/// ```pseudocode
/// loop {
///     rds.unlock();
///     if !rds.is_healthy() {
///         rds.stop();
///         rds.update();
///         rds.start();
///     } else {
///         rds.process_output_blocking();
///     }
/// }
/// ```
///
/// ### Thread 2: "Health monitoring of the game server"
///
/// Separate thread because Thread 1 is inherently blocking because it processes
/// STDOUT and STDERR that are typically handled in a blocking manner.
///
/// ```pseudocode
/// loop {
///     rds.unlock();
///     rds.check_health();
///     sleep(interval);
/// }
/// ```
///
/// ## Thread 3: "HTTP and WebSocket server"
///
/// Some kind of async single-thread executor accepting authorized WebSocket
/// connections and passing messages between them and the global `rds` instance.
///
/// Separate thread so the whole code base wouldn't be infected with whatever
/// async runtime framework is used here: I want to keep async Rust to the
/// minimum where it provides significant value.
///
/// ```pseudocode
/// non-blocking loop { // tokio-singlethreaded-whatever
///     server.accept(); // non-blocking
///     for web_client in server.clients {
///         rds.unlock();
///         rds.handle_duplex(web_client); // non-blocking
///     }
/// }
/// ```
///
/// ### Global thread-safe smart pointer `rds`
///
/// - `Arc<Mutex<whatever>>`: Must be accessible in all of the 3 threads.
/// - Stores:
///   - ID of the RustDedicated game server process (PID)
///   - state of the process: STARTING | HEALTHY | UNHEALTHY | TERMINATED
///     - enum shall contain time instant at which the state was last updated
/// - Provides methods for:
///   - stopping and starting
///   - updating all dependencies: SteamCMD, RustDediceted, Carbon
///   - passing messages both ways between WebSocket clients and itself
fn main() {}
