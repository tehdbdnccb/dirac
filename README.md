# 🛡️ Aegis-DLMM: Ultra-Low Latency Quantitative Engine for Solana

**Hackathon:** Colosseum Crypto World's Fair (Fall 2026)  
**Track:** Meteora Sidetrack (DLMM / DAMM v2)  
**Status:** Mainnet-Beta Ready  

Aegis is an institutional-grade, ultra-low latency high-frequency trading (HFT) and clearing engine built explicitly for the Solana ecosystem. By bridging a C++ AVX-512 vector math engine with a Rust/Axum networking layer via zero-copy shared memory, Aegis dynamically shifts liquidity across Meteora DLMM bins in micro-seconds to capture volatility spreads while completely neutralizing toxic flow and MEV sandwich attacks.

---

## ⚡ Technical Highlights

Unlike standard web3 applications that rely on heavy JSON-RPC polling and standard React state, Aegis brings TradFi HFT architecture to DeFi:

*   **Zero-Copy Memory Architecture:** Bypasses standard IPC serialization using `memmap2` and lock-free Single-Producer Single-Consumer (SPSC) ring buffers.
*   **Vectorized Quant Math:** Utilizes AVX-512 SIMD intrinsics in C++ to compute multi-dimensional **Hawkes Self-Exciting Point Processes** and **Avellaneda-Stoikov** inventory models concurrently, without branching.
*   **Anchor Bytemuck Deserialization:** Smart contracts use `bytemuck` to cast raw account data buffers directly to Rust structs in $O(1)$ time, saving tens of thousands of Compute Units (CUs) over standard `AccountDeserialize`.
*   **Mempool Bypass:** All dynamic bin rebalancing and liquidity shifting on Meteora is routed through atomic Jito bundles, guaranteeing execution order and preventing searcher front-running.
*   **Sub-Microsecond Telemetry:** Next.js dashboard powered by Zustand and WebSockets visualizes live order book depth and pipeline latency without triggering excessive React render cycles.

---

## 🏗️ System Architecture (The 5 Phases)

### 1. Memory & FFI Layer (C++ / Rust)
The ingestion layer writes incoming market ticks directly to memory-mapped SPSC ring buffers. The C++ execution engine reads from the exact same physical memory layout, evaluating market toxicity using exponential decay kernels in sub-microsecond timeframes.

### 2. High-Frequency Telemetry API
A heavily optimized Axum WebSocket server streams binary payloads directly to frontend clients. Threads are manually pinned to isolated CPU cores using `libc::sched_setaffinity` to prevent Linux kernel context-switching latency.

### 3. On-Chain Execution (Solana & Meteora)
When the Hawkes intensity breaches toxicity thresholds, the engine builds an Anchor Cross-Program Invocation (CPI) targeting Meteora's `remove_liquidity` and `add_liquidity_by_strategy` instructions.

### 4. Autonomous Risk Engine
Continuously sweeps account collateral against mark prices. If maintenance margins are breached, the lock-free liquidation queue natively dispatches execution orders to square the position before the next block.

### 5. Live Telemetry Dashboard
A Next.js 15 interface providing Colosseum judges and node operators visual proof of the system's performance, including a live DLMM bin distribution chart, dynamic APY fee tracking, and a hardware latency gauge.

---

## 🚀 Getting Started

### Prerequisites
*   Linux environment (Ubuntu 24.04+ recommended for thread pinning and PMU profiling)
*   Rust toolchain (1.80+)
*   C++20 Compiler with AVX-512 support (`g++` or `clang++`)
*   Node.js 20+ & pnpm
*   Solana CLI (`v2.0+`)

### 1. Compile the Backend (Rust + C++ FFI)
The `build.rs` script will automatically compile the AVX-512 engine and generate the FFI bindings.
```bash
# Clone the repository
git clone [https://github.com/your-username/aegis-dlmm.git](https://github.com/your-username/aegis-dlmm.git)
cd aegis-dlmm/backend

# Build with ultra-low latency release profile
cargo build --release
Run the Engine
Requires root privileges for CPU core pinning and SCHED_FIFO real-time priorities.

Bash
export SOLANA_RPC_URL="[https://api.mainnet-beta.solana.com](https://api.mainnet-beta.solana.com)"
export SOLANA_WALLET_PATH="~/.config/solana/aegis_clearing.json"

sudo -E ./target/release/aegis-engine-backend
3. Start the Next.js Telemetry Dashboard
Bash
cd ../frontend
pnpm install
pnpm run dev
Navigate to http://localhost:3000 to view the live bin utilization and latency gauges.

4. Run the Hardware Latency Profiler (Stress Test)
To verify cache-miss ratios and CPU core stability during the hackathon judging:

Bash
# Terminal 1: Run the load simulator (Blasts 5,000 TPS via WebSockets)
python3 scripts/load_simulator.py -c 20 -d 30 -r 0

# Terminal 2: Run the hardware profiler against the Rust binary
sudo ./scripts/latency_profiler.sh aegis-engine
🏆 Hackathon Context
This project was built for the Meteora Sidetrack at the Colosseum Crypto World's Fair (Fall 2026). It specifically demonstrates how institutional quant methodologies (Hawkes Processes) can be applied to Meteora's Dynamic Liquidity Market Maker (DLMM) architecture to actively defend LP positions against toxic flow.

Team: Elijah Branham Odhiambo

Track: Meteora DLMM / Infrastructure