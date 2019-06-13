 extern crate libloading as lib;
extern crate ffmpeg;

use ffmpeg::format::register_all;
use ffmpeg::format::version;
use ffmpeg::format::configuration;

use ffmpeg::format;
use ffmpeg::codec;
use ffmpeg::rescale;
use ffmpeg::util;
 use std::path::Path;
 use core::borrow::Borrow;

 fn main() {
    unsafe {
        let ver= version();
        println!("ver ={} {} ",ver,configuration());
       // register_all() ;
        println!(" license() = {}",format::license());
       // fmContex : &mut format::Context;
        //codeContex :&mut codec::Context;
//E:\音视频\vidoe

       // format::open("test.mp4",format::Input());



    };
    ffmpeg::init();
    //let path = Path::new("E:\\音视频\\vidoe");
     /// let string = String::from("foo.txt");
 /// let from_string = Path::new(&string);
 /// let from_path = Path::new(&from_string);
 /// assert_eq!(from_string, from_path);
 let path = String::from("E:\\音视频\\vidoe\\11.mp4");
     let path = Path::new(&path);

    match ffmpeg::format::input( &path) {
        Ok(context) => {
            for (k, v) in context.metadata().iter() {
                println!("{}: {}", k, v);
            }

            if let Some(stream) = context.streams().best(ffmpeg::media::Type::Video) {
                println!("Best video stream index: {}", stream.index());
            }

            if let Some(stream) = context.streams().best(ffmpeg::media::Type::Audio) {
                println!("Best audio stream index: {}", stream.index());
            }

            if let Some(stream) = context.streams().best(ffmpeg::media::Type::Subtitle) {
                println!("Best subtitle stream index: {}", stream.index());
            }

            println!("duration (seconds): {:.2}", context.duration() as f64 / ffmpeg::ffi::AV_TIME_BASE as f64);

            for stream in context.streams() {
                println!("stream index {}:", stream.index());
                println!("\ttime_base: {}", stream.time_base());
                println!("\tstart_time: {}", stream.start_time());
                println!("\tduration (stream timebase): {}", stream.duration());
                println!("\tduration (seconds): {:.2}", stream.duration() as f64 * f64::from(stream.time_base()));
                println!("\tframes: {}", stream.frames());
                println!("\tdisposition: {:?}", stream.disposition());
                println!("\tdiscard: {:?}", stream.discard());
                println!("\trate: {}", stream.rate());

                let codec = stream.codec();
                println!("\t medium: {:?}", codec.medium());

                //println!("\tid: {:?}", codec.id());

                if codec.medium() == ffmpeg::media::Type::Video {
                    if let Ok(video) = codec.decoder().video() {
                        println!("\tbit_rate: {}", video.bit_rate());
                        println!("\tmax_rate: {}", video.max_bit_rate());
                        println!("\tdelay: {}", video.delay());
                        println!("\tvideo.width: {}", video.width());
                        println!("\tvideo.height: {}", video.height());
                        println!("\tvideo.format: {:?}", video.format());
                        println!("\tvideo.has_b_frames: {}", video.has_b_frames());
                        println!("\tvideo.aspect_ratio: {}", video.aspect_ratio());
                        println!("\tvideo.color_space: {:?}", video.color_space());
                        println!("\tvideo.color_range: {:?}", video.color_range());
                        println!("\tvideo.color_primaries: {:?}", video.color_primaries());
                        println!("\tvideo.color_transfer_characteristic: {:?}", video.color_transfer_characteristic());
                        println!("\tvideo.chroma_location: {:?}", video.chroma_location());
                        println!("\tvideo.references: {}", video.references());
                        println!("\tvideo.intra_dc_precision: {}", video.intra_dc_precision());
                    }
                }
                else if codec.medium() == ffmpeg::media::Type::Audio {
                    print!(" codec.medium() == ffmpeg::media::Type::Audi");
                    if let Ok(audio) = codec.decoder().audio() {
                        println!("\tbit_rate: {}", audio.bit_rate());
                        println!("\tmax_rate: {}", audio.max_bit_rate());
                        println!("\tdelay: {}", audio.delay());
                        println!("\taudio.rate: {}", audio.rate());
                        println!("\taudio.channels: {}", audio.channels());
                        println!("\taudio.format: {:?}", audio.format());
                        println!("\taudio.frames: {}", audio.frames());
                        println!("\taudio.align: {}", audio.align());
                        println!("\taudio.channel_layout: {:?}", audio.channel_layout());
                        println!("\taudio.frame_start: {:?}", audio.frame_start());
                    }
                }else {
                    eprint!("unknow type!!")
                }
            }

        }

        Err(error) =>
            println!("error: {}", error)
    }



    println!("Hello, world!");
}

fn call_dynamic() -> lib::Result<u32> {
    let lib = lib::Library::new("/path/to/liblibrary.so")?;
    unsafe {
        let func: lib::Symbol<unsafe extern fn() -> u32> = lib.get(b"my_func")?;
        Ok(func())
    }
}