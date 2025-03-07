use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

trait Shape {
	fn area(&self) -> f32;
}

struct Square {
	width: u32,
}

impl Shape for Square {
	fn area(&self) -> f32 { (self.width * self.width) as f32 }
}

async fn calculate_area(shape: &dyn Shape) -> f32 {
	return shape.area();
}


fn block_on<F: Future + ?Sized>(future: &mut F) -> F::Output {
	let waker = futures::task::noop_waker();
	let mut context = Context::from_waker(&waker);
	let mut future = unsafe { Pin::new_unchecked(future) };

	loop {
		if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
			return value;
		}
		std::thread::yield_now();
	}
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
	unsafe extern "C" fn rust_shape_area(shape: *mut Shape) -> f32 {
		shape.as_ref().unwrap().inner.area()
	}


	struct Future<T> {
		inner: Box<dyn std::future::Future<Output=T>>,
	}

	#[no_mangle]
	unsafe extern "C" fn rust_await_f32(future: *mut Future<f32>) -> f32 {
		super::block_on(Box::from_raw(future).inner.as_mut())
	}

	#[no_mangle]
	unsafe extern "C" fn rust_shape_calculate_area(shape: *mut Shape) -> *mut Future<f32> {
		use std::ops::Deref;

		let shape = shape.as_mut().unwrap();
		let future = super::calculate_area(shape.inner.deref());
		Box::into_raw(Box::new(Future { inner: Box::new(future) }))
	}
}
