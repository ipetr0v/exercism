use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;

fn count_freq(input: String) -> HashMap<char, usize> {
    input.to_lowercase()
         .chars()
         .filter(|c| c.is_alphabetic())
         .fold(HashMap::new(), |mut acc, c| {
            acc.entry(c)
               .and_modify(|x| *x += 1)
               .or_insert(1);
            acc
         })
}

fn accumulate(mut acc: HashMap<char, usize>, res: HashMap<char, usize>) -> HashMap<char, usize> {
    res.iter()
       .for_each(|(&c, &count)| {
           acc.entry(c)
              .and_modify(|x| *x += count)
              .or_insert(count);
       });
    acc
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let (tx, rx) = mpsc::channel();
    input.chunks(input.len() / worker_count + 1)
         .map(|chunk| chunk.concat())
         .map(|chunk| {
             let thread_tx = tx.clone();
             thread::spawn(move || thread_tx.send(count_freq(chunk)))
         })
         .try_for_each(|handle| handle.join().unwrap());
    
    drop(tx);
    rx.iter()
      .fold(HashMap::new(), accumulate)
}
