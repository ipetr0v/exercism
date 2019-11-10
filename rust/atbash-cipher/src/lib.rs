use std::iter::once;

fn cipher_char(c: char) -> Option<char> {
    match c {
        c if c.is_ascii_alphabetic() => Some((b'a' + b'z' - c as u8) as char),
        c if c.is_ascii_alphanumeric() => Some(c),
        _ => None
    }
}

fn cipher(text: &str) -> impl Iterator<Item=char> + '_ {
     text.chars()
         .filter_map(|x| cipher_char(x.to_ascii_lowercase()))
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    cipher(plain).enumerate()
                 .flat_map(|(i, x)| {
                     Some(' ').filter(|_| i > 0 && i % 5 == 0)
                              .into_iter()
                              .chain(once(x))
                 })
                 .collect::<String>()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(encrypted: &str) -> String {
    cipher(encrypted).collect::<String>()
}
