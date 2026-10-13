#ifndef ORDERBOOK_H
#define ORDERBOOK_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Fixed-size memory layout for the zero-copy SPSC ring buffer
typedef struct {
    uint64_t order_id;
    uint64_t price;
    uint32_t size;
    uint8_t side; // 0 = Bid, 1 = Ask
    uint8_t _padding[3]; // 32-byte alignment for SIMD cache lines
} Order;

typedef struct {
    uint64_t best_bid;
    uint64_t best_ask;
    uint32_t bid_volume;
    uint32_t ask_volume;
} BookSnapshot;

void engine_init();
void process_orders(const Order* orders, uint32_t count);
BookSnapshot get_book_snapshot();

#ifdef __cplusplus
}
#endif

#endif // ORDERBOOK_H