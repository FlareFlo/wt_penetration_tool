use std::thread::sleep;

use enigo::{Enigo, MouseControllable, MouseButton};
use core::time;
use rand::Rng;

fn main() {
	let mut enigo = Enigo::new();

	sleep(time::Duration::from_secs(3));

	let target_y = 10+1;
	let target_x = 10+1;

	for height in 1..100 {
		// for width in 1..100 {
		// 	enigo.mouse_move_relative(1, 0);
		// 	sleep(time::Duration::from_millis(1));
		// }
		sleep(time::Duration::from_millis(10));
		enigo.mouse_move_relative(0,2);
	}
}
