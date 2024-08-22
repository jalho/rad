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