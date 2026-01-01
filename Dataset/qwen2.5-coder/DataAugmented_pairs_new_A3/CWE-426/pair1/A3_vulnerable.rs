struct Executor {}

impl Executor {
    pub fn prepare(&self, cmd: &str) -> String {
        cmd.to_string()
    }
}