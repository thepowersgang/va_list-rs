extern crate va_list;

extern "C" {
	fn dispatch(context: *mut (), count: u32, ...);
}

type CbType<'a> = &'a mut FnMut(u32, va_list::va_list);

#[no_mangle]
/// Method called by 'dispatch'
pub extern "C" fn inbound(context: *mut (), count: u32, args: va_list::va_list) {
	let cb_ptr = unsafe { ::std::ptr::read(context as *mut CbType ) };
	// call passed closure
	(cb_ptr)(count, args);
}

macro_rules! test_va_list {
	($int:expr, ($($args:expr),*), $code:expr) => ({
		let mut cb = $code;
		let mut cb_ref: CbType = &mut cb;
		
		unsafe {
			dispatch(&mut cb_ref as *mut _ as *mut (), $int, $($args),*);
		}
	});
}

#[test]
fn trivial_values() {
	// Trivial test: Pass four random-ish sized integers
	test_va_list!(4, (123456u32, 2u64, 1i32, -23i64),
		|_count, mut list: va_list::va_list| { unsafe {
			assert_eq!( list.get::<u32>(), 123456u32 );
			assert_eq!( list.get::<u64>(), 2u64 );
			assert_eq!( list.get::<i32>(), 1i32 );
			assert_eq!( list.get::<i64>(), -23i64 );
		} });
}
