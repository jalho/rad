enum RDSState {
    STARTING,
    HEALTHY,
    UNHEALTHY,
    TERMINATED,
}

pub struct RDS {
    pid: Option<u32>,
    state: RDSState,
}

impl RDS {
    pub const fn new() -> Self {
        return RDS {
          pid: None,
          state: RDSState::TERMINATED,
      };
    }

    pub fn noop(&mut self) -> String {
      self.state = RDSState::UNHEALTHY;
      return String::from("Hello from RDS!");
    }
}
