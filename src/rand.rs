/*
Taken from here:
https://en.wikipedia.org/wiki/Xorshift

struct xorshift32_state {
  uint32_t a;
};

/* The state word must be initialized to non-zero */
uint32_t xorshift32(struct xorshift32_state *state)
{
    /* Algorithm "xor" from p. 4 of Marsaglia, "Xorshift RNGs" */
    uint32_t x = state->a;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    return state->a = x;
}
*/

use core::cell::Cell;

pub struct XorShift32 {
    state: Cell<u32>,
}

impl XorShift32 {
    pub fn new(seed: u32) -> Self {
        assert_ne!(seed, 0);
        XorShift32 {
            state: Cell::new(seed),
        }
    }

    pub fn next(&self) -> u32 {
        let mut x: u32 = self.state.get();

        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;

        self.state.set(x);

        x
    }
}
