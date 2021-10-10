// use core::time;
// use std::collections::HashMap;
// use std::thread::sleep;
// use std::time::{Duration, Instant};
//
// use image::{ImageBuffer, Rgb, RgbImage};
// use inputbot::MouseCursor;
// use scrap::{Capturer, Display};
//
// use crate::resource_functions::{extract_pixel, get_frame};
//
// pub fn interpolated_capture(square_size_x: u32, square_size_y: u32, num_squares_x: u32, num_squares_y: u32, wait: u64) {
// 	let display = Display::primary().expect("Couldn't find primary display.");
// 	let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
//
// 	// Configuration
// 	// const DIM_X: u32 = 10;     // Width
// 	// const DIM_Y: u32 = 10;     //Height
// 	// const OFF_X: u32 = 1;       // Interpolation / pixel density setting (experimental)
// 	// const OFF_Y: u32 = 1;       // Interpolation / pixel density setting (experimental)
//
// 	// const COOLDOWN: u64 = 10;
// 	let screen_width = capturer.width();
// 	// let screen_height = capturer.height();
//
// 	sleep(time::Duration::from_secs(3));
//
// 	let offset_x = (MouseCursor::pos().0) as u32; //Left right
// 	let offset_y = (MouseCursor::pos().1) as u32; //up down
// 	let mut img: RgbImage = ImageBuffer::new(square_size_x * num_squares_x, square_size_y * num_squares_y);
//
// 	// let pixel = capture_pixel((y) as usize, (x) as usize, 1920 as usize);
// 	// println!("{} {} {}", pixel[0], pixel[1], pixel[2]);
//
// 	let start = Instant::now();
// 	let mut skip_counter: u32 = 0;
//
//
// 	for square_index_x in 0..num_squares_x {
// 		for square_index_y in 0..num_squares_y {
// 			// let current_position = MouseCursor::pos();
// 			let mut corner_pixels = Vec::with_capacity(4); //New vector with the capacity of the pixels we will check
//
// 			// waiting 3 frames on average due to mouse-cursor delays
//
// 			MouseCursor::move_abs((square_index_x * square_size_x + offset_x) as i32, (square_index_y * square_size_y + offset_y) as i32);
// 			std::thread::sleep(Duration::from_millis(wait));
// 			let frame_vec = get_frame(&mut capturer).to_vec();
// 			corner_pixels.push(extract_pixel((square_index_x * square_size_x) as usize, (square_index_y * square_size_y) as usize, offset_x, offset_y, screen_width, &frame_vec));
//
// 			MouseCursor::move_abs(((square_index_x + 1) * square_size_x - 1 + offset_x) as i32, (square_index_y * square_size_y + offset_y) as i32);
// 			std::thread::sleep(Duration::from_millis(wait));
// 			let frame_vec = get_frame(&mut capturer).to_vec();
// 			corner_pixels.push(extract_pixel(((square_index_x + 1) * square_size_x - 1) as usize, (square_index_y * square_size_y) as usize, offset_x, offset_y, screen_width, &frame_vec));
//
// 			MouseCursor::move_abs((square_index_x * square_size_x + offset_x) as i32, ((square_index_y + 1) * square_size_y - 1 + offset_y) as i32);
// 			std::thread::sleep(Duration::from_millis(wait));
// 			let frame_vec = get_frame(&mut capturer).to_vec();
// 			corner_pixels.push(extract_pixel((square_index_x * square_size_x) as usize, ((square_index_y + 1) * square_size_y - 1) as usize, offset_x, offset_y, screen_width, &frame_vec));
//
// 			MouseCursor::move_abs(((square_index_x + 1) * square_size_x - 1 + offset_x) as i32, ((square_index_y + 1) * square_size_y - 1 + offset_y) as i32);
// 			std::thread::sleep(Duration::from_millis(wait));
// 			let frame_vec = get_frame(&mut capturer).to_vec();
// 			corner_pixels.push(extract_pixel(((square_index_x + 1) * square_size_x - 1) as usize, ((square_index_y + 1) * square_size_y - 1) as usize, offset_x, offset_y, screen_width, &frame_vec));
//
// 			//Test for faster image manipulation as the loop below is rather slow
// 			// let mut image: dyn GenericImage;
// 			// let test: Rect = Rect {
// 			// 	x,
// 			// 	y,
// 			// 	width: 0,
// 			// 	height: 0
// 			// };
// 			// imga.copy_within(test, 0,0);
//
// 			// MouseCursor::move_abs(current_position.0, current_position.1);
// 			if color_amount_over_threshold_checker(&corner_pixels, 4) {
// 				// println!("All corners are the same, filling rectangle!");
// 				for pixel_x in 0..square_size_x {
// 					for pixel_y in 0..square_size_y {
// 						img.put_pixel(square_index_x * square_size_x + pixel_x, square_index_y * square_size_y + pixel_y, corner_pixels[0]);
// 					}
// 				}
// 				skip_counter += 1;
// 				continue;
// 			}
//
// 			for pixel_y in 0..square_size_y {
// 				for pixel_x in 0..square_size_x {
// 					let pos_x = square_index_x * square_size_x + pixel_x;
// 					let pos_y = square_index_y * square_size_y + pixel_y;
// 					MouseCursor::move_abs((pos_x + offset_x) as i32, (pos_y + offset_y) as i32);
//
// 					std::thread::sleep(Duration::from_millis(wait));
//
// 					let frame = get_frame(&mut capturer);
// 					let frame_vec = frame.to_vec();
// 					let pixel = extract_pixel(pos_x as usize, pos_y as usize, offset_x, offset_y, screen_width, &frame_vec);
// 					img.put_pixel(pos_x, pos_y, pixel);
// 				}
// 			}
// 		}
// 	}
//
// 	let stop = start.elapsed();
// 	println!("Time taken: {}ms, skipped pixel perfect scan {} times", stop.as_millis(), skip_counter);
//
// 	img.save("capture.png").unwrap();
//
// 	// println!("The program took {}ms and captured with {} FPS/PPS.", stop.as_millis(), (DIM_X * DIM_Y) as f64 / stop.as_secs_f64());
//
// 	// let mut debug_frame;
// 	// loop {
// 	// 	if let Ok(frame_new) = capturer.frame() {
// 	// 		debug_frame = frame_new;
// 	// 		break;
// 	// 	}
// 	// };
// 	//
// 	// let debug_pixel = extract_pixel(1946, 793, 0, 0, 3840, &debug_frame.to_vec());
// 	// println!("Debug pixel is {:?}", debug_pixel);
// }
// #[inline(always)]
// fn color_amount_over_threshold_checker(vec: &Vec<Rgb<u8>>, threshold: u32) -> bool {
// 	let mut color_map = HashMap::new();
//
// 	for color in vec {
// 		*color_map.entry(color).or_insert(0) += 1;
// 	}
//
// 	let max_color_amount = color_map.iter()
// 		.max_by_key(|entry| entry.1)
// 		.map(|(key, value)| value)
// 		.unwrap();
//
// 	*max_color_amount >= threshold
// }
//
// //Deprecated function
// // #[inline(always)]
// // fn all_corners_same(corner_pixels: &[Rgb<u8>; 4]) -> bool {
// // 	corner_pixels[0] == corner_pixels[1] && corner_pixels[0] == corner_pixels[2] && corner_pixels[0] == corner_pixels[3]
// // }