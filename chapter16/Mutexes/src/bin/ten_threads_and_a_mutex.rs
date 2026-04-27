use std::sync::{Arc,Mutex};
use std::time::Duration;
use std::thread;

fn main(){
	let counter = Arc::new(Mutex::new(0));
	let mut handles = vec![];

	for i in 1..11 {
		let counter = Arc::clone(&counter);
		let handle = thread::spawn(move || {
			// thread::sleep(Duration::from_secs(1));
			println!("{}: {}",i,Arc::strong_count(&counter));
			// thread::sleep(Duration::from_secs(1));
			let mut num = counter.lock().unwrap();

			*num += 1;
		});
		handles.push(handle);
	}

	println!("main: {}",Arc::strong_count(&counter));

	for handle in handles {
		handle.join().unwrap();
	}

	println!("{}", *counter.lock().unwrap());
}
