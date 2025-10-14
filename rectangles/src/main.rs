fn straigt_forward_approach() {
	let width1 = 30;
	let height1 = 50;

	println!(
		"The area of the rectangle is {}",
		sfa_area(width1,height1)
		);
}

fn sfa_area(width: u32, height: u32) -> u32 {
	width * height
}

fn tuple_approach(){
	let rect1 = (30, 50);
	println!("The area of the rectange is {}",
		ta_area(rect1));
}

fn ta_area(dimensions: (u32, u32)) -> u32 {
	dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle{
	height: u32,
	width: u32,
}

impl Rectangle {
	// fn area(&self) -> u32 {
	fn area(self: &Self) -> u32 {
		self.width * self.height
	}
	fn width(&self) -> bool {
		self.width > 0
	}
}

fn main() {
	
	let scale = 2;
	let rect1 = Rectangle {
		width: dbg!(30 * scale),
		height: 50
		};
	// println!("The area of the rectangle is {}",area(&rect1));
	// println!("The rectange is {:#?}", rect1);
	// dbg!(rect1);
	println!("The area of the rectangle is {}", rect1.area());
	if rect1.width() {
		println!("The non-zero width of rectangle is: {}",rect1.width);
	}

}

fn area(rect: &Rectangle) -> u32 {
	rect.width * rect.height
}
