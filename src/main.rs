use core::time;
use std::process::exit;
use std::thread::sleep;
use std::time::{Duration, Instant};

use image::{Frame, GenericImage, ImageBuffer, Rgb, RgbImage};
use image::math::Rect;
use inputbot::{KeybdKey::EscapeKey, MouseCursor};
use scrap::{Capturer, Display};

fn main() {
	let display = Display::primary().expect("Couldn't find primary display.");
	let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");

	// Configuration
	const DIM_X: u32 = 10;     // Width
	const DIM_Y: u32 = 10;     //Height
	const OFF_X: u32 = 1;       // Interpolation / pixel density setting (experimental)
	const OFF_Y: u32 = 1;       // Interpolation / pixel density setting (experimental)

	const SQUARE_SIZE_X: u32 = 30; //X Dimensions of the scan square
	const SQUARE_SIZE_Y: u32 = 30; //Y Dimensions of the scan square

	const NUM_SQUARES_X: u32 = 200;
	const NUM_SQUARES_Y: u32 = 55;

	// const COOLDOWN: u64 = 10;
	let screen_width = capturer.width();
	// let screen_height = capturer.height();

	sleep(time::Duration::from_secs(3));

	let offset_x = (MouseCursor::pos().0) as u32; //Left right
	let offset_y = (MouseCursor::pos().1) as u32; //up down
	let mut img: RgbImage = ImageBuffer::new(SQUARE_SIZE_X * NUM_SQUARES_X, SQUARE_SIZE_Y * NUM_SQUARES_Y);

	// let pixel = capture_pixel((y) as usize, (x) as usize, 1920 as usize);
	// println!("{} {} {}", pixel[0], pixel[1], pixel[2]);

	let start = Instant::now();


	for square_index_x in 0..NUM_SQUARES_X {
		for square_index_y in 0..NUM_SQUARES_Y {
			let current_position = MouseCursor::pos();
			let mut corner_pixels = [Rgb::from([0, 0, 0]); 4];
			// I would put it inside a function but right now the borrow checker flags it as multiple mutable borrow inside the loop


			//Lets do some fuckery
			//MouseCursor::move_rel(1, (SQUARE_SIZE_Y - 1) as i32);
			// let frame_vec = get_frame(&mut capturer).to_vec();
			// corner_pixels[0] = extract_pixel((square_index_x * SQUARE_SIZE_X) as usize, (square_index_y * SQUARE_SIZE_Y) as usize, offset_x, offset_y, screen_width, &frame_vec);
			//
			// MouseCursor::move_rel((SQUARE_SIZE_X - 1) as i32, 0);
			// let frame_vec = get_frame(&mut capturer).to_vec();
			// corner_pixels[1] = extract_pixel(((square_index_x + 1) * SQUARE_SIZE_X - 1) as usize, (square_index_y * SQUARE_SIZE_Y) as usize, offset_x, offset_y, screen_width, &frame_vec);
			//
			// MouseCursor::move_rel((SQUARE_SIZE_X - 1) as i32 * (-1), (SQUARE_SIZE_Y - 1) as i32 * (-1));
			// let frame_vec = get_frame(&mut capturer).to_vec();
			// corner_pixels[2] = extract_pixel((square_index_x * SQUARE_SIZE_X) as usize, ((square_index_y + 1) * SQUARE_SIZE_Y - 1) as usize, offset_x, offset_y, screen_width, &frame_vec);
			//
			// MouseCursor::move_rel((SQUARE_SIZE_X - 1) as i32, 0);
			// let frame_vec = get_frame(&mut capturer).to_vec();
			// corner_pixels[3] = extract_pixel(((square_index_x + 1) * SQUARE_SIZE_X - 1) as usize, ((square_index_y + 1) * SQUARE_SIZE_Y - 1) as usize, offset_x, offset_y, screen_width, &frame_vec);


			MouseCursor::move_abs((square_index_x * SQUARE_SIZE_X + offset_x) as i32, (square_index_y * SQUARE_SIZE_Y + offset_y) as i32);
			std::thread::sleep(Duration::from_millis(1));
			let frame_vec = get_frame(&mut capturer).to_vec();
			corner_pixels[0] = extract_pixel((square_index_x * SQUARE_SIZE_X) as usize, (square_index_y * SQUARE_SIZE_Y) as usize, offset_x, offset_y, screen_width, &frame_vec);

			MouseCursor::move_abs(((square_index_x + 1) * SQUARE_SIZE_X - 1 + offset_x) as i32, (square_index_y * SQUARE_SIZE_Y + offset_y) as i32);
			let frame_vec = get_frame(&mut capturer).to_vec();
			corner_pixels[1] = extract_pixel(((square_index_x + 1) * SQUARE_SIZE_X - 1) as usize, (square_index_y * SQUARE_SIZE_Y) as usize, offset_x, offset_y, screen_width, &frame_vec);

			MouseCursor::move_abs((square_index_x * SQUARE_SIZE_X + offset_x) as i32, ((square_index_y + 1) * SQUARE_SIZE_Y - 1 + offset_y) as i32);
			let frame_vec = get_frame(&mut capturer).to_vec();
			corner_pixels[2] = extract_pixel((square_index_x * SQUARE_SIZE_X) as usize, ((square_index_y + 1) * SQUARE_SIZE_Y - 1) as usize, offset_x, offset_y, screen_width, &frame_vec);

			MouseCursor::move_abs(((square_index_x + 1) * SQUARE_SIZE_X - 1 + offset_x) as i32, ((square_index_y + 1) * SQUARE_SIZE_Y - 1 + offset_y) as i32);
			let frame_vec = get_frame(&mut capturer).to_vec();
			corner_pixels[3] = extract_pixel(((square_index_x + 1) * SQUARE_SIZE_X - 1) as usize, ((square_index_y + 1) * SQUARE_SIZE_Y - 1) as usize, offset_x, offset_y, screen_width, &frame_vec);

			//Test for faster image manipulation as the loop below is rather slow
			// let mut image: dyn GenericImage;
			// let test: Rect = Rect {
			// 	x,
			// 	y,
			// 	width: 0,
			// 	height: 0
			// };
			// imga.copy_within(test, 0,0);

			MouseCursor::move_abs(current_position.0, current_position.1);
			if all_corners_same(&corner_pixels) {
				println!("All corners are the same, filling rectangle!");
				for pixel_x in 0..SQUARE_SIZE_X {
					for pixel_y in 0..SQUARE_SIZE_Y {
						img.put_pixel((square_index_x * SQUARE_SIZE_X + pixel_x), (square_index_y * SQUARE_SIZE_Y + pixel_y), corner_pixels[0]);
					}
				}
				continue;
			}
			println!("After rectangle");


			for pixel_y in 0..SQUARE_SIZE_Y {
				for pixel_x in 0..SQUARE_SIZE_X {
					let pos_x = square_index_x * SQUARE_SIZE_X + pixel_x;
					let pos_y = square_index_y * SQUARE_SIZE_Y + pixel_y;
					MouseCursor::move_abs((pos_x + offset_x) as i32, (pos_y + offset_y) as i32);

					let mut frame;
					loop {
						if let Ok(frame_new) = capturer.frame() {
							frame = frame_new;
							break;
						}
					};

					let frame_vec = frame.to_vec();
					let pixel = extract_pixel(pos_x as usize, pos_y as usize, offset_x, offset_y, screen_width, &frame_vec);
					img.put_pixel(pos_x, pos_y, pixel);
				}
				//MouseCursor::move_abs();
			}
			//MouseCursor::move_abs();
		}
		//MouseCursor::move_abs();
	}

	let stop = start.elapsed();

	img.save("capture.png").unwrap();

	// println!("The program took {}ms and captured with {} FPS/PPS.", stop.as_millis(), (DIM_X * DIM_Y) as f64 / stop.as_secs_f64());

	// let mut debug_frame;
	// loop {
	// 	if let Ok(frame_new) = capturer.frame() {
	// 		debug_frame = frame_new;
	// 		break;
	// 	}
	// };
	//
	// let debug_pixel = extract_pixel(1946, 793, 0, 0, 3840, &debug_frame.to_vec());
	// println!("Debug pixel is {:?}", debug_pixel);
}

