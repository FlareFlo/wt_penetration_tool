use std::thread::sleep;
use std::time::{Duration, Instant};

use image::{GenericImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
use scrap::{Capturer, Display};

mod main_captrs;

fn main() {
	let display = Display::primary().expect("Couldn't find primary display.");
	let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
	let (w, h) = (capturer.width(), capturer.height());


	let frame: Vec<u8>;

	let mut stop: Duration = Duration::from_millis(0);

	loop {
		let start = Instant::now();
		if let Ok(frame_result) = capturer.frame() {
			frame = frame_result.to_vec();
			stop = start.elapsed();
			break;
		}
	}
	let x = 0;
	let y = 0;
	const DIM_X: u32 = 75;
	const DIM_Y: u32 = 75;

	let mut img: RgbImage = ImageBuffer::new(DIM_X, DIM_Y);

	let location = (y * DIM_X + x) as usize;
	let rgb = Rgb::from([frame[location + 2], frame[location + 1], frame[location]]);

	img.put_pixel(x as u32, y as u32, rgb);
	img.save("experimental.png").unwrap();

	//Indexes:
	// 0 = B
	// 1 = G
	// 2 = R
	// 3 = Alpha

	println!("{}", frame[4]);
	println!("Time taken: {}ms", stop.as_millis());
}