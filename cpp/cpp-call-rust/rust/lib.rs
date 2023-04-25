use utils::*;

#[repr(C)]
pub struct MyStruct {

}

#[ffi]
fn addone(num: i32) -> i32 {
    println!("[rust] addone({})", &num);
    num + 1
}