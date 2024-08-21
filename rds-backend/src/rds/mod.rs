use std::sync::{Arc, Mutex};

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

    pub fn noop(&self) -> String {
      return String::from("Hello from RDS!");
    }
}
