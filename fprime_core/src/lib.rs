#![no_std]
mod serializable;

pub use fprime_macros::Serializable;

/// Fprime strings are stored on the stack with 16-bit lengths
pub type String<const N: usize> = heapless::String<N, u16>;

pub trait Serializable {
    const SIZE: usize;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize);
}
