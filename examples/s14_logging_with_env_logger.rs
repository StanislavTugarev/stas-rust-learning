use env_logger::Builder;
use log::{LevelFilter, debug, error, info, warn};

// error! (priority 5)- serious error (program cannot continue its execution)
// warn! (priority 4)- warnings, not fatal error
// info! (priority 3)- general information
// debug! (priority 2)- detailed diagnostic
// trace! (priority 1)- very fine-grained information

fn main() {
    // env_logger::init();
    // Builder::new().filter_level(LevelFilter::Trace).init();
    // login("guest", "");

    Builder::new()
        .filter_module("rust_test", LevelFilter::Debug)
        .filter_module("rust_test::security", LevelFilter::Trace)
        .init();
    login("guest", "");
    security::check_access("admin");
}

fn login(username: &str, password: &str) {
    debug!("Trying to authenticate user: {}", username);

    if username.is_empty() || password.is_empty() {
        error!("Empty username or password");
        return;
    }

    if username == "admin" && password == "secret" {
        info!("Admin login successful");
    } else {
        warn!("Login failed for user: {}", username)
    }
}

mod security {
    use log::{debug, info, trace};

    pub fn check_access(user: &str) {
        debug!("Checking access rights for: {}", user);
        trace!("Performing low-level permission check for: {}", user);

        if user == "admin" {
            info!("Access granted to admin");
        } else {
            info!("Limited access for user: {}", user)
        }
    }
}
