fn screenshot() {
	// sleep(Duration::from_millis(3000));
	let mut capturer = Capturer::new(0).unwrap();
	sleep(Duration::from_millis(100));
	let mut ps = capturer.capture_frame().unwrap();
	println!("Captured");


	let mut img: RgbImage = ImageBuffer::new(1920, 1080);
	for horizontal in 0..1919 {
		for vertical in 0..1079 {
			img.put_pixel(horizontal, vertical, Rgb::from([ps[(vertical * 1920 + horizontal) as usize].r, ps[(vertical * 1920 + horizontal) as usize].g, ps[(vertical * 1920 + horizontal) as usize].b]));
		}
	}
	img.save("yeet.png").unwrap();
}