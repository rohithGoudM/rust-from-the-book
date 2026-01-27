use std::cell::RefCell;
use std::rc::{Rc,Weak};

#[derive(Debug)]
struct Node {
	value: i32,
	parent: RefCell<Weak<Node>>,
	children: RefCell<Vec<Rc<Node>>>,
}

fn main(){
	let leaf = Node {
		value: 5,
		parent: RefCell::new(Weak::new()),
		children: RefCell::new(vec![]),
	};
	let leaf = Rc::new(leaf);
	
	println!(
		"leaf's strong: {} and weak: {} counts",
		Rc::strong_count(&leaf),
		Rc::weak_count(&leaf),
	);

	{
		let branch = Node {
			value: 10,
			parent: RefCell::new(Weak::new()),
			children: RefCell::new(vec![Rc::clone(&leaf)]),
		};
		let branch = Rc::new(branch);

		*leaf.parent.borrow_mut() = Rc::downgrade(&branch);

		println!(
			"branch's strong: {} and weak: {} counts",
			Rc::strong_count(&branch),
			Rc::weak_count(&branch),
		);
		println!(
			"leaf's strong: {} and weak: {} counts",
			Rc::strong_count(&leaf),
			Rc::weak_count(&leaf),
		);
	}

	println!(
		"leaf's strong: {} and weak: {} counts",
		Rc::strong_count(&leaf),
		Rc::weak_count(&leaf),
	);

	println!("leaf's parent: {:?}", leaf.parent.borrow().upgrade());
}
