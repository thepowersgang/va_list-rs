// TODO: This is identical to the x86-sysv code
/*
 *
 */
use std::{mem, ptr};
use super::VaPrimitive;

#[repr(transparent)]
pub struct VaList<'a>(*const u8, ::std::marker::PhantomData<&'a [usize]>);

impl<'a> VaList<'a> {
    // Read a raw value from the list
    unsafe fn get_raw<T: 'static>(&mut self) -> T {
        assert_eq!(self.0 as usize % mem::align_of::<T>(), 0);
        let rv = ptr::read(self.0 as *const T);
        self.0 = self.0.offset(mem::size_of::<T>() as isize);
        rv
    }
}

impl<T: 'static> VaPrimitive for *const T {
    unsafe fn get(l: &mut VaList) -> Self {
		l.get_raw()
    }
}
macro_rules! impl_va_prim {
	( $( $t:ty, )+ ) => {
		$(
			impl VaPrimitive for $t {
				unsafe fn get(l: &mut VaList) -> Self {
					l.get_raw()
				}
			}
		)+
	};
}
impl_va_prim!{
	usize, isize,
	u64, i64,
	u32, i32,
	// No 16-bit/8-bit values - not valid
	f64,
	// TODO: is f32 valid?
}

