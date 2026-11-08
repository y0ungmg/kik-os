// Kernel logging subsystem

// Removed unused import

pub fn log(level: &str, message: &str) {
    // Print to serial
    crate::serial::print("[");
    crate::serial::print(level);
    crate::serial::print("] ");
    crate::serial::print(message);
    crate::serial::print("\n");
}

pub fn info(msg: &str) {
    log("INFO", msg);
}

pub fn warn(msg: &str) {
    log("WARN", msg);
}

pub fn error(msg: &str) {
    log("ERROR", msg);
}

pub fn debug(msg: &str) {
    log("DEBUG", msg);
}