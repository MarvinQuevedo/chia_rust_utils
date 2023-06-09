use std::cmp::Ordering;
use std::ops;

const fn ucdiv(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

#[derive(Clone)]
pub struct BitVec {
    values: Vec<u64>,
    last_size: u32,
}
impl BitVec {
    pub fn new(value: u128, size: u32) -> Self {
        let mut bits: BitVec = BitVec {
            values: Vec::new(),
            last_size: 0,
        };
        if size > 64 {
            // std::cout << "SPLITTING BitsGeneric" << std::endl;
            bits.init_bits((value >> 64) as u64, size - 64);
            bits.append_value(value, 64);
        } else {
            bits.init_bits(value as u64, size);
        }
        bits
    }

    fn init_bits(&mut self, value: u64, size: u32) {
        self.last_size = 0;
        if size > 64 {
            // Get number of extra 0s added at the beginning.
            let mut zeros = size - self.get_size_bits(value as u128) as u32;
            // Add a full group of 0s (length 64)
            while zeros > 64 {
                self.append_value(0, 64);
                zeros -= 64;
            }
            // Add the incomplete group of 0s and then the value.
            self.append_value(0, zeros);
            self.append_value(value.into(), self.get_size_bits(value as u128) as u32);
        } else {
            /* 'value' must be under 'size' bits. */
            assert!(size == 64 || value == (value & ((1u64 << size) - 1)));
            self.values.push(value);
            self.last_size = size as u32;
        }
    }

    pub fn from_other(other: &BitVec) -> Self {
        BitVec {
            values: other.values.clone(),
            last_size: other.last_size,
        }
    }

    pub fn from_other_sized(other: &BitVec, size: u32) -> Self {
        let mut bits: BitVec = BitVec {
            values: Vec::new(),
            last_size: 0,
        };
        let total_size = other.get_size();
        assert!(size >= total_size);
        // Add the extra 0 bits at the beginning.
        let mut extra_space = size - total_size;
        while extra_space >= 64 {
            bits.append_value(0, 64);
            extra_space -= 64;
        }
        if extra_space > 0 {
            bits.append_value(0, extra_space);
        }
        // Copy the Bits object element by element, and append it to the current Bits object.
        if !other.values.is_empty() {
            let mut index = 0;
            while index < other.values.len() {
                bits.append_value(other.values[index].into(), 64);
                index += 1;
            }
            bits.append_value(other.values[other.values.len() - 1].into(), other.last_size);
        }
        bits
    }

    pub fn from_be_bytes(big_endian_bytes: Vec<u8>, num_bytes: u32, size_bits: u32) -> Self {
        let mut bits: BitVec = BitVec {
            values: Vec::new(),
            last_size: 0,
        };
        let mut extra_space = size_bits - num_bytes * 8;
        while extra_space >= 64 {
            bits.append_value(0, 64);
            extra_space -= 64;
        }
        if extra_space > 0 {
            bits.append_value(0, extra_space);
        }
        let mut i = 0;
        while i < num_bytes {
            let mut val = 0u64;
            let mut bucket_size = 0;
            // Compress bytes together into u64, either until we have 64 bits, or until we run
            // out of bytes in big_endian_bytes.
            let mut j = i;
            while j < i + u64::BITS / u8::BITS && j < num_bytes {
                val = (val << 8) + big_endian_bytes[j as usize] as u64;
                bucket_size += 8;
                j += 1;
            }
            bits.append_value(val.into(), bucket_size);
            i += u64::BITS / u8::BITS;
        }
        bits
    }

    pub fn slice(&self, start_index: u32) -> Self {
        self.range(start_index, self.get_size())
    }

    pub fn range(&self, start_index: u32, end_index: u32) -> Self {
        let mut start_index = start_index;
        let mut end_index = end_index;
        if end_index > self.get_size() {
            end_index = self.get_size();
        }
        if end_index == start_index {
            return BitVec {
                values: Vec::new(),
                last_size: 0,
            };
        }
        assert!(end_index > start_index);
        let start_bucket = start_index / 64;
        let end_bucket = end_index / 64;
        if start_bucket == end_bucket {
            // Positions inside the bucket.
            start_index %= 64;
            end_index %= 64;
            let bucket_size = if start_bucket as usize == (self.values.len() - 1) {
                self.last_size
            } else {
                64
            }; //u8?
            let mut val = self.values[start_bucket as usize];
            // Cut the prefix [0, start_index)
            if start_index != 0 {
                val &= ((1u64 << (bucket_size - start_index)) - 1);
            }
            // Cut the suffix after end_index
            val >>= (bucket_size - end_index);
            BitVec::new(val.into(), end_index - start_index)
        } else {
            let mut result = BitVec {
                values: Vec::new(),
                last_size: 0,
            };
            // Get the prefix from the last bucket.
            let mut split = self.split_number_by_prefix(
                self.values[start_bucket as usize],
                64,
                (start_index % 64) as u8,
            );
            result.append_value(split.1.into(), 64 - start_index % 64);
            // Append all the in between buckets
            let mut i = start_bucket + 1;
            while i < end_bucket {
                result.append_value(self.values[i as usize].into(), 64);
                i += 1;
            }
            if end_index % 64 > 0 {
                let bucket_size = if end_bucket == (self.values.len() - 1) as u32 {
                    self.last_size
                } else {
                    64
                }; //u8?
                   // Get the suffix from the last bucket.
                split = self.split_number_by_prefix(
                    self.values[end_bucket as usize],
                    bucket_size as u8,
                    (end_index % 64) as u8,
                );
                result.append_value(split.0.into(), end_index % 64);
            }
            result
        }
    }

    pub fn slice_to_int(&self, start_index: u32, end_index: u32) -> u64 {
        if (start_index >> 6) == (end_index >> 6) {
            let mut res: u64 = self.values[(start_index >> 6) as usize];
            if (start_index >> 6) as usize == self.values.len() - 1 {
                res >>= self.last_size - (end_index & 63);
            } else {
                res >>= 64 - (end_index & 63);
            }
            res &= ((1u64 << ((end_index & 63) - (start_index & 63))) - 1);
            res
        } else {
            assert!((start_index >> 6) + 1 == (end_index >> 6));
            let mut split = self.split_number_by_prefix(
                self.values[(start_index >> 6) as usize],
                64,
                (start_index & 63) as u8,
            );
            let mut result = split.1;
            if end_index % 64 > 0 {
                let bucket_size = if (end_index >> 6) as usize == self.values.len() - 1 {
                    self.last_size
                } else {
                    64
                };
                split = self.split_number_by_prefix(
                    self.values[(end_index >> 6) as usize],
                    bucket_size as u8,
                    (end_index & 63) as u8,
                );
                result = (result << (end_index & 63)) + split.0;
            }
            result
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut rtn = Vec::new();
        // Return if nothing to work on
        if !self.values.len() == 0 {
            return Vec::new();
        }
        let mut i = 0;
        while i < self.values.len() - 1 {
            rtn.extend(self.values[i].to_be_bytes());
            i += 1;
        }
        let size = ucdiv(self.last_size, 8);
        rtn.extend(
            (self.values[i] << (64 - self.last_size)).to_be_bytes()[0..size as usize].to_vec(),
        );
        rtn
    }

    pub fn to_string(&self) -> String {
        let mut str = String::new();
        let mut i = 0;
        while i < self.values.len() {
            let mut val: u64 = self.values[i];
            let size = if i == self.values.len() - 1 {
                self.last_size
            } else {
                64
            };
            let mut str_bucket = String::new();
            let mut i2 = 0;
            while i2 < size {
                if val % 2 > 0 {
                    str_bucket = "1".to_owned() + &str_bucket;
                } else {
                    str_bucket = "0".to_owned() + &str_bucket;
                }
                val /= 2;
                i2 += 1;
            }
            str += &str_bucket;
            i += 1;
        }
        return str;
    }

    pub fn get_value(&self) -> Option<u64> {
        if self.values.len() != 1 {
            println!("Number of 64 bit values is: {}", self.values.len());
            println!("Size of bits is: : {}", self.get_size());
            //Err("Number doesn't fit into a 64-bit type. {}", self.get_size());
            return None;
        }
        Some(self.values[0])
    }

    pub fn get_size(&self) -> u32 {
        if self.values.is_empty() {
            return 0;
        }
        // Full buckets contain each 64 bits, last one contains only 'last_size_' bits.
        (self.values.len() as u32 - 1) * 64 + self.last_size
    }

    fn append_value(&mut self, value: u128, length: u32) {
        if length > 64 {
            println!("SPLITTING append_value");
            self.do_append_value((value >> 64) as u64, length - 64);
            self.do_append_value(value as u64, 64);
        } else {
            self.do_append_value(value as u64, length);
        }
    }

    fn do_append_value(&mut self, value: u64, length: u32) {
        // The last bucket is full or no bucket yet, create a new one.
        if self.values.is_empty() || self.last_size == 64 {
            self.values.push(value);
            self.last_size = length;
        } else {
            let free_bits = 64 - self.last_size;
            if self.last_size == 0 && length == 64 {
                // Special case for OSX -O3, as per -fsanitize=undefined
                // runtime error: shift exponent 64 is too large for 64-bit type 'uint64_t' (aka
                // 'unsigned long long')
                let len: usize = self.values.len() - 1;
                self.values[len] = value;
                self.last_size = length;
            } else if length <= free_bits {
                // If the value fits into the last bucket, append it all there.
                let len: usize = self.values.len() - 1;
                self.values[len] = (self.values[len] << length) + value;
                self.last_size += length;
            } else {
                // Otherwise, append the prefix into the last bucket, and create a new bucket for
                // the suffix.
                let (prefix, suffix) =
                    self.split_number_by_prefix(value, length as u8, free_bits as u8);
                let len: usize = self.values.len() - 1;
                self.values[len] = (self.values[len] << free_bits) + prefix;
                self.values.push(suffix);
                self.last_size = length - free_bits;
            }
        }
    }

    fn split_number_by_prefix(&self, number: u64, num_bits: u8, prefix_size: u8) -> (u64, u64) {
        assert!(num_bits >= prefix_size);
        if prefix_size == 0 {
            let prefix = 0;
            let suffix = number;
            return (prefix, suffix);
        }
        let suffix_size = num_bits - prefix_size;
        let mut mask = 1u64 << suffix_size;
        mask -= 1;
        let suffix = number & mask;
        let prefix = number >> suffix_size;
        (prefix, suffix)
    }

    fn get_size_bits(&self, value: u128) -> u8 {
        let mut val = value;
        let mut count = 0;
        while val > 0 {
            count += 1;
            val >>= 1;
        }
        count
    }
}
impl ops::Add<BitVec> for BitVec {
    type Output = BitVec;

    fn add(self, _rhs: BitVec) -> BitVec {
        let mut rtn = self.clone();
        if !_rhs.values.is_empty() {
            let mut i = 0;
            while i < _rhs.values.len() - 1 {
                rtn.append_value(_rhs.values[i] as u128, 64);
                i += 1;
            }
            rtn.append_value(_rhs.values[_rhs.values.len() - 1] as u128, _rhs.last_size);
        }
        rtn
    }
}
impl ops::AddAssign<BitVec> for BitVec {
    fn add_assign(&mut self, _rhs: BitVec) {
        if !_rhs.values.is_empty() {
            let mut i = 0;
            while i < _rhs.values.len() - 1 {
                self.append_value(_rhs.values[i] as u128, 64);
                i += 1;
            }
            self.append_value(_rhs.values[_rhs.values.len() - 1] as u128, _rhs.last_size);
        }
    }
}
impl ops::AddAssign<&BitVec> for BitVec {
    fn add_assign(&mut self, _rhs: &BitVec) {
        if !_rhs.values.is_empty() {
            let mut i = 0;
            while i < _rhs.values.len() - 1 {
                self.append_value(_rhs.values[i] as u128, 64);
                i += 1;
            }
            self.append_value(_rhs.values[_rhs.values.len() - 1] as u128, _rhs.last_size);
        }
    }
}
impl PartialEq for BitVec {
    fn eq(&self, other: &Self) -> bool {
        if self.get_size() != other.get_size() {
            return false;
        }
        let mut i = 0;
        while i < self.values.len() {
            if self.values[i] != other.values[i] {
                return false;
            }
            i += 1;
        }
        true
    }
}
impl Eq for BitVec {}

impl PartialOrd for BitVec {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.get_size() != other.get_size() {
            return None;
        }
        Some(self.cmp(other))
    }
}
impl Ord for BitVec {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.get_size() != other.get_size() {
            return self.get_size().cmp(&other.get_size());
        }
        let mut i = 0;
        while i < self.values.len() {
            if self.values[i] < other.values[i] {
                return Ordering::Less;
            }
            if self.values[i] > other.values[i] {
                return Ordering::Greater;
            }
            i += 1;
        }
        Ordering::Equal
    }
}
