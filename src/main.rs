use core::time;
use std::thread::sleep;
use std::time::Duration;

use captrs::*;
use image::{GenericImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
use inputbot::{*, KeybdKey::*, MouseButton::*};
use rand::Rng;

const RED: usize = 0;
const GREEN: usize = 1;
const BLUE: usize = 2;
const ALPHA: usize = 3;

fn main() {
	// let mut img = image::open("test.png").unwrap();
	//
	// let pixel = img.get_pixel(0, 0);
	// println!("{} {} {} {} ", pixel.0[RED], pixel.0[GREEN], pixel.0[BLUE], pixel.0[ALPHA]);

	// sleep(time::Duration::from_secs(3));
	//
	// for height in 1..=500 {
	// 	for width in 1..500 {
	// 		MouseCursor::move_rel(1,0);
	// 	}
	// 	MouseCursor::move_rel(-500,1);
	// }

	screenshot();
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
			// let coords = new[(horizontal) as usize][(vertical) as usize];
			// println!("R: {} G: {} B: {}", coords.r, coords.g, coords.b);
			// println!("x: {} y: {}", horizontal, vertical);
			img.put_pixel(horizontal, vertical, Rgb::from([ps[(vertical * 1920 + horizontal) as usize].r,ps[(vertical * 1920 + horizontal) as usize].g,ps[(vertical * 1920 + horizontal) as usize].b]));
		}
	}
	img.save("yeet.png").unwrap();

	// let x = (MouseCursor::pos().0);
	// let y = (MouseCursor::pos().1);
}