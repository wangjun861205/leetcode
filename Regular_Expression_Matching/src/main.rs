struct Solution;

enum Pattern {
    None,
    SingleChar(char),
    RepeatChar(char),
    AnyChar,
    RepeatAny,
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut s_chars: Vec<char> = s.chars().collect();
        let mut p_chars: Vec<char> = p.chars().collect();
        let mut state: Pattern = Pattern::None;
        while !s_chars.is_empty() {
            match state {
                Pattern::None => match p_chars.len() {
                    0 => return false,
                    1 => match p_chars[0] {
                        c if c.is_ascii_lowercase() => {
                            state = Pattern::SingleChar(c);
                            p_chars.remove(0);
                        }
                        c if c == '.' => {
                            state = Pattern::AnyChar;
                            p_chars.remove(0);
                        }
                        _ => unreachable!(),
                    },
                    _ => match p_chars[0] {
                        c if c.is_ascii_lowercase() => {
                            if p_chars[1] == '*' {
                                state = Pattern::RepeatChar(c);
                                p_chars.drain(..2);
                            } else {
                                state = Pattern::SingleChar(c);
                                p_chars.remove(0);
                            }
                        }
                        c if c == '.' => {
                            if p_chars[1] == '*' {
                                state = Pattern::RepeatAny;
                                p_chars.drain(..2);
                            } else {
                                state = Pattern::AnyChar;
                                p_chars.remove(0);
                            }
                        }
                        _ => unreachable!(),
                    },
                },
                Pattern::SingleChar(c) => {
                    if s_chars[0] != c {
                        return false;
                    }
                    state = Pattern::None;
                    s_chars.remove(0);
                }
                Pattern::RepeatChar(c) => {
                    if s_chars[0] != c {
                        state = Pattern::None;
                    } else {
                        s_chars.remove(0);
                    }
                }
                Pattern::AnyChar => {
                    s_chars.remove(0);
                    state = Pattern::None;
                }
                Pattern::RepeatAny => {
                    if p_chars.is_empty() {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
        }
        p_chars.is_empty()
    }
}
fn main() {
    println!("{}", Solution::is_match("aa".to_owned(), "a".to_owned()));
}
