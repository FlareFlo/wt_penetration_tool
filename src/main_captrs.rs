use core::time;
use std::thread::sleep;
use std::time::{Duration, Instant};

// use captrs::*;
use scrap::{Capturer, Display};
use image::{GenericImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
use inputbot::{*, KeybdKey::*, MouseButton::*};
use rand::Rng;

fn main_captrs() {
	// Configuration
	const DIM_X: u32 = 75;        // Width
	const DIM_Y: u32 = 75;     //Height
	const OFF_X: i32 = 1;       // Interpolation / pixel density setting (experimental)
	const OFF_Y: i32 = 1;       // Interpolation / pixel density setting (experimental)
	const SCREEN_W: u32 = 3840; // Screen width
	const COOLDOWN: u64 = 10;

	sleep(time::Duration::from_secs(3));

	let x = (MouseCursor::pos().0) as u32; //Left right
	let y = (MouseCursor::pos().1) as u32; //up down
	let mut img: RgbImage = ImageBuffer::new(DIM_X, DIM_Y);

	// let pixel = capture_pixel((y) as usize, (x) as usize, 1920 as usize);
	// println!("{} {} {}", pixel[0], pixel[1], pixel[2]);

	let start = Instant::now();

	let display = Display::primary().expect("Couldn't find primary display.");
	let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
	sleep(time::Duration::from_millis(50));

	for width in 0..(DIM_X) {
		for height in 0..(DIM_Y) {
			MouseCursor::move_rel(0, OFF_Y);
			// I love you GPU, but i really need those screenshots (NOTE: Loops until capture_frame() does not return timeout (refreshing faster than FPS))
			let frame;
			loop {
				if let Ok(frame_result) = capturer.frame() {
					frame = frame_result;
					break;
				}
			}
			let pixel = extract_pixel((height + y) as usize, (width + x) as usize, SCREEN_W as usize, frame.to_vec());
			img.put_pixel(width as u32, height as u32, pixel);
			// println!("{} {} {}", pixel[0], pixel[1], pixel[2]);
		}
		MouseCursor::move_rel(OFF_X, -(DIM_X as i32));
	}
	let stop = start.elapsed();

	img.save("capture.png").unwrap();

	let reference_time = 5.8;
	let actual_time = ((stop.as_secs_f64() / reference_time) * 100.0) - 100.0;
	println!("The program took {}ms and is {}% faster than reference time. It captured with {} FPS/PPS.", stop.as_millis(), actual_time, (DIM_X * DIM_Y) as f64 / stop.as_secs_f64());
}

fn extract_pixel(y: usize, x: usize, dim_x: usize, cap: Vec<u8>) -> Rgb<u8> {
	let location = (y * dim_x + x) as usize;
	let rgb = Rgb::from([cap[location + 2], cap[location + 1], cap[location]]);
	return rgb;
}