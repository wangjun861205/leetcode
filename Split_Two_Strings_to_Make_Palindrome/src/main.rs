struct Solution;

impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        if a.len() % 2 == 0 {
            let mut a_border = vec![false; a.len() / 2];
            for i in 0..a.len() / 2 {
                if a_chars[i] == b_chars[a.len() - i - 1] {
                    a_border[i] = true;
                } else {
                    break;
                }
            }
            if *a_border.last().unwrap() {
                return true;
            }
            let mut b_border = vec![false; a.len() / 2];
            for i in 0..a.len() / 2 {
                if b_chars[i] == a_chars[a.len() - i - 1] {
                    b_border[i] = true;
                } else {
                    break;
                }
            }
            if *b_border.last().unwrap() {
                return true;
            }
            let mut left = a.len() / 2 - 1;
            let mut right = a.len() / 2;
            loop {
                if a_chars[left] == a_chars[right] {
                    if left == 0 {
                        return true;
                    }
                    if a_border[left - 1] || b_border[left - 1] {
                        return true;
                    }
                    left -= 1;
                    right += 1;
                } else {
                    break;
                }
            }
            let mut left = a.len() / 2 - 1;
            let mut right = a.len() / 2;
            loop {
                if b_chars[left] == b_chars[right] {
                    if left == 0 {
                        return true;
                    }
                    if a_border[left - 1] || b_border[left - 1] {
                        return true;
                    }
                    left -= 1;
                    right += 1;
                } else {
                    break;
                }
            }
        } else {
            let mut a_border = vec![false; a.len() / 2 + 1];
            for i in 0..=a.len() / 2 {
                if a_chars[i] == b_chars[a.len() - i - 1] {
                    a_border[i] = true;
                } else {
                    break;
                }
            }
            if *a_border.last().unwrap() {
                return true;
            }
            let mut b_border = vec![false; a.len() / 2 + 1];
            for i in 0..=a.len() / 2 {
                if b_chars[i] == a_chars[a.len() - i - 1] {
                    b_border[i] = true;
                } else {
                    break;
                }
            }
            if *b_border.last().unwrap() {
                return true;
            }
            let mut left = a.len() / 2;
            let mut right = a.len() / 2;
            loop {
                if a_chars[left] == a_chars[right] {
                    if left == 0 {
                        return true;
                    }
                    if a_border[left - 1] || b_border[left - 1] {
                        return true;
                    }
                    left -= 1;
                    right += 1;
                } else {
                    break;
                }
            }
            let mut left = a.len() / 2;
            let mut right = a.len() / 2;
            loop {
                if b_chars[left] == b_chars[right] {
                    if left == 0 {
                        return true;
                    }
                    if a_border[left - 1] || b_border[left - 1] {
                        return true;
                    }
                    left -= 1;
                    right += 1;
                } else {
                    break;
                }
            }
        }
        false
    }
}

fn main() {
    println!(
        "{}",
        Solution::check_palindrome_formation("khekdvakkkggbopatnbtcbbqkntplzoqectgexifhinhsjohplfebkynpxkraayuythwgbwvzqzprhapxgevfnmexkkutaybuspnmkztgxryipgxlowdsnmqlsslnxupsvsbttxdlgvjvrbxnqezowacfplqkzubwduirbgmjzkdmpwctoowzcsjbaoiumsthvgcagvsihjqgbfjejhtspyrdsmoabvmrgmtshraxgmwknmijgypvgmgfqcytumqcqhgiuihbkcrcehnglsoxegdailsjlibsfnyeoejeltxsvtubakuvskokudtgkbhaab".to_owned(),
"baahbkgtdukoksvukabutvsxtlejeoeynfsbilqwuqnbpyzmlttjzewwcgvcmaqlsosagpztvpbbxkxsclcashgzstktuuernbmymfkfpalyprnmzeakyyruodavblsyxohctqzcknefhucfdpsntixoczieytxeaqaextyeizcoxitnspdfcuhfenkczqtchoxyslbvadouryykaezmnrpylapfkfmymbnreuutktszghsaclcsxkxbbpvtzpgasoslqamcvgcwwezjttlmzypbnquwqhgcqjqzjrfhcfloqdrpvggnupsizifdzeqpvbz".to_owned())
    );
}
