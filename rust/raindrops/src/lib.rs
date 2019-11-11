const SOUND_MAP: [(u32, &str); 3] = [
    (3, "Pling"),
    (5, "Plang"),
    (7, "Plong")
];

pub fn raindrops(n: u32) -> String {
    let out = SOUND_MAP
        .iter()
        .filter_map(|(d, s)| {
            if n % d == 0 {
                Some(s.chars())
            } else {
                None
            }
        })
        .flatten()
        .collect::<String>();

    if out.is_empty() {
        n.to_string()
    } else {
        out
    }
}
