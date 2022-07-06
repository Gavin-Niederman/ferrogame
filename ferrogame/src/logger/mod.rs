use chrono;

pub fn info(message: String) {
    println!("\x1b[1;32m[{}] \x1b[1;32m[INFO] {}\x1b[0m", chrono::offset::Local::now().time(), message);
}

pub fn warn(message: String) {
    println!("\x1b[1;32m[{}] \x1b[1;33m[WARN] {}\x1b[0m", chrono::offset::Local::now().time() , message);
}

pub fn error(message: String) {
    println!("\x1b[1;32m[{}] \x1b[1;41m\x1b[38;5;232m[ERROR] {}\x1b[0m", chrono::offset::Local::now().time() , message);
}

pub fn debug(message: String) {
    println!("\x1b[1;32m[{}] \x1b[38;5;32m[DEBUG] {}\x1b[0m", chrono::offset::Local::now().time() , message);
}