
use super::imp;

/// Rust version of C's `va_list` type from the `stdarg.h` header
#[repr(transparent)]
pub struct VaList<'a> {
    internal: imp::VaList<'a>,
}

/// Core type as passed though the FFI
impl<'a> VaList<'a> {
    /// Read a value from the VaList.
    ///
    /// Users should take care that they are reading the correct type
    pub unsafe fn get<T: VaPrimitive>(&mut self) -> T {
        T::get(&mut self.internal)
    }
}

/// Trait implemented on types that can be read from a va_list
pub trait VaPrimitive: 'static {
    #[doc(hidden)]
    unsafe fn get(_: &mut imp::VaList) -> Self;
}

#[allow(dead_code)]
mod check_core_types {
	struct Foo<T: super::VaPrimitive>([T;0]);

	struct Checks {
		_ptr: Foo<*const u8>,
		_usize: Foo<usize>,
		_isize: Foo<isize>,
	}
}

