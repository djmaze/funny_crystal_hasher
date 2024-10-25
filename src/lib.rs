use std::hash::Hasher;

use rand::random;

const C1: u64 = 0xacd5ad43274593b9u64;
const C2: u64 = 0x6956abd6ed268a3du64;

pub struct FunnyHasher {
    pub a: u64,
    pub b: u64,
}

impl Hasher for FunnyHasher {
    fn write(&mut self, bytes: &[u8]) {
        let mut size = bytes.len();
        let mut ptr = bytes;

        let last = if size <= 0 {
            0
        } else if size <= 3 {
            self.read_u24(ptr, size)
        } else if size <= 7 {
            let last = self.read_u32(ptr);
            let (_, part) = ptr.split_at(size & 3);
            last | self.read_u32(part) << 32
        } else {
            while size >= 8 {
                let val = self.read_u64(&mut ptr);
                self.permute(val);
                size -= 8;
            }
            // let mut last = orig_ptr.get(string.len() - 8..).unwrap();
            let (_, mut last) = bytes.split_at(bytes.len() - 8);
            self.read_u64(&mut last)
        };

        self.a ^= size as u64;
        self.b ^= size as u64;
        self.permute(last);
    }

    fn finish(&self) -> u64 {
        let (mut a, mut b) = (self.a.clone(), self.b.clone());
        a ^= (a >> 23) ^ (a >> 40);
        b ^= (b >> 23) ^ (b >> 40);
        a = a.wrapping_mul(C1);
        b = b.wrapping_mul(C2);
        a ^= a >> 32;
        b ^= b >> 32;
        a.wrapping_add(b)
    }
}

impl FunnyHasher {
    pub fn new(a: u64, b: u64) -> Self {
        Self { a, b }
    }

    fn permute(&mut self, v: u64) {
        self.a = (self.a ^ v).rotate_left(32).wrapping_mul(C1);
        self.b = (self.b.rotate_left(32) ^ v).wrapping_mul(C2);
    }

    fn read_u64(&self, input: &mut &[u8]) -> u64 {
        let (int_bytes, rest) = input.split_at(std::mem::size_of::<u64>());
        *input = rest;
        u64::from_le_bytes(int_bytes.try_into().unwrap())
    }

    fn read_u32(&self, input: &[u8]) -> u64 {
        let (int_bytes, _) = input.split_at(std::mem::size_of::<u32>());
        u32::from_le_bytes(int_bytes.try_into().unwrap()) as u64
    }

    fn read_u24(&self, input: &[u8], size: usize) -> u64 {
        let first_number = input[0] as u64;
        let second_number = (input[size / 2] as u64) << 8;
        let third_number = (input[size - 1] as u64) << 16;

        first_number | second_number | third_number
    }
}

impl Default for FunnyHasher {
    fn default() -> Self {
        Self {
            a: random(),
            b: random(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::hash::Hasher;

    use crate::FunnyHasher;

    const TEST_SEEDS: (u64, u64) = (11111, 22222);

    #[test]
    fn test_hashes() {
        let mut hasher = FunnyHasher::new(TEST_SEEDS.0, TEST_SEEDS.1);
        hasher.write("foo@bar.com".as_bytes());
        assert_eq!(15797299671405575656, hasher.finish());

        let mut hasher = FunnyHasher::new(TEST_SEEDS.0, TEST_SEEDS.1);
        hasher.write("mike@awesomewebsite.com".as_bytes());
        assert_eq!(13983182794833320390, hasher.finish());

        let mut hasher = FunnyHasher::new(TEST_SEEDS.0, TEST_SEEDS.1);
        hasher.write("test@example.com".as_bytes());
        assert_eq!(6500205784594761051, hasher.finish());
    }

    #[test]
    fn test_seven_byte_string_hash() {
        let mut hasher = FunnyHasher::new(TEST_SEEDS.0, TEST_SEEDS.1);
        hasher.write("d@aa.bb".as_bytes());
        assert_eq!(15971106912940118731, hasher.finish());
    }

    #[test]
    fn test_short_string_hash() {
        let mut hasher = FunnyHasher::new(TEST_SEEDS.0, TEST_SEEDS.1);
        hasher.write("nida".as_bytes());
        assert_eq!(17727584103583305057, hasher.finish());
    }

    #[test]
    fn test_even_shorter_string_hash() {
        let mut hasher = FunnyHasher::new(TEST_SEEDS.0, TEST_SEEDS.1);
        hasher.write("nid".as_bytes());
        assert_eq!(11947054985116609127, hasher.finish());
    }

    #[test]
    fn test_empty_string_hash() {
        let mut hasher = FunnyHasher::new(TEST_SEEDS.0, TEST_SEEDS.1);
        hasher.write("".as_bytes());
        assert_eq!(9795944661289419247, hasher.finish());
    }

    #[test]
    fn test_default_seeds() {
        let mut hasher = FunnyHasher::default();
        hasher.write("foo".as_bytes());
        assert_ne!(0, hasher.finish());
    }
}
