#[derive(PartialEq)]
enum Brace {
    Opened(BraceType),
    Closed(BraceType),
}

#[derive(PartialEq)]
enum BraceType {
    Round,
    Curly,
    Square,
}

fn brace(ch: char) -> Option<Brace> {
    match ch {
        '(' => Some(Brace::Opened(BraceType::Round)),
        '{' => Some(Brace::Opened(BraceType::Curly)),
        '[' => Some(Brace::Opened(BraceType::Square)),
        ')' => Some(Brace::Closed(BraceType::Round)),
        '}' => Some(Brace::Closed(BraceType::Curly)),
        ']' => Some(Brace::Closed(BraceType::Square)),
        _ => None,
    }
}

pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<BraceType> = vec![];

    for ch in s.chars() {
        if let Some(brace) = brace(ch) {
            match brace {
                Brace::Opened(t) => stack.push(t),
                Brace::Closed(t) => {
                    if let Some(opened) = stack.pop() {
                        if t != opened {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example4() {
        do_test("([])".to_owned(), true);
    }

    #[test]
    fn example3() {
        do_test("(]".to_owned(), false);
    }

    #[test]
    fn example2() {
        do_test("()[]{}".to_owned(), true);
    }

    #[test]
    fn example1() {
        do_test("()".to_owned(), true);
    }

    fn do_test(s: String, expected: bool) {
        assert_eq!(is_valid(s), expected);
    }
}
