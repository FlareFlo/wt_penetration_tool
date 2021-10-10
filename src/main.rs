use crate::capture_patterns::full_capture::full_capture;
// use crate::capture_patterns::interpolated_capture::interpolated_capture;
use crate::capture_patterns::skip_sample::skip_sample_capture;
use crate::resource_functions::get_start_position;

mod capture_patterns;
mod resource_functions;

//X Dimensions of the scan square
const SQUARE_SIZE_X: u32 = 650;
const SQUARE_SIZE_Y: u32 = 650; //Y Dimensions of the scan square

const NUM_SQUARES_X: u32 = 2;
const NUM_SQUARES_Y: u32 = 2;

fn main() {
	let start = get_start_position();
	skip_sample_capture((start.1.0 - start.0.0) as u32, (start.1.1 - start.0.1) as u32, start.0, 0, 5, 5);
}

// Grey = 128 128 128
// Red = 255 75 56
// Yellow = 255 244 0
// Green = 32 240 32