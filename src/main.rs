use std::thread::sleep;

use inputbot::{KeybdKey::*, MouseButton::*, *};
use core::time;
use rand::Rng;

fn main() {

	sleep(time::Duration::from_secs(3));

	for height in 1..=500 {
		for width in 1..500 {
			MouseCursor::move_rel(1,0);
		}
		MouseCursor::move_rel(-500,1);
	}
}
