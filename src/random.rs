/// This is the "belt-and-suspenders" PRNG
/// from the 3rd edition of Numerical Recipes
///
/// It combines a few different number generation schemes to
/// make something pretty "bulletproof" - more so than needed
/// in this case, to be perfectly honest.
///
/// Still, it's not too much slower than just an XorShift-
/// based PRNG, and it was easy to copy the code for this from
/// another project.

pub struct Random {
    u: u64,
    v: u64,
    w: u64,
}

impl Random {
    pub fn new(seed: u64) -> Self {
        let v = 4_101_842_887_655_102_017;
        let w = 1;

        let u = seed ^ v;

        let mut r = Self { u, v, w };
        r.get();

        r.v = r.u;
        r.get();

        r.w = r.v;
        r.get();

        r
    }

    pub fn get(&mut self) -> u64 {
        let u_mul: u64 = 2_862_933_555_777_941_757;
        let u_add: u64 = 7_046_029_254_386_353_087;

        self.u = self.u.wrapping_mul(u_mul).wrapping_add(u_add);

        self.v ^= self.v.wrapping_shr(17);
        self.v ^= self.v.wrapping_shl(31);
        self.v ^= self.v.wrapping_shr(8);

        self.w = 4_294_957_665u64
            .wrapping_mul(self.w & 0xffff_ffff)
            .wrapping_add(self.w.wrapping_shr(32));

        let mut x = self.u ^ (self.u << 21);
        x ^= x.wrapping_shr(35);
        x ^= x.wrapping_shl(4);

        x.wrapping_add(self.v) ^ self.w
    }

    pub fn get_f32(&mut self) -> f32 {
        (self.get() as f32) / (u64::MAX as f32 + 1.0)
    }

    pub fn get_f32_in_range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.get_f32()
    }
}
