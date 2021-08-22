use core::time;
use std::thread::sleep;
use std::time::{Duration, Instant};

use image::{ImageBuffer, RgbImage};
use inputbot::MouseCursor;
use scrap::{Capturer, Display};

use crate::resource_functions::{extract_pixel, get_frame};

pub fn full_capture(square_size_x: u32, square_size_y: u32, wait: u64) {
	let display = Display::primary().expect("Couldn't find primary display.");
	let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");

	let screen_width = capturer.width();

	sleep(time::Duration::from_secs(3));

	let offset_x = (MouseCursor::pos().0) as u32; //Left right
	let offset_y = (MouseCursor::pos().1) as u32; //up down
	let mut img: RgbImage = ImageBuffer::new(square_size_x, square_size_y);


	let start = Instant::now();


	for pixel_y in 0..square_size_y {
		for pixel_x in 0..square_size_x {
			let pos_x = pixel_x;
			let pos_y = pixel_y;
			MouseCursor::move_abs((pos_x + offset_x) as i32, (pos_y + offset_y) as i32);

			std::thread::sleep(Duration::from_millis(wait));

			let frame = get_frame(&mut capturer);
			let frame_vec = frame.to_vec();
			let pixel = extract_pixel(pos_x as usize, pos_y as usize, offset_x, offset_y, screen_width, &frame_vec);
			img.put_pixel(pos_x, pos_y, pixel);
		}
	}
	let stop = start.elapsed();

	println!("Time taken: {}ms", stop.as_millis());

	img.save("capture.png").unwrap();
}