use memmap2::{MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::mem::size_of;
use std::sync::atomic::{AtomicUsize, Ordering};

// 1 Million order capacity before wrap-around
pub const RING_BUFFER_SIZE: usize = 1024 * 1024; 

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct FfiOrder {
    pub order_id: u64,
    pub price: u64,
    pub size: u32,
    pub side: u8,
    pub _padding: [u8; 3],
}

#[repr(C)]
pub struct SpscRingBuffer {
    pub head: AtomicUsize,
    pub tail: AtomicUsize,
    pub buffer: [FfiOrder; RING_BUFFER_SIZE],
}

pub struct MemoryManager {
    mmap: MmapMut,
}

impl MemoryManager {
    pub fn new(file_path: &str) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .expect("Failed to open or create shared memory file");

        let size = size_of::<SpscRingBuffer>() as u64;
        file.set_len(size).expect("Failed to set memory map size");

        let mmap = unsafe {
            MmapOptions::new()
                .map_mut(&file)
                .expect("Failed to map IPC memory")
        };

        let ring = unsafe { &mut *(mmap.as_ptr() as *mut SpscRingBuffer) };
        ring.head.store(0, Ordering::Release);
        ring.tail.store(0, Ordering::Release);

        Self { mmap }
    }

    pub fn ring_buffer(&mut self) -> &mut SpscRingBuffer {
        unsafe { &mut *(self.mmap.as_mut_ptr() as *mut SpscRingBuffer) }
    }
}