struct Solution;

#[derive(Debug, PartialEq, Eq)]
enum State {
    None,
    Sign,
    SignedInteger,
    UnsignedInteger,
    Point,
    Decimal,
    Exponent,
    ExponentSign,
    SignedExponent,
    UnsignedExponent,
}
impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut state = State::None;
        for c in s.chars() {
            match c {
                '+' | '-' => match state {
                    State::None => state = State::Sign,
                    State::Exponent => state = State::ExponentSign,
                    _ => return false,
                },
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => match state {
                    State::None => state = State::UnsignedInteger,
                    State::Sign => state = State::SignedInteger,
                    State::Point => state = State::Decimal,
                    State::Exponent => state = State::UnsignedExponent,
                    State::ExponentSign => state = State::SignedExponent,
                    _ => {}
                },
                'e' | 'E' => match state {
                    State::SignedInteger | State::UnsignedInteger | State::Decimal => {
                        state = State::Exponent
                    }
                    _ => return false,
                },
                '.' => match state {
                    State::None | State::Sign => {
                        state = State::Point;
                    }
                    State::SignedInteger | State::UnsignedInteger => {
                        state = State::Decimal;
                    }
                    _ => return false,
                },
                _ => {
                    return false;
                }
            }
        }
        state != State::Point
            && state != State::Sign
            && state != State::Exponent
            && state != State::ExponentSign
    }
}
fn main() {
    println!("Hello, world!");
}
