struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let s = x.to_string();
        let mut s_iter: std::str::Chars<'_> = s.chars();
        loop {
            match (s_iter.next(), s_iter.next_back()) {
                (None, None) => return true,
                (Some(l), Some(r)) if l != r => return false,
                (Some(_), Some(_)) => continue,
                (Some(_), None) => return true,
                _ => return false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($test_name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                assert_eq!(Solution::is_palindrome($input), $expected);
            }
        };
    }

    test!(t01, 1, true);
    test!(t02, 121, true);
    test!(t03, 12321, true);
    test!(t04, 12, false);
    test!(t05, 21, false);
    test!(t06, 1210, false);
    test!(t07, -1, false);
    test!(t08, -121, false);
    test!(t09, -12321, false);
    test!(t10, -12, false);
    test!(t11, -21, false);
}