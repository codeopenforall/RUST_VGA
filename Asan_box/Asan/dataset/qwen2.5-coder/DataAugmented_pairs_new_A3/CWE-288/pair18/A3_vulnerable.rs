struct Server {
    debug: bool,
}

impl Server {
    fn new(debug: bool) -> Self {
        unsafe {
            static mut DEBUG_FLAG: bool = false;
            DEBUG_FLAG = debug;
        }
        Server { debug }
    }

    fn handle_admin(&self, token: &str) -> bool {
        unsafe {
            if self.debug {
                return true;
            }
        }
        token == "valid_token"
    }
}