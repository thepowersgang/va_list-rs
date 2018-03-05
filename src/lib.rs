/*!
 * C FFI - va_list support
 *
 * This crate provides an interface for rust code to read values passed in C's va_list type.
 *
 * ## Example
 * In C Code
 * ```c
 * #include <stdint.h>
 * #include <stdarg.h>
 * extern void print_ints_va(uint32_t count, va_list args);
 * extern void print_ints(uint32_t count, ...)
 * {
 *   va_list args;
 *   va_start(args, count);
 *   print_ints_va(count, args);
 *   va_end(args);
 * }
 * ```
 *
 * In rust code:
 * ```rust
 * extern crate va_list;
 *
 * #[no_mangle]
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
#![cfg_attr(feature = "no_std", no_std)]
#![crate_type = "lib"]
#![crate_name = "va_list"]

#[cfg(feature = "no_std")]
#[doc(hidden)]
mod std {
    pub use core::{mem, ptr};
}

// x86_64 on unix platforms is _usually_ ELF.
#[cfg(all(target_arch = "x86_64", any(target_family = "unix", target_os = "tifflin")))]
#[path = "impl-x86_64-elf.rs"]
mod imp;

//// x86_64 on windows is special
#[cfg(all(target_arch = "x86_64", target_family = "windows"))]
#[path = "impl-x86_64-win64.rs"]
mod imp;

// x86+unix = cdecl
#[cfg(all(target_arch = "x86", target_family = "unix"))]
#[path = "impl-x86-sysv.rs"]
mod imp;

// arm+unix = cdecl
#[cfg(all(target_arch = "arm", target_family = "unix"))]
#[path = "impl-arm-sysv.rs"]
mod imp;

/// Rust version of C's `va_list` type from the `stdarg.h` header
#[repr(C)]
pub struct VaList {
    internal: imp::VaList,
}

/// Core type as passed though the FFI
impl VaList {
    /// Read a value from the VaList.
    ///
    /// Users should take care that they are reading the correct type
    pub unsafe fn get<T: VaPrimitive>(&mut self) -> T {
        T::get(&mut self.internal)
    }
}

/// Trait implemented on types that can be read from a va_list
pub trait VaPrimitive: 'static {
    #[doc(hidden)]
    unsafe fn get(&mut imp::VaList) -> Self;
}