// Deactivated for now, the borrow checker is too stoopid. Gotta copy paste it.
#[inline(always)]
fn get_frame(capturer: &mut Capturer) -> scrap::Frame {
	loop {
		if let Ok(frame) = capturer.frame() {
			let frame = unsafe { std::mem::transmute::<_, scrap::Frame<'static>>(frame) };
			return frame;
		}
	}
}

#[inline(always)]
fn all_corners_same(corner_pixels: &[Rgb<u8>; 4]) -> bool {
	corner_pixels[0] == corner_pixels[1] && corner_pixels[0] == corner_pixels[2] && corner_pixels[0] == corner_pixels[3]
}

#[inline(always)]
fn extract_pixel(x: usize, y: usize, offset_x: u32, offset_y: u32, display_width: usize, cap: &Vec<u8>) -> Rgb<u8> {
	let location = ((((y + offset_y as usize) * display_width) + (x + offset_x as usize)) * 4) as usize;
	let rgb = Rgb::from([cap[location + 2], cap[location + 1], cap[location]]);
	println!("Extracting pixel at location x: {} with y: {} offsets are x: {} y: {} delta_x: {} delta_y: {} and got color: {:?}", x, y, offset_x, offset_y, (x + offset_x as usize), (y + offset_y as usize), rgb);
	return rgb;
}