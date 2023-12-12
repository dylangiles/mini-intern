const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

#[allow(dead_code)]
pub(crate) fn fnv_hash(i: &[u8]) -> u64 {
    let mut h = FNV_OFFSET_BASIS;
    i.iter()
        .map(|b| {
            (h, _) = h.overflowing_mul(FNV_PRIME);
            h = h ^ (*b as u64);
        })
        .for_each(drop);
    h
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::fnv_hash;
        assert_eq!(fnv_hash("hello world".as_bytes()), 0x7dcf62cdb1910e6f)
    }
}
