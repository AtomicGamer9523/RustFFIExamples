#[utils::ffi]
fn addone_rs(num: i32) -> i32 {
    println!("[rust] addone({})", &num);
    num + 1
}