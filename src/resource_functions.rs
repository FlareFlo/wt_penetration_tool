use image::{Rgb};
use scrap::Capturer;

#[inline(always)]
pub fn get_frame(capturer: &mut Capturer) -> scrap::Frame {
	loop {
		if let Ok(frame) = capturer.frame() {
			//Unsafe is needed due to borrow checker-bug
			let frame = unsafe { std::mem::transmute::<_, scrap::Frame<'static>>(frame) };
			return frame;
		}
	}
}

#[inline(always)]
pub fn extract_pixel(x: usize, y: usize, offset_x: u32, offset_y: u32, display_width: usize, cap: &Vec<u8>) -> Rgb<u8> {
	let location = ((((y + offset_y as usize) * display_width) + (x + offset_x as usize)) * 4) as usize;
	let rgb = Rgb::from([cap[location + 2], cap[location + 1], cap[location]]);
	//println!("Extracting pixel at location x: {} with y: {} offsets are x: {} y: {} delta_x: {} delta_y: {} and got color: {:?}", x, y, offset_x, offset_y, (x + offset_x as usize), (y + offset_y as usize), rgb);
	return rgb;
}