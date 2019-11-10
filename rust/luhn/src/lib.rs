fn accumulate((len, sum): (usize, u32), value: char) -> Option<(usize, u32)> {
    value.to_digit(10).map(|x| if len % 2 == 1 {x*2} else {x})
                      .map(|x| if x > 9 {x-9} else {x})
                      .map(|x| (len + 1, sum + x))
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), accumulate)
        .map_or(false, |(len, sum)| len > 1 && sum % 10 == 0)
}
