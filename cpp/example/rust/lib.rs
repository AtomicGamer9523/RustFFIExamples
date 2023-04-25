#[path="log.level"]mod level;use level::*;
use std::ffi::*;

#[utils::ffi(unsafe)]
fn rslog(level: i32, message: *const c_char) {
    let level: Level = level.into();
    println!("[rust] ({}) {}", level, CStr::from_ptr(message).to_str().unwrap() );
}