use std::thread::sleep;

use inputbot::{KeybdKey::*, MouseButton::*, *};
use core::time;
use rand::Rng;

use captrs::*;
use std::time::Duration;
use image::GenericImageView;
use image::ImageBuffer;


fn main() {

	// sleep(time::Duration::from_secs(3));
	//
	// for height in 1..=500 {
	// 	for width in 1..500 {
	// 		MouseCursor::move_rel(1,0);
	// 	}
	// 	MouseCursor::move_rel(-500,1);
	// }
	capture();
}
fn capture() {
	let mut capturer = Capturer::new(0).unwrap();

	loop {
		let ps = capturer.capture_frame().unwrap();
		let mut new: Vec<Vec<Bgr8>> = Vec::new();
		for i in 0..1920 {
			new.push(ps[i..i+1080].to_vec());
		}
		let x = (MouseCursor::pos().0);
		let y = (MouseCursor::pos().1);
		let coords = new[(x + 20) as usize][(y + 20) as usize];
		println!("R: {} G: {} B: {}", coords.r, coords.g, coords.b);
		println!("x: {} y: {}", x, y);
		sleep(Duration::from_millis(1000));
	}
}

fn test() {
	let im = image::open("test.png").unwrap();
	let first_pixel = im.get_pixel(0, 500);
	println!("{:?}", first_pixel);
}
