// x86_64 ELF - Aka the Itanium ABI
//
use ::core::{mem, ptr};
use ::core::ffi::c_void;
use super::VaPrimitive;	// Note: Uses `super` for testing purposes

#[repr(transparent)]
pub struct VaList<'a>(&'a mut VaListInner);

#[repr(C)]
#[derive(Debug)]
#[doc(hidden)]
pub struct VaListInner {
    gp_offset: u32,
    fp_offset: u32,
    overflow_arg_area: *const c_void,
    reg_save_area: *const c_void,
}

impl<'a> VaList<'a> {
    fn inner(&mut self) -> &mut VaListInner {
        &mut *self.0
    }
}

#[doc(hidden)]
impl VaListInner {
    fn check_space(&self, num_gp: u32, num_fp: u32) -> bool {
        !(self.gp_offset > 48 - num_gp * 8 || self.fp_offset > 304 - num_fp * 16)
    }

    unsafe fn get_gp<T>(&mut self) -> T {
        let n_gp = (mem::size_of::<T>() + 7) / 8;
        assert!(self.check_space(n_gp as u32, 0));
        let rv = ptr::read((self.reg_save_area as usize + self.gp_offset as usize) as *const _);
        self.gp_offset += (8 * n_gp) as u32;
        rv
    }

    unsafe fn get_overflow<T>(&mut self) -> T {
        let align = mem::align_of::<T>();
        // 7. Align overflow_reg_area upwards to a 16-byte boundary if alignment
        //    needed by T exceeds 8 bytes
        let addr = self.overflow_arg_area as usize;
        if align > 8 {
            if addr % 16 != 0 {
                self.overflow_arg_area = ((addr + 15) & !(16 - 1)) as *const _;
            }
        } else {
            if addr % 8 != 0 {
                self.overflow_arg_area = ((addr + 7) & !(8 - 1)) as *const _;
            }
        }
        // 8. Fetch from overflow areay
        let rv = ptr::read(self.overflow_arg_area as *const _);
        self.overflow_arg_area =
            ((self.overflow_arg_area as usize) + mem::size_of::<T>()) as *const _;
        rv
    }
}


impl<T: 'static> VaPrimitive for *const T {
    unsafe fn get(list: &mut VaList) -> Self {
        <usize>::get(list) as *const T
    }
}

macro_rules! impl_va_prim {
    ($u: ty, $s: ty) => {
        impl VaPrimitive for $u {
            unsafe fn get(list: &mut VaList) -> Self {
                let inner = list.inner();
                // See the ELF AMD64 ABI document for a description of how this should act
                if !inner.check_space(1, 0) {
                    inner.get_overflow()
                } else {
                    inner.get_gp()
                }
            }
        }
        impl VaPrimitive for $s {
            unsafe fn get(list: &mut VaList) -> Self {
                mem::transmute(<$u>::get(list))
            }
        }
    };
}

impl_va_prim!{ usize, isize }
impl_va_prim!{ u64, i64 }
impl_va_prim!{ u32, i32 }
//impl_va_prim!{ u16, i16 }
//impl_va_prim!{ u8, i8 }

