extern crate libloading as lib;
extern crate ffmpeg_sys;

use ffmpeg_sys::av_register_all;

fn main() {
    unsafe {
        ffmpeg_sys::av_register_all();
        av_register_all() };
    println!("Hello, world!");
}

fn call_dynamic() -> lib::Result<u32> {
    let lib = lib::Library::new("/path/to/liblibrary.so")?;
    unsafe {
        let func: lib::Symbol<unsafe extern fn() -> u32> = lib.get(b"my_func")?;
        Ok(func())
    }
}