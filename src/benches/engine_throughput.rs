use criterion::{criterion_group, criterion_main, Criterion, black_box};
use aegis_engine_backend::engine::memory::{FfiOrder, SpscRingBuffer, RING_BUFFER_SIZE};
use std::sync::atomic::{AtomicUsize, Ordering};

fn bench_spsc_ring_push(c: &mut Criterion) {
    // Allocate buffer on the heap to avoid blowing the stack in the benchmark runner
    let mut ring = Box::new(SpscRingBuffer {
        head: AtomicUsize::new(0),
        tail: AtomicUsize::new(0),
        buffer: [FfiOrder::default(); RING_BUFFER_SIZE],
    });

    c.bench_function("spsc_ring_buffer_push_1M_ops", |b| {
        b.iter(|| {
            let mut current_head = ring.head.load(Ordering::Acquire);
            
            // Simulate bursting 1,000 orders into the buffer
            for i in 0..1000 {
                let next_head = (current_head + 1) % RING_BUFFER_SIZE;
                
                ring.buffer[current_head] = FfiOrder {
                    order_id: black_box(i as u64),
                    price: black_box(100_000),
                    size: black_box(10),
                    side: black_box(0),
                    _padding: [0; 3],
                };
                
                current_head = next_head;
            }
            
            ring.head.store(current_head, Ordering::Release);
        })
    });
}

criterion_group!(benches, bench_spsc_ring_push);
criterion_main!(benches);