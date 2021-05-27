extern crate rand;

use rand::distributions::Alphanumeric;
use rand::Rng;
use std::collections::HashMap;

struct Codec {
    rnd: rand::rngs::ThreadRng,
    map: HashMap<String, String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {
            rnd: rand::thread_rng(),
            map: HashMap::new(),
        }
    }

    // Encodes a URL to a shortened URL.
    fn encode(&mut self, longURL: String) -> String {
        loop {
            let s: String = format!(
                "http://tinyurl.com/{}",
                self.rnd
                    .clone()
                    .sample_iter(&Alphanumeric)
                    .take(6)
                    .map(char::from)
                    .collect::<String>()
            );
            if !self.map.contains_key(&s) {
                self.map.insert(s.clone(), longURL);
                return s;
            }
        }
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        self.map.get(&shortURL).unwrap().to_owned()
    }
}

fn main() {
    let mut codec = Codec::new();
    let short = codec.encode("http://www.google.com/".to_owned());
    println!("short url: {}", short);
    let long = codec.decode(short);
    println!("long url: {}", long);
}
