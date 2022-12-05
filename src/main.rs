use std::fs::File;
use std::collections::HashMap;
use std::io::Write;
use once_cell::sync::Lazy;
use indicatif::{ProgressIterator, ProgressStyle};


static ATOMIC_LENGTH: Lazy<HashMap<usize, &str>> = Lazy::new(|| {
    HashMap::from([
        (0  , ""),
        (1  , "one"),
        (2  , "two"),
        (3  , "three"),
        (4  , "four"),
        (5  , "five"),
        (6  , "six"),
        (7  , "seven"),
        (8  , "eight"),
        (9  , "nine"),
        (10 , "ten"),
        (11 , "eleven"),
        (12 , "twelve"),
        (13 , "thirteen"),
        (14 , "fourteen"),
        (15 , "fifteen"),
        (16 , "sixteen"),
        (17 , "seventeen"),
        (18 , "eighteen"),
        (19 , "nineteen"),
        (20 , "twenty"),
        (30 , "thirty"),
        (40 , "forty"),
        (50 , "fifty"),
        (60 , "sixty"),
        (70 , "seventy"),
        (80 , "eighty"),
        (90 , "ninety")
    ])
});

static MAGNITUDE: Lazy<HashMap<usize, &str>> = Lazy::new(|| {
    HashMap::from([
        (0 , "hundred"),
        (1 , "thousand"),
        (2 , "million"),
        (3 , "billion"),
        (4 , "trillion"),
        (5 , "quadrillion")
    ])
});

static PRELOAD: Lazy<HashMap<usize, usize>> = Lazy::new(|| {
    let mut map = HashMap::with_capacity(1000);
    for i in 0..1000 {
        map.insert(i, preload_sum(i));
    }
    return map;
});

fn preload_sum(n: usize) -> usize {
    let mut total = 0;

    let num_hundreds = n / 100;
    if num_hundreds > 0 {
        total += ATOMIC_LENGTH[&num_hundreds].len() + MAGNITUDE[&0].len();
    }

    let under_hundred = n % 100;
    if ATOMIC_LENGTH.contains_key(&under_hundred) {
        total += ATOMIC_LENGTH[&under_hundred].len();
    } else {
        total += ATOMIC_LENGTH[&(under_hundred / 10 * 10)].len() + ATOMIC_LENGTH[&(under_hundred % 10)].len();
    }

    return total;
}

fn is_num(num: usize) -> usize {
    let mut curr = num;
    let mut total = 0;
    let mut i = 0;

    while curr != 0 {
        let group = curr % 1000;
        total += PRELOAD[&group];
        if i > 0 && group > 0 {
            total += MAGNITUDE[&i].len();
        }

        i += 1;
        curr = curr / 1000;
    }

    return total;
}

fn cosmic_chain(num: usize) -> String {
    let mut curr = num;
    let mut text = format!("num={curr}\n");

    while curr != 4 {
        let next = is_num(curr);
        text.push_str(&format!("{curr} is {next}\n"));
        curr = next;
    }
    text.push_str("4 is cosmic\n----\n");
    return text;
}

fn main() -> std::io::Result<()> {
    let mut file = File::create("cosmic.log")?;
    let style = ProgressStyle::with_template("eta:{eta} {bar:40.white} {pos:>7}/{len:7} {msg} [{elapsed}]")
        .unwrap()
        .progress_chars("##-");

    for i in (1..1_000_001).progress_with_style(style) {
        file.write(cosmic_chain(i).as_bytes())?;
    }
    Ok(())
}
