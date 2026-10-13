use std::mem;

mod engine;

// Isolate the process to a specific core to maximize L1/L2 cache hits
fn pin_to_core(core_id: usize) {
    #[cfg(target_os = "linux")]
    unsafe {
        let mut cpuset: libc::cpu_set_t = mem::zeroed();
        libc::CPU_ZERO(&mut cpuset);
        libc::CPU_SET(core_id, &mut cpuset);
        libc::sched_setaffinity(0, mem::size_of::<libc::cpu_set_t>(), &cpuset);
    }
}

fn main() {
    println!("Initializing Aegis Engine...");
    
    // Pin main thread to core 1 
    pin_to_core(1);
    
    // Initialize the shared memory file
    let engine = engine::MatchingEngine::new("/tmp/aegis_engine.mmap");
    
    // Verify memory map and FFI cross-talk
    let snap = engine.snapshot();
    println!("Memory Map Synchronized.");
    println!("Initial Book Snapshot - Bid: {}, Ask: {}", snap.best_bid, snap.best_ask);
}