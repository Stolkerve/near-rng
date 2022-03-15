pub struct Rng {
    state: (u64, u64)
}

impl Rng {
    pub fn new(random_vec_seed: &Vec<u8>) -> Self {
        let mut seed = 0;

        seed |= u64::from(random_vec_seed[0]) << 56;
        seed |= u64::from(random_vec_seed[1]) << 48;
        seed |= u64::from(random_vec_seed[2]) << 40;
        seed |= u64::from(random_vec_seed[3]) << 32;
        seed |= u64::from(random_vec_seed[4]) << 24;
        seed |= u64::from(random_vec_seed[5]) << 16;
        seed |= u64::from(random_vec_seed[6]) << 8;
        seed |= u64::from(random_vec_seed[7]) << 0;

        Self {state: (
            seed ^ 0xf4dbdf2183dcefb7, // [crc32(b"0"), crc32(b"1")]
            seed ^ 0x1ad5be0d6dd28e9b  // [crc32(b"2"), crc32(b"3")]
        )}
    }

    pub fn rand_u64(&mut self) -> u64 {
        let (mut x, y) = self.state;
        self.state.0 = y;
        x ^= x << 23;
        self.state.1 = x ^ y ^ (x >> 17) ^ (y >> 26);
        self.state.1.wrapping_add(y)
    }

    #[inline]
    pub fn rand_u32(&mut self) -> u32 {
        (self.rand_u64() >> 32) as u32
    }
    #[inline]
    pub fn rand_u8(&mut self) -> u8 {
        self.rand_u32() as u8
    }

    #[inline]
    pub fn rand_u16(&mut self) -> u16 {
        self.rand_u32() as u16
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    pub fn rand_usize(&mut self) -> usize {
        self.rand_u32() as usize
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn rand_usize(&mut self) -> usize {
        self.rand_u64() as usize
    }

    // Algorithm to generate an equidistributed random number in 0..m.
    // Applies the idea of rejection sampling.
    pub fn rand_bounded_u32(&mut self, m: u32) -> u32 {
        let threshold = m.wrapping_neg().wrapping_rem(m);
        loop {
            let r = self.rand_u32();
            if r >= threshold {
                return r.wrapping_rem(m);
            }
        }
    }

    pub fn rand_bounded_u64(&mut self, m: u64) -> u64 {
        let threshold = m.wrapping_neg().wrapping_rem(m);
        loop {
            let r = self.rand_u64();
            if r >= threshold {
                return r.wrapping_rem(m);
            }
        }
    }

    #[cfg(target_pointer_width = "32")]
    pub fn rand_bounded_usize(&mut self, m: usize) -> usize {
        self.rand_bounded_u32(m as u32) as usize
    }

    #[cfg(target_pointer_width = "64")]
    pub fn rand_bounded_usize(&mut self, m: usize) -> usize {
        self.rand_bounded_u64(m as u64) as usize
    }

    pub fn rand_range_u32(&mut self, a: u32, b: u32) -> u32 {
       a + self.rand_bounded_u32(b-a)
    }
    pub fn rand_range_u64(&mut self, a: u64, b: u64) -> u64 {
       a + self.rand_bounded_u64(b-a)
    }
    pub fn rand_range_i32(&mut self, a: i32, b: i32) -> i32 {
        a + self.rand_bounded_u32((b-a) as u32) as i32
    }
    pub fn rand_range_i64(&mut self, a: i64, b: i64) -> i64 {
        a + self.rand_bounded_u64((b-a) as u64) as i64
    }

    pub fn rand_f32(&mut self) -> f32 {
        self.rand_u32() as f32 * 2.3283064E-10
    }

    pub fn rand_f64(&mut self) -> f64 {
        self.rand_u32() as f64 * 2.3283064365386963E-10
    }

    pub fn shuffle<T>(&mut self, a: &mut [T]) {
        if a.is_empty() {return;}
        let mut i = a.len() - 1;
        while i > 0 {
            let j = self.rand_usize()%(i + 1);
            a.swap(i, j);
            i -= 1;
        }
    }

    pub fn fill(&mut self, a: &mut[u8]) {
        let mut x = self.rand_u32();
        let mut count = 3;
        for p in a {
            *p = x as u8;
            if count == 0 {
                x = self.rand_u32();
                count = 3;
            } else {
                x  >>= 8;
                count -= 1;
            }
        }
    }
}