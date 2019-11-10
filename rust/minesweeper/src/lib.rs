const DIRS: [(isize, isize); 8] = [
    (-1,-1), (0,-1), (1,-1),
    (-1, 0),         (1, 0),
    (-1, 1), (0, 1), (1, 1)
];

fn is_mine(mines: &[&str], i: usize, j: usize) -> bool {
    mines.get(i)
         .map(|s| s.chars().nth(j) == Some('*'))
         .unwrap_or(false)
}

fn count_mines(mines: &[&str], i: usize, j: usize) -> u8 {
    DIRS.iter()
        .map(|&(x,y)| (x + i as isize, y + j as isize))
        .filter(|&(x,y)| x >=0 && y >= 0)
        .fold(0, |sum, (x,y)| sum + is_mine(&mines, x as usize, y as usize) as u8)
}

fn place_elem(mines: &[&str], i: usize, j: usize, ch: char) -> String {
    match (ch, count_mines(&mines, i, j)) {
        (' ', 0) => ' '.to_string(),
        (' ', x) => x.to_string(),
        _ => ch.to_string()
    }
}

pub fn annotate(mines: &[&str]) -> Vec<String> {
    mines.iter()
         .enumerate()
         .map(|(i, line)| {
             line.chars()
                 .enumerate()
                 .map(|(j, ch)| place_elem(&mines, i, j, ch))
                 .collect::<String>()
         })
         .collect::<Vec<String>>()
}
