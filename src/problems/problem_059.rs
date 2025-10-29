use std::error::Error;
use std::fs::read_to_string;

#[test]
fn problem_059() -> Result<(), Box<dyn Error>> {
    let message = decrypt()?;
    let result = message.bytes().map(|x| x as i32).sum::<i32>();
    println!("{}", result);
    Ok(())
}

fn decrypt() -> Result<String, Box<dyn Error>> {
    let encrypted = read_to_string("src/problems/data/0059_cipher.txt")?;
    for a in 'a'..='z' {
        for b in 'a'..='z' {
            for c in 'a'..='z' {
                let decrypted = try_key(&encrypted, &mut Key::new(vec![a, b, c]))?;
                if decrypted.contains(" the ") && decrypted.contains(" and ") {
                    return Ok(decrypted)
                }
            }
        }
    }
    panic!()
}

fn is_valid(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | ' ' => true,
        _ => false,
    }
}

fn try_key(encrypted: &String, key: &mut Key) -> Result<String, Box<dyn Error>> {
    Ok(encrypted
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .map(|x| key.crypt(x))
        .map(|x| x as char)
        .collect())
}

struct Key {
    bytes: Vec<u8>,
    offset: usize,
}

impl Key {
    fn new(key: Vec<char>) -> Key {
        Self {
            bytes: key.into_iter().map(|x| x as u8).collect(),
            offset: 0,
        }
    }

    fn crypt(&mut self, byte: u8) -> u8 {
        let res = self.bytes[self.offset] ^ byte;
        self.offset += 1;
        self.offset %= self.bytes.len();
        res
    }
}