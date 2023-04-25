use std::ffi::*;
use utils::*;
mod level;
use level::*;

#[ffi(unsafe)]
fn rslog(level: i32, message: *const c_char) {
    let level: Level = level.into();
    println!("[rust] ({}) {}", level, CStr::from_ptr(message).to_str().unwrap() );
}