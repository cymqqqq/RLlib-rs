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
        fn mix(&mut self);
        fn rand_u32(&mut self) -> u32;
        fn rand_u64(&mut self) -> u64;
        fn rand_p(&mut self, blob: Box<_>, bytes: i32);
    }
    impl XorshiftTrait for Xorshift {
        fn reseed(&mut self, seed: u32) {
            let x = 0x498b3bc5 ^ seed;
            let y = 0;
            let z = 0;
            for i in 0..10 {
                self.mix();
            }
        }
        
        fn mix(&mut self) {
            let t: u32 = self.x ^ (self.x << 11);
            let x = self.x;
            let y = self.x;
            let z = self.w;
            let w = self.w ^ (self.w >> 19) ^ t ^ (t >> 8);
        }
        fn reseed_u64(&mut self, seed: u64) {
            let x = 0x498b3bc5 ^ (seed >> 0 as u8 as u32);
            let y = 0x5a05089a ^ (seed >> 32 as u8 as u32);
            let z = 0;
            let w = 0;
            for i in 0..10 {
                self.mix();
            }
        }
        fn rand_u32(&mut self) -> u32 {
            self.mix();
            return self.x;
        }
        fn rand_u64(&mut self) -> u64 {
            self.mix();
            let mut a: u64 = self.x.into();
            let mut b: u64 = self.y.into();
            return (a << 32) | b
        }
        /*
        fn rand_p(&mut self, blob: &[], mut bytes: i32) {
            let mut blocks = blob as &[u8] as &[u32];
            let mut i: usize = 0;
            while bytes >= 4 {
                blocks[0] = self.rand_u32();
                blocks[i += 1];
                bytes -= 4;
            }
            
        }
        */
    }
}
