use std::thread::sleep;
use std::time::Duration;
use image::{Rgb};
use scrap::Capturer;

#[inline(always)]
pub fn get_frame(capturer: &mut Capturer) -> scrap::Frame {
	loop {
		if let Ok(frame) = capturer.frame() {
			//Unsafe is needed due to borrow checker-bug
			return unsafe { std::mem::transmute::<_, scrap::Frame<'static>>(frame) };
		}
	}
}

#[inline(always)]
pub fn extract_pixel(x: usize, y: usize, offset_x: u32, offset_y: u32, display_width: usize, cap: &Vec<u8>) -> Rgb<u8> {
	let location = ((((y + offset_y as usize) * display_width) + (x + offset_x as usize)) * 4) as usize;
	return Rgb::from([cap[location + 2], cap[location + 1], cap[location]]);
	//println!("Extracting pixel at location x: {} with y: {} offsets are x: {} y: {} delta_x: {} delta_y: {} and got color: {:?}", x, y, offset_x, offset_y, (x + offset_x as usize), (y + offset_y as usize), rgb);
}

pub fn get_start_position() -> ((i32, i32), (i32, i32)) {
	let mut value: ((i32, i32), (i32, i32)) = ((0, 0), (0, 0));
	loop {
		if inputbot::MouseButton::LeftButton.is_pressed() {
			value.0 =  inputbot::MouseCursor::pos();
			println!("registered start at x = {} y = {}", value.0.0, value.0.1);
			break
		}
	}
	loop {
		if !inputbot::MouseButton::LeftButton.is_pressed() {
			value.1 =  inputbot::MouseCursor::pos();
			println!("registered stop at x = {} y = {}", value.1.0, value.1.1);
			break
		}
	}
	return value
}