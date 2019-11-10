fn isqrt(n: u32) -> u32 {
    (n as f64).sqrt() as u32 + 1
}

fn iln(n: u32) -> u32 {
    (n as f64).ln() as u32 + 1
}

fn get_sieve_len(n: u32) -> u32 {
    if n >= 6 { 
        n * iln(n * iln(n))
    } else {
        n * n + 2
    }
}

fn create_sieve(len: u32) -> Vec<bool> {
    let mut sieve: Vec<bool> = vec![true; len as usize];
    sieve[0] = false;
    sieve[1] = false;
    return sieve;
}

pub fn nth(n: u32) -> u32 {
    let sieve_len = get_sieve_len(n+1);
    let mut sieve = create_sieve(sieve_len);

    for i in 1..isqrt(sieve_len) as usize {
        if sieve[i] == true {
            for j in (i*i..sieve_len as usize).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    let primes = sieve.iter()
                      .enumerate()
                      .filter(|&(_, _val)| *_val == true)
                      .map(|(_idx, _)| _idx as u32)
                      .collect::<Vec<u32>>();

    return primes[n as usize];
}
