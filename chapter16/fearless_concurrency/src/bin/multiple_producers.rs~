use std::time::Duration;
use std::thread;
use std::sync::mpsc;

fn main () {
	let v = vec![
		String::from("Hi"),
		String::from("How"),
		String::from("are"),
		String::from("you")
	];
	let v1 = vec![
		String::from("When"),
		String::from("were"),
		String::from("we"),
		String::from("here"),
	];

	let (tx, rx) = mpsc::channel();
	let tx1 	 = tx.clone();

	thread::spawn(move || {
		for val in v1 {
			tx1.send(val).unwrap();
			thread::sleep(Duration::from_millis(200));
		}			
	});
	
	thread::spawn(move || {
		for val in v {
			tx.send(val).unwrap();
			thread::sleep(Duration::from_millis(300));
		}			
	});
	
	for received in rx {
		println!("received: {}",received);
	}
}
