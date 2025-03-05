trait Shape {
	fn area(&self) -> u32;
}

struct Square {
	width: u32,
}

impl Square {
	fn new(width: u32) -> Square {
		Square { width }
	}
}

impl Shape for Square {
	fn area(&self) -> u32 { self.width * self.width }
}

mod sys {
	#[no_mangle]
	extern "C" fn rust_test() {
		println!("Hello, Rust!");
	}

	struct Shape {
		inner: Box<dyn crate::Shape>
	}

	#[no_mangle]
	extern "C" fn rust_shape_new_square(width: u32) -> *mut Shape {
		Box::into_raw(Box::new(Shape { inner: Box::new(super::Square { width }) }))
	}

	#[no_mangle]
	unsafe extern "C" fn rust_shape_free(square: *mut Shape) {
		let square = Box::from_raw(square);
		drop(square);
	}

	#[no_mangle]
	unsafe extern "C" fn rust_shape_area(shape: *mut Shape) -> u32 {
		shape.as_ref().unwrap().inner.area()
	}
}
