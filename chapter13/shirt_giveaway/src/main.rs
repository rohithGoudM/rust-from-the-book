#[derive(Debug)]
enum ShirtColor {
	Red,
	Blue
}

struct Inventory {
	shirts: Vec<ShirtColor>
}

impl Inventory {
	fn giveaway (&self, user_preference: Option<ShirtColor>) -> ShirtColor {
		user_preference.unwrap_or_else(|| self.mock_stocked())
	}
	
	fn mock_stocked(&self) -> ShirtColor {
		let mut num_red = 0;
		let mut num_blue = 0;

		for shirt in &self.shirts {
			match shirt {
				ShirtColor::Red => num_red += 1,
				ShirtColor::Blue => num_blue += 1,
			}
		}
		if num_red > num_blue {
			ShirtColor::Red
		} else {
			ShirtColor::Blue
		}
	}
}


fn main() {
	let shirt_vec: Vec<ShirtColor> = vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red];
	let store = Inventory { shirts: shirt_vec };
	let giveaway_color = store.giveaway(Some(ShirtColor::Blue));
	let giveaway_color2 = store.giveaway(None);
	println!("{:?}, {:?}",giveaway_color, giveaway_color2);
}
