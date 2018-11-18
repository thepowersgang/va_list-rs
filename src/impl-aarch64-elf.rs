use std::{mem, ptr};
use super::VaPrimitive;

#[repr(C)]
pub struct VaList(*mut VaListInner);

#[repr(C)]
#[derive(Debug)]
#[doc(hidden)]
pub struct VaListInner {
    stack: *const u64,
    gr_top: *const u64,
    vr_top: *const u64,
    gr_offs: i32,
    vr_offs: i32,
}

impl VaList {
    fn inner(&mut self) -> &mut VaListInner {
        // This pointer should be valid
        unsafe { &mut *self.0 }
    }
}

impl VaListInner {
    pub unsafe fn get_gr<T>(&mut self) -> T {
        assert!(!self.gr_top.is_null());
        let rv = ptr::read((self.gr_top as usize - self.gr_offs.abs() as usize) as *const _);
        self.gr_offs += 8;
        rv
    }

    pub unsafe fn get_vr<T>(&mut self) -> T {
        assert!(!self.vr_top.is_null());
        let rv = ptr::read((self.vr_top as usize - self.vr_offs.abs() as usize) as *const _);
        self.vr_offs += 16;
        rv
    }
}

impl<T: 'static> VaPrimitive for *const T {
    unsafe fn get(list: &mut VaList) -> Self {
        <usize>::get(list) as *const T
    }
}

macro_rules! impl_va_prim_gr {
    ($u: ty, $s: ty) => {
        impl VaPrimitive for $u {
            unsafe fn get(list: &mut VaList) -> Self {
                list.inner().get_gr()
            }
        }
        impl VaPrimitive for $s {
            unsafe fn get(list: &mut VaList) -> Self {
                mem::transmute(<$u>::get(list))
            }
        }
    };
}

macro_rules! impl_va_prim_vr {
    ($($t:ty),+) => {
        $(
            impl VaPrimitive for $t {
                unsafe fn get(list: &mut VaList) -> Self {
                    list.inner().get_vr()
                }
            }
        )+
    };
}

impl_va_prim_gr!{ usize, isize }
impl_va_prim_gr!{ u64, i64 }
impl_va_prim_gr!{ u32, i32 }
impl_va_prim_vr!{ f64, f32 }
