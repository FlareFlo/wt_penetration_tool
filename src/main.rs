use core::time;
use std::thread::sleep;
use std::time::{Duration, Instant};

use captrs::*;
use image::{GenericImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
use inputbot::{*, KeybdKey::*, MouseButton::*};
use rand::Rng;

const RED: usize = 0;
const GREEN: usize = 1;
const BLUE: usize = 2;
const ALPHA: usize = 3;

fn main() {
	// Configuration
	const DIM_X: u32 = 10; 			// Width
	const DIM_Y: u32 = 10; 			//Height
	const OFF_X: i32 = 1; 			// Interpolation / pixel density setting (experimental)
	const OFF_Y: i32 = 1;			// Interpolation / pixel density setting (experimental)
	const SCREEN_W: u32 = 3840; 	// Screen width

	sleep(time::Duration::from_secs(3));

	let x = (MouseCursor::pos().0) as u32; //Left right
	let y = (MouseCursor::pos().1) as u32; //up down
	let mut img: RgbImage = ImageBuffer::new(DIM_X, DIM_Y);

	// let pixel = capture_pixel((y) as usize, (x) as usize, 1920 as usize);
	// println!("{} {} {}", pixel[0], pixel[1], pixel[2]);

	let start = Instant::now();

	let mut capturer = Capturer::new(0).unwrap();
	sleep(time::Duration::from_millis(50));

	for width in 0..(DIM_Y) {
		for height in 0..(DIM_X) {
			MouseCursor::move_rel(OFF_X, 0);
			let pixel = capture_pixel((height + y) as usize, (width + x) as usize, SCREEN_W as usize, capturer.capture_frame().unwrap());
			img.put_pixel(width as u32, height as u32, pixel);
			// println!("{} {} {}", pixel[0], pixel[1], pixel[2]);
		}
		MouseCursor::move_rel(-(DIM_Y as i32), OFF_Y);
	}
	let stop = start.elapsed();

	img.save("capture.png").unwrap();

	// Reference time is 23500 ms
	println!("The program took {}ms and is {}% of the reference timing", stop.as_millis(), stop.as_millis() / 23500 * 100);
}

fn capture_pixel(y: usize, x: usize, dim_x: usize, cap: Vec<Bgr8>) -> Rgb<u8> {
	// let cap = capturer.capture_frame().unwrap();
	let rgb = Rgb::from([cap[(y * dim_x + x) as usize].r, cap[(y * dim_x + x) as usize].g, cap[(y * dim_x + x) as usize].b]);
	return rgb;
}


fn screenshot() {
	// sleep(Duration::from_millis(3000));
	let mut capturer = Capturer::new(0).unwrap();
	sleep(Duration::from_millis(100));
	let mut ps = capturer.capture_frame().unwrap();
	println!("Captured");


	let mut img: RgbImage = ImageBuffer::new(1920, 1080);
	for horizontal in 0..1919 {
		for vertical in 0..1079 {
			img.put_pixel(horizontal, vertical, Rgb::from([ps[(vertical * 1920 + horizontal) as usize].r, ps[(vertical * 1920 + horizontal) as usize].g, ps[(vertical * 1920 + horizontal) as usize].b]));
		}
	}
	img.save("yeet.png").unwrap();
}