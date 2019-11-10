fn bottles(n: i32, cap: bool) -> String {
    if n > 1 {
        format!("{} bottles", n)
    }
    else if n == 1 {
        format!("{} bottle", n)
    }
    else {
        format!("{}o more bottles", if cap {"N"} else {"n"})
    }
}

fn beer(n: i32, cap: bool) -> String {
    format!("{} of beer", bottles(n, cap))
}

fn beer_on_the_wall(n: i32, cap: bool) -> String {
    format!("{} on the wall", beer(if n >= 0 {n} else {99}, cap))
}

fn beer_action(n: i32) -> String {
    if n > 0 {
        format!("Take {} down and pass it around",
                if n == 1 {"it"} else {"one"})
    }
    else {
        "Go to the store and buy some more".to_string()
    }
}

pub fn verse(n: i32) -> String {
    format!("{}, {}.\n{}, {}.\n",
            beer_on_the_wall(n, true), beer(n, false),
            beer_action(n), beer_on_the_wall(n-1, false))
}

pub fn sing(start: i32, end: i32) -> String {
    let mut res = String::new();    
    for i in (end..=start).rev() {
        res.push_str(&verse(i));
        if i != end {
            res.push_str("\n");
        }
    }
    return res;
}
