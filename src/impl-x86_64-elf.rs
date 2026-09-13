// x86_64 ELF - Aka the Itanium ABI
//
use ::core::{mem, ptr};
use super::VaPrimitive;	// Note: Uses `super` for testing purposes

#[repr(transparent)]
pub struct VaList<'a>(&'a mut VaListInner);

pub struct VaListBuffer(mem::MaybeUninit<VaListInner>);

#[repr(C)]
#[derive(Debug)]
#[doc(hidden)]
pub struct VaListInner {
    /// Offset of general-purpose registers in `reg_save_area`
    gp_offset: u32,
    /// Offset of floating-point registers in `reg_save_area`
    fp_offset: u32,
    /// Pointer to the on-stack arguments
    overflow_arg_area: *const u64,
    /// Save area for register arguments
    reg_save_area: *const u64,
}

impl<'a> VaList<'a> {
    fn inner(&mut self) -> &mut VaListInner {
        &mut *self.0
    }
    pub(crate) fn copy<'b>(&self, buffer: &'b mut VaListBuffer) -> VaList<'b>
        where 'a: 'b
    {
        VaList(buffer.0.write(VaListInner {
            gp_offset: self.0.gp_offset,
            fp_offset: self.0.fp_offset,
            overflow_arg_area: self.0.overflow_arg_area,
            reg_save_area: self.0.reg_save_area,
        }))
    }
}

impl VaListBuffer {
    pub(crate) fn new() -> Self {
        Self(mem::MaybeUninit::uninit())
    }
}

#[doc(hidden)]
impl VaListInner {
    /// Checks that the specified number of registers can be read from the save area
    fn check_space_gp(&self, num_gp: u32) -> bool {
        self.gp_offset / 8 + num_gp <= 6
    }
    /// Checks that the specified number of registers can be read from the save area
    fn check_space_fp(&self, num_fp: u32) -> bool {
        self.fp_offset + num_fp * 16 <= 304
    }

    /// Read an argument from a general-purpose register
    unsafe fn get_gp<T>(&mut self) -> T {
        let n_gp = (mem::size_of::<T>() + 7) / 8;
        assert!(self.check_space_gp(n_gp as u32));
        let rv = ptr::read(self.reg_save_area.offset(self.gp_offset as isize / 8) as *const _);
        self.gp_offset += (8 * n_gp) as u32;
        rv
    }

    /// Read an argument from the overflow region
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

macro_rules! impl_va_prim_gp {
    ($u: ty, $s: ty) => {
        impl VaPrimitive for $u {
            unsafe fn get(list: &mut VaList) -> Self {
                let inner = list.inner();
                // See the ELF AMD64 ABI document for a description of how this should act
                if !inner.check_space_gp(1) {
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

impl_va_prim_gp!{ usize, isize }
impl_va_prim_gp!{ u64, i64 }
impl_va_prim_gp!{ u32, i32 }
//impl_va_prim!{ u16, i16 }
//impl_va_prim!{ u8, i8 }

macro_rules! impl_va_prim_fp {
    ($t: ty) => {
        impl VaPrimitive for $t {
            unsafe fn get(list: &mut VaList) -> Self {
                let inner = list.inner();
                // See the ELF AMD64 ABI document for a description of how this should act
                if !inner.check_space_fp(1) {
                    inner.get_overflow()
                } else {
                    inner.get_gp()
                }
            }
        }
    }
}
impl_va_prim_fp!{ f32 }
impl_va_prim_fp!{ f64 }

