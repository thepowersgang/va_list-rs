/*
 *
 */
use std::{mem, ptr};
use super::VaPrimitive;

#[allow(non_camel_case_types)]
pub struct VaList(*const u8);

impl VaList {
    // Read a raw value from the list
    unsafe fn get_raw<T: 'static>(&mut self) -> T {
        assert_eq!(self.0 as usize % mem::align_of::<T>(), 0);
        let rv = ptr::read(self.0 as *const T);
        self.0 = self.0.offset(mem::size_of::<T>() as isize);
        rv
    }
}

impl<T: 'static> VaPrimitive for *const T {
    unsafe fn get(list: &mut VaList) -> Self {
        <usize>::get(list) as *const T
    }
}
impl VaPrimitive for usize {
    unsafe fn get(l: &mut VaList) -> Self {
        l.get_raw()
    }
}
impl VaPrimitive for isize {
    unsafe fn get(l: &mut VaList) -> Self {
        l.get_raw()
    }
}
impl VaPrimitive for u64 {
    unsafe fn get(l: &mut VaList) -> Self {
        l.get_raw()
    }
}
impl VaPrimitive for i64 {
    unsafe fn get(l: &mut VaList) -> Self {
        l.get_raw()
    }
}
impl VaPrimitive for u32 {
    unsafe fn get(l: &mut VaList) -> Self {
        l.get_raw::<u64>() as u32
    }
}
impl VaPrimitive for i32 {
    unsafe fn get(l: &mut VaList) -> Self {
        l.get_raw::<i64>() as i32
    }
}
//impl VaPrimitive for u16 { unsafe fn get(l: &mut VaList) -> Self { l.get_raw() } }
//impl VaPrimitive for i16 { unsafe fn get(l: &mut VaList) -> Self { l.get_raw() } }
//impl VaPrimitive for u8 { unsafe fn get(l: &mut VaList) -> Self { l.get_raw() } }
//impl VaPrimitive for i8 { unsafe fn get(l: &mut VaList) -> Self { l.get_raw() } }

impl VaPrimitive for f64 {
    unsafe fn get(l: &mut VaList) -> Self {
        l.get_raw()
    }
}
