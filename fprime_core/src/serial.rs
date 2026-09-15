use core::cell::UnsafeCell;
use core::marker::PhantomData;

use crate::abi;

use crate::Serializable;

pub struct Queue<T: Serializable, const N: usize> {
    index: i32,
    buffer: [u8; N],
    data: PhantomData<T>,
}

impl<T: Serializable, const N: usize> Queue<T, N> {
    pub fn new(index: i32) -> Queue<T, N> {
        const { assert!(N == T::SIZE) };

        Queue {
            index,
            buffer: [0; N],
            data: Default::default(),
        }
    }

    pub fn block_recv(&self) -> T {
        let mut written: u32 = 0;

        let _ = unsafe {
            abi::serial_recv(
                self.index,
                self.buffer.as_ptr() as u32,
                core::mem::size_of::<T>() as u32,
                (&mut written) as *mut u32 as u32,
                /* blocking */ 0,
            )
        };

        T::deserialize(&self.buffer[0..(written as usize)])
    }

    pub fn recv(&self) -> Option<T> {
        let mut written: u32 = 0;

        let status = unsafe {
            abi::serial_recv(
                self.index,
                self.buffer.as_ptr() as u32,
                core::mem::size_of::<T>() as u32,
                (&mut written) as *mut u32 as u32,
                /* non blocking */ 1,
            )
        };

        if status == 0 {
            // ok
            Some(T::deserialize(&self.buffer[0..(written as usize)]))
        } else {
            None
        }
    }
}

pub struct Sender<T: Serializable, const N: usize> {
    index: i32,
    buffer: UnsafeCell<[u8; N]>,
    data: PhantomData<T>,
}

impl<T: Serializable, const N: usize> Sender<T, N> {
    pub fn new(index: i32) -> Sender<T, N> {
        const { assert!(N == T::SIZE) };

        Sender {
            index,
            buffer: UnsafeCell::new([0; N]),
            data: Default::default(),
        }
    }

    pub fn send(&self, value: T) {
        let mut size = 0;
        let ptr = &mut unsafe { self.buffer.get().read() }[..];
        value.serialize_to(ptr, &mut size);

        unsafe {
            abi::serial_send(
                self.index,
                self.buffer.get().read().as_ptr() as u32,
                size as u32,
            )
        };
    }
}
