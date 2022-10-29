
use std::collections::HashMap;
use indicatif::ProgressBar;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref ATOMIC_LENGTH: HashMap<i32, i32> =
        HashMap::from([
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

    static ref MAGNITUDE: HashMap<i32, i32> = 
        HashMap::from([
            (0 , 7), // hundred
            (1 , 8), // thousand
            (2 , 7), // million
            (3 , 7), // billion
            (4 , 8), // trillion
            (5 , 11) // quadrillion
        ]);

    static ref PRELOAD: HashMap<i32, i32> = {
        let mut map = HashMap::new();
        for i in 0..1000 {
            map.insert(i, preload_sum(i));
        }
        return map;
    };
}

fn preload_sum(n: i32) -> i32 {
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

fn is_num(num: i32) -> i32 {
    let mut curr = num;
    let mut total = 0;
    let mut i = 0;

    while curr != 0 {
        let group = curr % 1000;
        total += PRELOAD[&group];
        if i > 0 && group > 0 {
            total += MAGNITUDE[&i];
        }

        i += 1;
        curr = curr / 1000;
    }
    
    return total;
}

fn cosmic_chain(num: i32) {
    let mut curr = num;
    let mut next = is_num(curr);
    let mut text = format!("num={curr}\n");

    while curr != 4 {
        text.push_str(&format!("{curr} is {next}\n"));
        curr = next;
        next = is_num(curr);
    }
    text.push_str("4 is cosmic\n----");
    println!("{text}");
}

fn main() {
    let pb = ProgressBar::new(1000000);
    for i in 1..1_000_001 {
        cosmic_chain(i);
        pb.inc(1);
    }
    pb.finish_with_message("done!")
}
