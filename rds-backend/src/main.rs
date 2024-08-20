fn main() {
    exec_observable("echo", ["hello", "world"]);
}

// TODO: Accept some opts that allow for writing stderr, stdout somewhere (e.g.
//       some buffer that'll be flushed to a number of WebSocket clients and
//       perhaps to disk too?)
// TODO: Document!
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
