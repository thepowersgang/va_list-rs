/*!
 * C FFI - va_list support
 *
 * This crate provides an interface for rust code to read values passed in C's va_list type.
 */
#![cfg_attr(nightly,feature(no_std,core))]
#![cfg_attr(nightly,no_std)]
#![crate_type="lib"]
#![crate_name="va_list"]

#[cfg(nightly)] use core::prelude::*;
#[cfg(nightly)] use core::{mem,ptr};
#[cfg(not(nightly))] use std::{mem,ptr};

#[cfg(nightly)]
#[macro_use]
extern crate core;

#[allow(non_camel_case_types)]
/// Core type as passed though the FFI
pub struct va_list(*mut VaListInner);
/// Saves the state of the va_list, similar to va_copy
impl Copy for va_list {}
impl Clone for va_list { fn clone(&self) -> Self { *self } }

#[repr(C)]
#[derive(Debug)]
#[allow(raw_pointer_derive)]
#[doc(hidden)]
pub struct VaListInner
{
	gp_offset: u32,
	fp_offset: u32,
	overflow_arg_area: *const (),
	reg_save_area: *const (),
}

/// Trait implemented on types that can be read from a va_list
pub trait VaPrimitive
{
	#[doc(hidden)]
	unsafe fn get(&mut VaListInner) -> Self;
}

impl va_list
{
	/// Read a value from the va_list
	///
	/// Users should take care that they are reading the correct type
	pub unsafe fn get<T: VaPrimitive>(&mut self) -> T {
		//log_debug!("inner = {:p} {:?}", self.0, *self.0);
		T::get(&mut *self.0)
	}
}

#[doc(hidden)]
impl VaListInner
{
	fn check_space(&self, num_gp: u32, num_fp: u32) -> bool {
		!(self.gp_offset > 48 - num_gp * 8 || self.fp_offset > 304 - num_fp * 16)
	}
	unsafe fn get_gp<T>(&mut self) -> T {
		let n_gp = (mem::size_of::<T>()+7)/8;
		assert!( self.check_space(n_gp as u32, 0) );
		let rv = ptr::read( (self.reg_save_area as usize + self.gp_offset as usize) as *const _ );
		self.gp_offset += (8*n_gp) as u32;
		rv
	}
	unsafe fn get_overflow<T>(&mut self) -> T {
		let align = mem::min_align_of::<T>();
		// 7. Align overflow_reg_area upwards to a 16-byte boundary if alignment
		//    needed by T exceeds 8 bytes
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
		// 8. Fetch from overflow areay
		let rv = ptr::read( self.overflow_arg_area as *const _ );
		self.overflow_arg_area = ((self.overflow_arg_area as usize) + mem::size_of::<T>()) as *const _;
		rv
	}
}

impl<T> VaPrimitive for *const T
{
	unsafe fn get(inner: &mut VaListInner) -> Self {
		<usize>::get(inner) as *const T
	}
}

macro_rules! impl_va_prim {
	($u:ty, $s:ty) => {
		impl VaPrimitive for $u {
			unsafe fn get(inner: &mut VaListInner) -> Self {
				// See the ELF AMD64 ABI document for a description of how this should act
				if ! inner.check_space(1, 0) {
					inner.get_overflow()
				}
				else {
					inner.get_gp()
				}
			}
		}
		impl VaPrimitive for $s {
			unsafe fn get(inner: &mut VaListInner) -> Self {
				mem::transmute( <$u>::get(inner) )
			}
		}
	};
}

impl_va_prim!{ usize, isize }
impl_va_prim!{ u64, i64 }
impl_va_prim!{ u32, i32 }
impl_va_prim!{ u16, i16 }
impl_va_prim!{ u8, i8 }

