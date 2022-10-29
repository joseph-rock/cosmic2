
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref ATOMIC_LENGTH: HashMap<i32, i32> = {
        return HashMap::from([
            (0  , 0),
            (1  , 3),
            (2  , 3),
            (3  , 5),
            (4  , 4),
            (5  , 4),
            (6  , 3),
            (7  , 5),
            (8  , 5),
            (9  , 4),
            (10 , 3),
            (11 , 6),
            (12 , 6),
            (13 , 8),
            (14 , 8),
            (15 , 7),
            (16 , 7),
            (17 , 9),
            (18 , 8),
            (19 , 7),
            (20 , 6),
            (30 , 6),
            (40 , 5),
            (50 , 5),
            (60 , 5),
            (70 , 7),
            (80 , 6),
            (90 , 6)
        ]);
    };

    static ref MAGNITUDE: HashMap<i32, i32> = {
        return HashMap::from([
            (0 , 7), // hundred
            (1 , 8), // thousand
            (2 , 7), // million
            (3 , 7), // billion
            (4 , 8), // trillion
            (5 , 11) // quadrillion
        ]);
    };
}

fn num_groups(num: i32) -> Vec<i32> {
    let mut groups: Vec<i32> = vec![];
    let mut curr = num;

    while curr != 0 {
        groups.push(curr % 1000);
        curr = curr / 1000;
    }
    
    return groups;
}

fn total_group(n: i32) -> i32 {
    let mut total: i32 = 0;

    let num_hundreds: i32 = n / 100;
    if num_hundreds > 0 {
        total += ATOMIC_LENGTH[&num_hundreds] + MAGNITUDE[&0];
    }

    let under_hundred: i32 = n % 100;
    if ATOMIC_LENGTH.contains_key(&under_hundred) {
        total += ATOMIC_LENGTH[&under_hundred];
    } else {
        total += ATOMIC_LENGTH[&(&under_hundred / &10 * &10)] + ATOMIC_LENGTH[&(&under_hundred % &10)];
    }

    return total;
}

fn is_num(n: i32) -> i32 {
    let groups: Vec<i32> = num_groups(n);
    let mut total = 0;
    let mut i = 0;

    for group in groups {
        total += total_group(group);

        if i > 0 && group > 0 {
            total += MAGNITUDE[&i];
        }
        i += 1;
    }

    return total;
}

fn cosmic_chain(num: i32) {
    let mut curr = num;
    let mut next = is_num(curr);

    while curr != 4 {
        println!("{curr} is {next}");
        curr = next;
        next = is_num(curr);
    }
    println!("4 is cosmic");
    println!("----")
}

fn main() {
    for i in 1..1000001 {
        cosmic_chain(i);
    }
}
