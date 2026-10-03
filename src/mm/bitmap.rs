const BITS_PER_WORD: usize = u64::BITS as usize;

// Macro to get the number of words required to track `n` elements.
#[macro_export]
macro_rules! bitmap_words {
    ($n:expr) => {
        ($n + 63) / 64
    };
}

pub struct Bitmap<const W: usize> {
    words: [u64; W],
}

impl<const W: usize> Bitmap<W> {
    pub const fn new() -> Self {
        Self { words: [0; W] }
    }

    pub fn set(&mut self, index: usize) {
        let word = index / BITS_PER_WORD;
        let bit = index % BITS_PER_WORD;

        self.words[word] |= 1 << bit;
    }

    pub fn clear(&mut self, index: usize) {
        let word = index / BITS_PER_WORD;
        let bit = index % BITS_PER_WORD;

        self.words[word] &= !(1 << bit);
    }

    pub fn is_set(&self, index: usize) -> bool {
        let word = index / BITS_PER_WORD;
        let bit = index % BITS_PER_WORD;

        self.words[word] & (1 << bit) != 0
    }

    pub fn find_zero(&self) -> Option<usize> {
        for (word_index, &word) in self.words.iter().enumerate() {
            if word != u64::MAX {
                for bit in 0..BITS_PER_WORD {
                    if word & 1 << bit == 0 {
                        return Some(word_index * BITS_PER_WORD + bit);
                    }
                }
            }
        }

        None
    }
}
