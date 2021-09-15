use core::ptr;
mod rllib {
    pub struct Xorshift {
        x: u32,
        y: u32,
        z: u32,
        w: u32,
    }
    impl Xorshift {
        fn new() -> Self {
            Self {
                x: 0,
                y: 0,
                z: 0,
                w: 0,
            }
        }
        fn new_seed(seed: u32) -> Self {
            Self {
                x: seed,
                y: seed,
                z: seed,
                w: seed,
            }
        }
    }
    pub trait XorshiftTrait {
        fn reseed(&mut self, seed: u32);
        fn reseed_u64(&mut self, seed: u64);
        //fn mix(&mut self);
        fn rand_u32(&mut self) -> u32;
        fn rand_u64(&mut self) -> u64;
        //fn rand_p(&mut self, blob: *mut [u32], bytes: usize);
    }
    impl XorshiftTrait for Xorshift {
        fn reseed(&mut self, seed: u32) {
            let mut x = 0x498b3bc5 ^ (seed >> 0);
            let mut y = 0x5a05089a ^ (seed >> 2);
            let mut z = 0;
            let mut w = 0;
            for i in 0..10 {
                let t: u32 = x ^ (x << 11);
                x = y;
                y = z;
                z = w;
                w = w ^ (w >> 19) ^ t ^ (t >> 8); 
            }
        }
        
        fn reseed_u64(&mut self, seed: u64) {
            let mut x = 0x498b3bc5 ^ (seed >> 0 as u8 as u32);
            let mut y = 0x5a05089a ^ (seed >> 32 as u8 as u32);
            let mut z = 0;
            let mut w = 0;
            for i in 0..10 {
                let t: u64 = x ^ (x << 11);
                x= y;
                y = z;
                z = w;
                w = w ^ (w >> 19) ^ t ^ (t >> 8);
            }
        }
        fn rand_u32(&mut self) -> u32 {
            let t: u32 = self.x ^ (self.x << 11);
            let x = self.y;
            let y = self.z;
            let z = self.w;
            let w = self.w ^ (self.w >> 19) ^ t ^ (t >> 8);
            return x;
        }
        fn rand_u64(&mut self) -> u64 {
            let x = self.y;
            let y = self.z;
            let a: u64 = x.into();
            let b: u64 = y.into();
            return (a << 32) | b
        }
        /*
        fn rand_p(&mut self, blob: *mut [u32], mut bytes: usize) {
            let mut blocks = blob;
            let mut i: usize = 0;
            while bytes >= 4 {
                unsafe {
                    blocks.offset(0) = self.rand_u32();
                    blocks.add(1);
                }
                bytes -= 4;
            }
            let mut tail = blocks as *mut [u8]; 
            for i in 0..bytes {
                unsafe {
                    tail.add(i).write(self.rand_u32());
                }
            }
        }
        */
        
    }
    pub struct Random {
        xorshift: Xorshift,
    }
    impl Random {
        fn new() -> Self {
            Self {
                xorshift: Xorshift::new(),
            }
        }
    }
    pub trait RandomTrait {
        fn reseed(&mut self, seed: u32);
        fn rnd(&mut self) -> u32;
        fn nextint(&mut self, sz: i32) -> i32;
    }
    impl RandomTrait for Random {
        fn reseed(&mut self, seed: u32) {
            self.xorshift.reseed(seed);
        }
        fn rnd(&mut self) -> u32 {
            return self.xorshift.rand_u32();
        }
        fn nextint(&mut self, sz: i32) -> i32 {
            return self.rnd() as i32 % sz;
        }
    }
}
