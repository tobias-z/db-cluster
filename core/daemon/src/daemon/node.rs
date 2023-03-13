use super::Daemon;

impl Daemon {
    /// Ensure that the state contains the admin server applications
    pub fn init_as_admin(&self) {
        let state = self.desired_state.lock().unwrap();
    }
}
