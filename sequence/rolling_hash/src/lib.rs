pub mod rolling_hash {
    const MOD: u64 = (1 << 61) - 1;
    const MASK30: u64 = (1 << 30) - 1;
    const MASK31: u64 = (1 << 31) - 1;

    fn mul_mod(a: &u64, b: &u64) -> u64 {
        let (au, al) = (a >> 31, a & MASK31);
        let (bu, bl) = (b >> 31, b & MASK31);
        let c = al * bu + au * bl;
        let (cu, cl) = (c >> 30, c & MASK30);
        let d = au * bu * 2 + cu + (cl << 31) + al * bl;
        let e = (d >> 61) + d & MOD;
        if e >= MOD {
            e - MOD
        } else {
            e
        }
    }

    pub struct RollingHash {
        hash_acc: Vec<u64>,
        base_pow: Vec<u64>,
    }

    impl RollingHash {
        pub fn new(a: &[u64], base: u64) -> Self {
            let mut hash_acc = vec![0];
            let mut base_pow = vec![1];
            for a in a {
                let mut hash = mul_mod(hash_acc.last().unwrap(), &base) + a + 1;
                if hash >= MOD {
                    hash -= MOD;
                }
                hash_acc.push(hash);
                base_pow.push(mul_mod(base_pow.last().unwrap(), &base));
            }
            Self { hash_acc, base_pow }
        }

        pub fn substring_hash(&self, l: usize, r: usize) -> u64 {
            let l = mul_mod(&self.hash_acc[l], &self.base_pow[r - l]);
            let r = self.hash_acc[r];
            if r < l {
                MOD - l + r
            } else {
                r - l
            }
        }
    }
}
