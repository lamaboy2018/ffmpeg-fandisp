extern crate libloading as lib;
extern crate ffmpeg;

use libffmpeg::format::register_all;
use libffmpeg::format::version;
fn main() {
  /*  unsafe {
        let ver= version();
        println!("ver ={}",ver);
        register_all() };*/
    println!("Hello, world!");
}

fn call_dynamic() -> lib::Result<u32> {
    let lib = lib::Library::new("/path/to/liblibrary.so")?;
    unsafe {
        let func: lib::Symbol<unsafe extern fn() -> u32> = lib.get(b"my_func")?;
        Ok(func())
    }
}