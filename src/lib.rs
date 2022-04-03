//!
//! C FFI - va_list support
//!
//! This crate provides an interface for rust code to read values passed in C's va_list type.
//!
//! ## Example
//! In C Code
//!
//! ```c++
//! #include <stdint.h>
//! #include <stdarg.h>
//! extern void print_ints_va(uint32_t count, va_list args);
//! extern void print_ints(uint32_t count, ...)
//! {
//!   va_list args;
//!   va_start(args, count);
//!   print_ints_va(count, args);
//!   va_end(args);
//! }
//! ```
//!
//! In rust code:
//!
//! ```rust
//! extern crate va_list;
//!
//! #[no_mangle]
//! extern "C" fn print_ints_va(count: u32, mut args: va_list::VaList)
//! {
//!   unsafe {
//!     for i in (0 .. count) {
//!       println!("{}: {}", i, args.get::<i32>());
//!     }
//!   }
//! }
//! ```
//!
#![cfg_attr(feature = "no_std", no_std)]
#![crate_type = "lib"]
#![crate_name = "va_list"]

#[cfg(feature = "no_std")]
#[doc(hidden)]
mod std {
    pub use core::{mem, ptr, ffi};
}

// Helper macro that allows build-testing all platforms at once
macro_rules! def_platforms {
	(
		$(
		if $conds:meta {
			mod $name:ident = $path:expr;
		}
		)*
	) => {
	#[cfg(build_check_all)]
	#[path="."]
	mod build_all {
		$(
		#[path="."]
		mod $name {
			#[path=$path]
			mod imp;

			#[allow(dead_code)]
			mod wrapper;
			#[allow(dead_code)]
			use self::wrapper::*;
		}
		)*
	}

	$(
		#[cfg($conds)]
		#[path=$path]
		mod imp;
	)*
	}
}

def_platforms! {
	// x86+unix = cdecl
	if all(target_arch = "x86", target_family = "unix") {
		mod x86_unix = "impl-x86-sysv.rs";
	}
	// arm+unix = cdecl
	if all(target_arch = "arm", target_family = "unix") {
		mod arm_sysv = "impl-arm-sysv.rs";
	}

	// x86_64 on unix platforms is _usually_ the ELF/itanium ABI
	if all(
		target_arch = "x86_64",
		any(target_family = "unix", target_os = "redox", target_os = "tifflin")
		) {
		mod x8664_elf = "impl-x86_64-elf.rs";
	}
	// x86_64 windows = ?cdecl (64-bit)
	if all(target_arch = "x86_64", target_family = "windows") {
		mod x8665_win64 = "impl-x86_64-win64.rs";
	}

	// aarch64 elf ABI
	if all(
		target_arch = "aarch64",
		any(target_family = "unix", target_os = "redox"),
		not(any(target_os = "macos", target_os = "ios")),	// Apple uses a 64-bit cdecl instead
		) {
		mod aarch64_elf = "impl-aarch64-elf.rs";
	}

	// aarch64+macos = cdecl (64-bit)
	if all(target_arch = "aarch64", any(target_os = "macos", target_os = "ios")) {
		mod aarch64_macos = "impl-aarch64-macos.rs";
	}
}

/// Wrapper logic, shared for testing
mod wrapper;
pub use self::wrapper::*;

