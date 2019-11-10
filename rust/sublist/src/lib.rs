#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn sublist_of<T: PartialEq>(sublist: &[T], superlist: &[T]) -> bool {
    sublist.is_empty()
        || superlist.windows(sublist.len()).any(|x| x == sublist)
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    match (first, second) {
        (x, y) if x == y => Comparison::Equal,
        (x, y) if sublist_of(x, y) => Comparison::Sublist,
        (x, y) if sublist_of(y, x) => Comparison::Superlist,
        _ => Comparison::Unequal
    }
}
