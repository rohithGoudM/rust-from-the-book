use trpl::StreamExt;

fn main() {
	trpl::block_on(async {
		let nums = vec![1,2,3,4,5,6,7,8,9,10];
		let iter = nums.iter().map(|n| n*n);
		let mut stream = trpl::stream_from_iter(iter);

		
		while let Some(val) = stream.next().await {
			println!("{val}");
		}
	});
}
