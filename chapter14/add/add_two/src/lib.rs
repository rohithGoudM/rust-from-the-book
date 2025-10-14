pub fn add_two(x: i32) -> i32 {
	x + 2
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn adding_two() {
		assert_eq!(3, add_two(1));
	}
}
