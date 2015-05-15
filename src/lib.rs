/*!
 * C FFI - va_list support
 *
 * This crate provides an interface for rust code to read values passed in C's va_list type.
 *
 * ## Example 
 * ```rust
 * extern crate va_list;
 * use va_list::VaList;
 * 
 * extern "C" print_ints_va(count: u32, mut args: VaList)
 * {
 * 	for i in (0 .. count) {
 * 		println!("{}: {}", i, args.get::<i32>());
 * 	}
 * }
 * ```
 */
#![cfg_attr(nightly,feature(no_std,core))]
#![cfg_attr(nightly,no_std)]
#![crate_type="lib"]
#![crate_name="va_list"]

#[cfg(nightly)]
#[macro_use]
extern crate core;

// x86_64 on unix platforms is _usually_ ELF.
#[cfg(target_arch="x86_64")] #[cfg(target_family="unix")]
#[path="impl-x86_64-elf.rs"] mod imp;
//// x86_64 on windows is special
//#[cfg(target_arch="x86_64")] #[cfg(target_family="windows")]
//#[path="impl-x86_64-elf.rs"] mod imp;
// x86+unix = cdecl
#[cfg(target_arch="x86")] #[cfg(target_family="unix")]
#[path="impl-x86-sysv.rs"] mod imp;

pub use imp::VaList;

/// Trait implemented on types that can be read from a va_list
pub trait VaPrimitive: 'static
{
	#[doc(hidden)]
	unsafe fn get(&mut VaList) -> Self;
}

