use crate::capture_patterns::full_capture::full_capture;
use crate::capture_patterns::interpolated_capture::interpolated_capture;

mod capture_patterns;
mod resource_functions;

//X Dimensions of the scan square
const SQUARE_SIZE_X: u32 = 10;
const SQUARE_SIZE_Y: u32 = 10; //Y Dimensions of the scan square

const NUM_SQUARES_X: u32 = 2;
const NUM_SQUARES_Y: u32 = 2;

fn main() {
	full_capture(SQUARE_SIZE_X, SQUARE_SIZE_Y, 17 * 3);
	// interpolated_capture(SQUARE_SIZE_X, SQUARE_SIZE_Y, NUM_SQUARES_X, NUM_SQUARES_Y, 17 * 3);
}

// Grey = 128 128 128
// Red = 255 75 56
// Yellow = 255 244 0
// Green = 32 240 32