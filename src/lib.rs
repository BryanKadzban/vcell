//! Just like [`Cell`] but with [volatile] read / write operations
//!
//! [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
//! [volatile]: https://doc.rust-lang.org/std/ptr/fn.read_volatile.html

#![deny(missing_docs)]
#![deny(warnings)]
#![cfg_attr(feature = "const-fn", feature(const_fn))]
#![no_std]

use core::cell::UnsafeCell;
use core::ptr;

/// Just like [`Cell`] but with [volatile] read / write operations
///
/// [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
/// [volatile]: https://doc.rust-lang.org/std/ptr/fn.read_volatile.html
pub struct VolatileCell<T> {
    value: UnsafeCell<T>,
}

impl<T> VolatileCell<T> {
    /// Creates a new `VolatileCell` containing the given value
    #[cfg(feature = "const-fn")]
    pub const fn new(value: T) -> Self {
        VolatileCell { value: UnsafeCell::new(value) }
    }

    /// Creates a new `VolatileCell` containing the given value
    ///
    /// NOTE A `const fn` variant is available under the "const-fn" Cargo
    /// feature
    #[cfg(not(feature = "const-fn"))]
    pub fn new(value: T) -> Self {
        VolatileCell { value: UnsafeCell::new(value) }
    }

    /// Returns a copy of the contained value
    #[inline(always)]
    pub fn get(&self) -> T
        where T: Copy
    {
        unsafe { ptr::read_volatile(self.value.get()) }
    }

    /// Sets the contained value
    #[inline(always)]
    pub fn set(&self, value: T)
        where T: Copy
    {
        unsafe { ptr::write_volatile(self.value.get(), value) }
    }

    /// Sets a sub-field of the contained value with the bit-manipulation-engine, if enabled.
    /// See [NXP documentation] on the BME. This is a "BFI" operation.
    ///
    /// [NXP documentation]: https://www.nxp.com/docs/en/application-note/AN4838.pdf
    #[inline(always)]
    #[cfg(feature = "bit-manipulation")]
    pub fn set_field(&self, first_bit: u8, bit_count: u8, value: T)
        where T: Copy
    {
        unsafe {
            let addr = self.value.get() as usize as u32;
            if addr & 0xe007ffff != addr {
                panic!("Tried to use BME on address 0x{:x?}, which is not in either the peripheral or upper-SRAM address range");
            }
            let bfi_addr = addr | 0x10000000 |
                (((first_bit & 0x1f) as u32) << 23) |
                ((((bit_count-1) & 0xf) as u32) << 19);
            let bfi_ptr = bfi_addr as usize as *mut T;
            ptr::write_volatile(bfi_ptr, value)
        }
    }
}

// NOTE implicit because of `UnsafeCell`
// unsafe impl<T> !Sync for VolatileCell<T> {}