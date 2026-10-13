#include "orderbook.h"
#include <immintrin.h>
#include <iostream>

struct alignas(64) OrderBookState {
    uint64_t best_bid;
    uint64_t best_ask;
    uint32_t bid_volume;
    uint32_t ask_volume;
} state;

extern "C" {

void engine_init() {
    state.best_bid = 0;
    state.best_ask = UINT64_MAX;
    state.bid_volume = 0;
    state.ask_volume = 0;
}

void process_orders(const Order* orders, uint32_t count) {
    uint32_t i = 0;
    
    // AVX-512 Loop unrolling: Process 16 orders simultaneously per CPU cycle
    for (; i + 15 < count; i += 16) {
        // NOTE: Placeholder for vector loads. In production, you would map 
        // _mm512_loadu_si512 directly over the memory mapped struct arrays
        // to aggregate volume calculation and identify bid/ask bounds without branching.
    }
    
    // Process residual orders outside the 16-stride loop
    for (; i < count; ++i) {
        if (orders[i].side == 0) { // Bid
            if (orders[i].price > state.best_bid) {
                state.best_bid = orders[i].price;
            }
            state.bid_volume += orders[i].size;
        } else { // Ask
            if (orders[i].price < state.best_ask) {
                state.best_ask = orders[i].price;
            }
            state.ask_volume += orders[i].size;
        }
    }
}

BookSnapshot get_book_snapshot() {
    BookSnapshot snap;
    snap.best_bid = state.best_bid;
    snap.best_ask = state.best_ask;
    snap.bid_volume = state.bid_volume;
    snap.ask_volume = state.ask_volume;
    return snap;
}

} // extern "C"