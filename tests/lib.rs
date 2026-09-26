extern crate libc;
extern crate va_list;

#[link(name = "va_list_test", kind = "static")]
extern "C" {
    fn dispatch(context: *mut u8, count: ::libc::c_uint, ...);
}

type CbType<'a> = &'a mut dyn FnMut(u32, va_list::VaList);

#[no_mangle]
/// Method called by 'dispatch'
pub extern "C" fn inbound(context: *mut u8, count: u32, args: va_list::VaList) {
    let cb_ptr = unsafe { ::std::ptr::read(context as *mut CbType) };
    // call passed closure
    (cb_ptr)(count, args);
}

macro_rules! test_va_list {
	($int:expr, ($($args:expr),*), $code:expr) => ({
		let mut cb = $code;
		let mut cb_ref: CbType = &mut cb;

		unsafe {
			dispatch(&mut cb_ref as *mut _ as *mut u8, $int, $($args),*);
		}
	});
}

#[test]
fn trivial_values() {
    // Trivial test: Pass six random-ish sized integers
    // - The tester consumes two arguments (context and count), so this is a total of 8 arguments -
    //   enough to overflow x86_64-elf
    test_va_list!(
        4,
        (0xaabbaabbu32, 0xccddccddu32, 123456u32, 2u64, 1i32, -23i64),
        |_count, mut list: va_list::VaList| unsafe {
            assert_eq!(list.get::<u32>(), 0xaabbaabb);
            assert_eq!(list.get::<u32>(), 0xccddccdd);
            assert_eq!(list.get::<u32>(), 123456u32);
            let mut buf = va_list::VaListBuffer::new();
            let mut list2 = list.copy(&mut buf);
            assert_eq!(list.get::<u64>(), 2u64);
            assert_eq!(list.get::<i32>(), 1i32);
            assert_eq!(list.get::<i64>(), -23i64);

            assert_eq!(list2.get::<u64>(), 2u64);
            assert_eq!(list2.get::<i32>(), 1i32);
            assert_eq!(list2.get::<i64>(), -23i64);
        }
    );
    // Repeat the test
    test_va_list!(
        4,
        (0xaabbaabbu32, 0xccddccddu32, 123456u32, 2u64, 1i32, -23i64),
        |_count, mut list: va_list::VaList| unsafe {
            assert_eq!(list.get::<u32>(), 0xaabbaabb);
            assert_eq!(list.get::<u32>(), 0xccddccdd);
            assert_eq!(list.get::<u32>(), 123456u32);
            let mut buf = va_list::VaListBuffer::new();
            let mut list2 = list.copy(&mut buf);
            assert_eq!(list.get::<u64>(), 2u64);
            assert_eq!(list.get::<i32>(), 1i32);
            assert_eq!(list.get::<i64>(), -23i64);

            assert_eq!(list2.get::<u64>(), 2u64);
            assert_eq!(list2.get::<i32>(), 1i32);
            assert_eq!(list2.get::<i64>(), -23i64);
        }
    );
}

#[test]
fn floating_point() {
    test_va_list!(
        4,
        (123456f64, 0.1f64),
        |_count, mut list: va_list::VaList| unsafe {
            assert_eq!(list.get::<f64>(), 123456f64);
            assert_eq!(list.get::<f64>(), 0.1f64);
        }
    );
}

#[test]
fn mixed_float_int() {
    test_va_list!(
        4,
        (
            0xaabbaabbu32,
            123456f64,
            0xccddccddu32,
            0.1f64,
            42i64,
            2.5f64
        ),
        |_count, mut list: va_list::VaList| unsafe {
            assert_eq!(list.get::<u32>(), 0xaabbaabb);
            assert_eq!(list.get::<f64>(), 123456f64);
            assert_eq!(list.get::<u32>(), 0xccddccdd);
            assert_eq!(list.get::<f64>(), 0.1f64);
            assert_eq!(list.get::<i64>(), 42i64);
            assert_eq!(list.get::<f64>(), 2.5f64);
        }
    );
}

#[test]
fn mixed_float_int_overflow() {
    test_va_list!(
        4,
        (
            1u32,
            1.5f64,
            2u32,
            2.5f64,
            3u32,
            3.5f64,
            4u32,
            4.5f64,
            5u32,
            5.5f64,
            6u32,
            6.5f64,
            7u32,
            7.5f64,
            8u32,
            8.5f64,
            9u32,
            9.5f64,
            10u32,
            10.5f64
        ),
        |_count, mut list: va_list::VaList| unsafe {
            assert_eq!(list.get::<u32>(), 1);
            assert_eq!(list.get::<f64>(), 1.5f64);
            assert_eq!(list.get::<u32>(), 2);
            assert_eq!(list.get::<f64>(), 2.5f64);
            assert_eq!(list.get::<u32>(), 3);
            assert_eq!(list.get::<f64>(), 3.5f64);
            assert_eq!(list.get::<u32>(), 4);
            assert_eq!(list.get::<f64>(), 4.5f64);
            assert_eq!(list.get::<u32>(), 5);
            assert_eq!(list.get::<f64>(), 5.5f64);

            assert_eq!(list.get::<u32>(), 6);
            assert_eq!(list.get::<f64>(), 6.5f64);
            assert_eq!(list.get::<u32>(), 7);
            assert_eq!(list.get::<f64>(), 7.5f64);
            assert_eq!(list.get::<u32>(), 8);
            assert_eq!(list.get::<f64>(), 8.5f64);
            assert_eq!(list.get::<u32>(), 9);
            assert_eq!(list.get::<f64>(), 9.5f64);
            assert_eq!(list.get::<u32>(), 10);
            assert_eq!(list.get::<f64>(), 10.5f64);
        }
    );
}

#[test]
fn mixed_float_int_with_copy() {
    test_va_list!(
        4,
        (7u32, 1.25f64, -3i32, -2.75f64),
        |_count, mut list: va_list::VaList| unsafe {
            assert_eq!(list.get::<u32>(), 7);
            assert_eq!(list.get::<f64>(), 1.25f64);

            let mut buf = va_list::VaListBuffer::new();
            let mut list2 = list.copy(&mut buf);

            assert_eq!(list.get::<i32>(), -3i32);
            assert_eq!(list.get::<f64>(), -2.75f64);

            assert_eq!(list2.get::<i32>(), -3i32);
            assert_eq!(list2.get::<f64>(), -2.75f64);
        }
    );
}
