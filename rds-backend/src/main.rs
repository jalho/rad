fn main() {
    exec_observable("echo", ["hello", "world"]);
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

    match cmd_process.wait() {
        Ok(n) => {
            return n;
        }
        Err(_) => todo!(),
    }
}
