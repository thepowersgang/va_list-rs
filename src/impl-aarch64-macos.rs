use std::ffi::c_void;
use super::VaPrimitive;

#[repr(transparent)]
pub struct VaList<'a> {
    ptr: *mut c_void,
	pd: ::std::marker::PhantomData<&'a [usize]>,
}

impl<'a> VaList<'a> {
    unsafe fn get_raw<T>(&mut self) -> T {
        let res = std::ptr::read(self.ptr as _);
        self.ptr = self.ptr.add(8);
        res
    }
}

impl<T: 'static> VaPrimitive for *const T {
    unsafe fn get(list: &mut VaList) -> Self {
        list.get_raw()
    }
}

macro_rules! impl_va_prim {
    ($($t:ty),+) => {
        $(
            impl VaPrimitive for $t {
                unsafe fn get(list: &mut VaList) -> Self {
                    list.get_raw()
                }
            }
        )+
    };
}

impl_va_prim!{ usize, isize, u64, i64, u32, i32, f64, f32 }

