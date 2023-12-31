mod impl1 {
    use std::collections::HashMap;

    pub struct LengthOfLongestSubstring<'a> {
        s: &'a str,
        left: usize,
        right: usize,
        largest: usize,
        buf: HashMap<&'a str, i32>,
        dublicate: Option<&'a str>,
    }

    impl<'a> LengthOfLongestSubstring<'a> {
        pub fn new(s: &str) -> LengthOfLongestSubstring<'_> {
            LengthOfLongestSubstring {
                s,
                left: 0,
                right: 0,
                largest: 0,
                buf: HashMap::new(),
                dublicate: None,
            }
        }

        pub fn compute(mut self) -> i32 {
            if self.s.is_empty() {
                return 0;
            }

            loop {
                self.progress_right();
                self.progress_left();
                if self.right == self.s.len() {
                    return self.largest as i32;
                }
            }
        }

        fn ch_at_idx(&self, idx: usize) -> &'a str {
            &self.s[idx..idx + 1]
        }

        /// Progresses right index till dublicate appears and updates self.dublicate with it.
        /// Inserts new chars into self.buf.
        /// Updates self.largest if needed.
        fn progress_right(&mut self) {
            while self.right < self.s.len() {
                let ch = self.ch_at_idx(self.right);
                self.right += 1;
                let entry = self.buf.entry(ch);
                let count = *entry.and_modify(|i| *i += 1).or_insert(1);

                let dublicate_appeared = count > 1;
                if dublicate_appeared {
                    self.dublicate = Some(ch);
                    return;
                } else {
                    self.largest = std::cmp::max(self.largest, self.right - self.left);
                }
            }
        }

        /// Progresses left index till dublicate disappears and self.dublicate to None after it.
        /// Removes old chars from self.buf.
        fn progress_left(&mut self) {
            if let Some(dublicate) = dbg!(self.dublicate.take()) {
                while self.left < self.right - 1 {
                    let ch = dbg!(self.ch_at_idx(self.left));
                    self.left += 1;

                    let v = self.buf.get_mut(ch).unwrap();
                    *v -= 1;
                    let dublicate_disappeared = *v == 1 && ch == dublicate;

                    if *v == 0 {
                        self.buf.remove(ch);
                    }

                    if dublicate_disappeared {
                        self.dublicate = None;
                        return;
                    }
                }
            }
        }
    }
}

mod impl2 {
    use std::collections::HashMap;

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_seen_idxs: HashMap<char, usize> = HashMap::new();
        let mut longest: usize = 0;
        let mut start_idx: usize = 0;
        let mut len = 0; // we don't want to iterate over chars again to get length
        for (cur_idx, ch) in s.chars().enumerate() {
            if let Some(last_seen_idx) = last_seen_idxs.insert(ch, cur_idx) {
                longest = longest.max(cur_idx - start_idx);
                start_idx = start_idx.max(last_seen_idx + 1);
            }
            len += 1;
        }
        longest.max(len - start_idx) as i32
    }
}

pub fn length_of_longest_substring(s: String) -> i32 {
    impl2::length_of_longest_substring(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example6() {
        do_test("abba".to_owned(), 2)
    }

    #[test]
    fn example5() {
        do_test("dvdf".to_owned(), 3)
    }

    #[test]
    fn example4() {
        do_test("aab".to_owned(), 2)
    }

    #[test]
    fn example3() {
        do_test("pwwkew".to_owned(), 3)
    }

    #[test]
    fn example2() {
        do_test("bbbbb".to_owned(), 1)
    }

    #[test]
    fn example1() {
        do_test("abcabcbb".to_owned(), 3)
    }

    fn do_test(s: String, expected: i32) {
        assert_eq!(length_of_longest_substring(s), expected);
    }
}
