/*
 * "cdecl32"
 * Standard stack-based calling convention, with 32-bit alignment
 */
use ::core::mem;
use super::VaPrimitive;

const ALIGN: usize = 4;

#[repr(transparent)]
pub struct VaList<'a>(*const u8, ::core::marker::PhantomData<&'a [u32]>);

impl<'a> VaList<'a> {
    // Read a raw value from the list
	// UNSAFE: Doesn't check that the value is POD
    unsafe fn get_raw<T: 'static>(&mut self) -> T {
		// TODO: Advance until type's alignment is met?
        assert_eq!(self.0 as usize % mem::align_of::<T>(), 0);
        let rv = ::core::ptr::read(self.0 as *const T);
		// Allow reading under-sized values (always advance by a multiple of 32-bits)
		let slots = (mem::size_of::<T>() + (ALIGN-1)) / ALIGN;
        self.0 = self.0.offset( (slots * ALIGN) as isize );
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

