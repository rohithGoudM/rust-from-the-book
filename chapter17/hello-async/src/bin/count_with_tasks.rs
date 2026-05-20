use std::time::Duration;

fn main () {
	trpl::run(async {
		let fut_1 = async {
			for i in 1..10 {
				println!("third task is at {i}");
				trpl::sleep(Duration::from_millis(500)).await;
			}
		};
		// let fut_2 = async {
			for i in 1..10 {
				println!("fourth task is at {i}");
				trpl::sleep(Duration::from_millis(500)).await;
		//	}
		};
		fut_1.await;
		// trpl::join(fut_1, fut_2).await;
	});
}

fn start_runtime() {
	trpl::run(async {
		let handle = trpl::spawn_task(async {
			for i in 1..10 {
				println!("first task is at {i}");
				trpl::sleep(Duration::from_millis(500)).await;
			}
		});
		
		for i in 1..5 {
			println!("second task is at {i}");
			trpl::sleep(Duration::from_millis(500)).await;
		}
		handle.await.unwrap();
	});
}
