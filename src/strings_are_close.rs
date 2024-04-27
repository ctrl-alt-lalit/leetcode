struct AsciiMap {
    arr: [i32; 26]
}

impl AsciiMap {
    fn new() -> Self {
        return Self {
            arr: [0; 26]
        }
    }

    fn get(&self, c: char) -> i32 {
        return self.arr[(c as u8 - b'a') as usize];
    }

    fn increment(&mut self, c: char) {
        self.arr[(c as u8 - b'a') as usize] += 1;
    }
}

pub fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        return false;
    }
    // Swap operation == count of each letter is the same
    // Transform operation == count of x equal count of y

    let mut cc1 = AsciiMap::new();
    let mut cc2 = AsciiMap::new();
    
    word1.chars().into_iter().for_each(|c| cc1.increment(c) );
    word2.chars().into_iter().for_each(|c| cc2.increment(c) );

    for (v1, v2) in cc1.arr.iter().zip(cc2.arr.iter()) {
        if v1.is_positive() != v2.is_positive() {
            return false;
        }
    }

    cc1.arr.sort_unstable();
    cc2.arr.sort_unstable();
    return cc1.arr.into_iter().zip(cc2.arr.into_iter()).all(|(x,y)| x == y);
}