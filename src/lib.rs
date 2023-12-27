use std::{collections::HashMap, marker::PhantomData};

/// A struct responsible for efficiently interning strings
// pub struct Interner<I, H = DefaultHasher> {
// TODO: allow plugging in a custom hasher.
pub struct Interner<I> {
    map: HashMap<&'static str, u32>,
    vec: Vec<&'static str>,
    buf: String,
    full: Vec<String>,
    _phantom_i: PhantomData<I>,
    // _phantom_h: PhantomData<H>,
}

impl<I> Interner<I>
where
    I: From<u32>,
{
    pub fn with_capacity(cap: usize) -> Self {
        Interner {
            map: Default::default(),
            vec: Vec::new(),
            buf: String::with_capacity(cap.next_power_of_two()),
            full: Vec::new(),
            _phantom_i: PhantomData,
            // _phantom_h: PhantomData,
        }
    }

    pub fn intern(&mut self, name: &str) -> I {
        match self.map.get(name) {
            Some(id) => (*id).into(),
            None => {
                let name = self.alloc(name);
                let id = self.map.len() as u32;
                self.map.insert(name, id);

                #[cfg(test)]
                println!("{:#?}", self.map);

                self.vec.push(name);

                id.into()
            }
        }
    }

    fn alloc(&mut self, name: &str) -> &'static str {
        let cap = self.buf.capacity();
        if cap < self.buf.len() + name.len() {
            let new_cap = (cap.max(name.len()) + 1).next_power_of_two();
            let new_buf = String::with_capacity(new_cap);
            let old_buf = std::mem::replace(&mut self.buf, new_buf);
            self.full.push(old_buf);
        }
        let interned = {
            let start = self.buf.len();
            self.buf.push_str(name);
            &self.buf[start..]
        };

        unsafe { &*(interned as *const str) }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn intern() {
        use super::Interner;
        const TEST_CASE: &str = "test_case";
        const DUMMY_ONE: &str = "dummy_one";
        const DUMMY_TWO: &str = "dummy_two";
        const DUMMY_THREE: &str = "dummy_three";

        let mut interner = <Interner<u32>>::with_capacity(4);
        _ = interner.intern(DUMMY_ONE);
        _ = interner.intern(DUMMY_TWO);
        _ = interner.intern(DUMMY_THREE);
        _ = interner.intern(TEST_CASE);
        assert_eq!(interner.intern(TEST_CASE), 3)
    }
}
