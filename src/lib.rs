/*!
 * C FFI - va_list support
 *
 * This crate provides an interface for rust code to read values passed in C's va_list type.
 *
 * ## Example 
 * ```rust
 * extern crate va_list;
 * 
 * extern "C" fn print_ints_va(count: u32, mut args: va_list::VaList)
 * {
 *   unsafe {
 *     for i in (0 .. count) {
 *       println!("{}: {}", i, args.get::<i32>());
 *     }
 *   }
 * }
 * ```
 */
#![cfg_attr(any(feature="no_std",nightly),feature(no_std))]
#![cfg_attr(any(feature="no_std",no_std),no_std)]
#![crate_type="lib"]
#![crate_name="va_list"]

#[cfg(any(feature="no_std",no_std))] #[doc(hidden)]
mod std {
	pub use core::{mem,ptr};
}

// x86_64 on unix platforms is _usually_ ELF.
#[cfg(target_arch="x86_64")] #[cfg(target_family="unix")]
#[path="impl-x86_64-elf.rs"] mod imp;
//// x86_64 on windows is special
//#[cfg(target_arch="x86_64")] #[cfg(target_family="windows")]
//#[path="impl-x86_64-elf.rs"] mod imp;
// x86+unix = cdecl
#[cfg(target_arch="x86")] #[cfg(target_family="unix")]
#[path="impl-x86-sysv.rs"] mod imp;

#[cfg(target_arch="arm")] #[cfg(target_family="unix")]
#[path="impl-arm-sysv.rs"] mod imp;

pub use imp::VaList;

/// Trait implemented on types that can be read from a va_list
pub trait VaPrimitive: 'static
{
	#[doc(hidden)]
	unsafe fn get(&mut VaList) -> Self;
}

