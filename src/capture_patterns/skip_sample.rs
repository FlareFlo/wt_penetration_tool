use core::time;
use std::thread::sleep;
use std::time::{Duration, Instant};

use image::{ImageBuffer, RgbImage};
use inputbot::{KeybdKey, MouseButton, MouseCursor};
use scrap::{Capturer, Display};

use crate::resource_functions::{extract_pixel, get_frame};

pub fn skip_sample_capture(square_size_x: u32, square_size_y: u32, start: (i32, i32), wait: u64, skip_x: u32, skip_y: u32) {
	let display = Display::primary().expect("Couldn't find primary display.");
	let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");

	let screen_width = capturer.width();
	let screen_height = capturer.height();

	sleep(time::Duration::from_secs(3));

	let offset_x = start.0 as u32; //Left right
	let offset_y = start.1 as u32; //up down
	let mut img: RgbImage = ImageBuffer::new(screen_width as u32, screen_height as u32);

	let mut captured_pixels = 0_u32;

	let start = Instant::now();


	for mut pixel_y in 0..square_size_y / skip_y {
		for mut pixel_x in 0..square_size_x / skip_x {
			captured_pixels += 1;
			let pos_x = pixel_x * skip_x;
			let pos_y = pixel_y * skip_y;
			MouseCursor::move_abs((pos_x + offset_x) as i32, (pos_y + offset_y) as i32);

			std::thread::sleep(Duration::from_millis(wait));

			let frame = get_frame(&mut capturer);
			let frame_vec = frame.to_vec();
			let pixel = extract_pixel(pos_x as usize, pos_y as usize, offset_x, offset_y, screen_width, &frame_vec);
			img.put_pixel(pos_x + offset_x, pos_y + offset_y, pixel);

			if MouseButton::LeftButton.is_pressed() | KeybdKey::EscapeKey.is_pressed() {
				panic!("Mouse was distrubed/moved")
			}
		}
	}
	let stop = start.elapsed();
	println!("{}", captured_pixels);

	println!("Time taken: {}ms", stop.as_millis());

	img.save("capture.png").unwrap();
}