pub mod bindings;
pub mod memory;

use bindings::{engine_init, get_book_snapshot, process_orders, BookSnapshot, Order};
use memory::{MemoryManager, SpscRingBuffer};

pub struct MatchingEngine {
    mem_mgr: MemoryManager,
}

impl MatchingEngine {
    pub fn new(ipc_path: &str) -> Self {
        unsafe {
            engine_init();
        }
        Self {
            mem_mgr: MemoryManager::new(ipc_path),
        }
    }

    pub fn process_batch(&mut self, batch_size: u32) {
        let ring = self.mem_mgr.ring_buffer();
        let tail = ring.tail.load(std::sync::atomic::Ordering::Acquire);
        
        let orders_ptr = unsafe { ring.buffer.as_ptr().add(tail) };
        
        unsafe {
            // Reinterpret cast from memory::FfiOrder to bindings::Order
            process_orders(orders_ptr as *const Order, batch_size);
        }
        
        // Move tail forward to consume the batch
        let new_tail = (tail + batch_size as usize) % memory::RING_BUFFER_SIZE;
        ring.tail.store(new_tail, std::sync::atomic::Ordering::Release);
    }

    pub fn snapshot(&self) -> BookSnapshot {
        unsafe { get_book_snapshot() }
    }
}