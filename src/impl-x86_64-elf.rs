// x86_64 ELF - Aka the Itanium ABI
//
use std::{mem,ptr};
use super::VaPrimitive;

pub struct VaList(*mut VaListInner);
// /// Saves the state of the VaList, similar to va_copy
//impl Clone for VaList { fn clone(&self) -> Self { va_list(self.0) } }

#[repr(C)]
#[derive(Debug)]
#[doc(hidden)]
pub struct VaListInner
{
	gp_offset: u32,
	fp_offset: u32,
	overflow_arg_area: *const (),
	reg_save_area: *const (),
}

impl VaList
{
	fn inner(&mut self) -> &mut VaListInner {
		// This pointer should be valid
		unsafe { &mut *self.0 }
	}
}

#[doc(hidden)]
impl VaListInner
{
	/// Expected that either num_gp or num_fp will be 0
	fn check_space(&self, num_gp: u32, num_fp: u32) -> bool {
		!(self.gp_offset > 48 - num_gp || self.fp_offset > 304 - num_fp)
	}
	/// Fetch an integer value of the gp register save area
	///
	/// Need to have checked that the type requested will fit in what remains of the stack
	unsafe fn get_gp<T>(&mut self) -> T {
		let rv = ptr::read( (self.reg_save_area as usize + self.gp_offset as usize) as *const _ );
		self.gp_offset += ((mem::size_of::<T>() + 7) & !7) as u32; // align up to 8 bits
		rv
	}

	/// Fetch a float value of the fp register save area
	///
	/// Need to have checked that the type requested will fit in what remains of the stack
	unsafe fn get_fp<T>(&mut self) -> T {
		let rv = ptr::read( (self.reg_save_area as usize + self.fp_offset as usize) as *const _ );
		self.fp_offset += ((mem::size_of::<T>() + 15) & !15) as u32; // align up to 16 bits
		rv
	}

	/// Fetch a value of the overflow save area
	unsafe fn get_overflow<T>(&mut self) -> T {
		let align = mem::align_of::<T>();
		// 7. Align overflow_reg_area upwards to a 16-byte boundary if alignment
		//    needed by T exceeds 8 bytes
		//
		// This also covers 10., but does it before the next iteration, rather than after the
		// previous
		let addr = self.overflow_arg_area as usize;
		if align > 8 {
			if addr % 16 != 0 {
				self.overflow_arg_area = ((addr + 15) & !(16-1)) as *const _;
			}
		}
		else {
			if addr % 8 != 0 {
				self.overflow_arg_area = ((addr + 7) & !(8-1)) as *const _;
			}
		}
		// 8. Fetch from overflow array
		let rv = ptr::read( self.overflow_arg_area as *const _ );
		self.overflow_arg_area = ((self.overflow_arg_area as usize) + mem::size_of::<T>()) as *const _;
		rv
	}
}

// pointers are 8 bytes wide (so re-use usize)
impl<T: 'static> VaPrimitive for *const T
{
	unsafe fn get(list: &mut VaList) -> Self {
		<usize>::get(list) as *const T
	}
}

macro_rules! impl_va_int {
	($u:ty, $s:ty) => {
		impl VaPrimitive for $u {
			unsafe fn get(list: &mut VaList) -> Self {
				let inner = list.inner();
				// See the ELF AMD64 ABI document for a description of how this should act
				if ! inner.check_space(mem::size_of::<$u>() as u32, 0) {
					inner.get_overflow()
				}
				else {
					inner.get_gp()
				}
			}
		}
		impl VaPrimitive for $s {
			unsafe fn get(list: &mut VaList) -> Self {
				mem::transmute( <$u>::get(list) )
			}
		}
	};
}

impl_va_int!{ usize, isize }
//impl_va_int!{ u128, i128 }
impl_va_int!{ u64, i64 }
impl_va_int!{ u32, i32 }
impl_va_int!{ u16, i16 }
impl_va_int!{ u8, i8 }

macro_rules! impl_va_float {
	($f:ty) => {
		impl VaPrimitive for $f {
			unsafe fn get(list: &mut VaList) -> Self {
				let inner = list.inner();
				// See the ELF AMD64 ABI document for a description of how this should act
				if ! inner.check_space(0, mem::size_of::<$f>() as u32) {
					inner.get_overflow()
				}
				else {
					inner.get_fp()
				}
			}
		}
	};
}

impl_va_float!{ f32 }
impl_va_float!{ f64 }
