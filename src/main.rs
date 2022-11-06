use std::fs::File;
use std::collections::HashMap;
use std::io::Write;
use once_cell::sync::Lazy;
use indicatif::{ProgressIterator, ProgressStyle};


static ATOMIC_LENGTH: Lazy<HashMap<usize, usize>> = Lazy::new(|| {
    HashMap::from([
        (0  , 0),
        (1  , "one".len()),
        (2  , "two".len()),
        (3  , "three".len()),
        (4  , "four".len()),
        (5  , "five".len()),
        (6  , "six".len()),
        (7  , "seven".len()),
        (8  , "eight".len()),
        (9  , "nine".len()),
        (10 , "ten".len()),
        (11 , "eleven".len()),
        (12 , "twelve".len()),
        (13 , "thirteen".len()),
        (14 , "fourteen".len()),
        (15 , "fifteen".len()),
        (16 , "sixteen".len()),
        (17 , "seventeen".len()),
        (18 , "eighteen".len()),
        (19 , "nineteen".len()),
        (20 , "twenty".len()),
        (30 , "thirty".len()),
        (40 , "forty".len()),
        (50 , "fifty".len()),
        (60 , "sixty".len()),
        (70 , "seventy".len()),
        (80 , "eighty".len()),
        (90 , "ninety".len())
    ])
});

static MAGNITUDE: Lazy<HashMap<usize, usize>> = Lazy::new(|| {
    HashMap::from([
        (0 , "hundred".len()),
        (1 , "thousand".len()),
        (2 , "million".len()),
        (3 , "billion".len()),
        (4 , "trillion".len()),
        (5 , "quadrillion".len())
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
        total += ATOMIC_LENGTH[&num_hundreds] + MAGNITUDE[&0];
    }

    let under_hundred = n % 100;
    if ATOMIC_LENGTH.contains_key(&under_hundred) {
        total += ATOMIC_LENGTH[&under_hundred];
    } else {
        total += ATOMIC_LENGTH[&(under_hundred / 10 * 10)] + ATOMIC_LENGTH[&(under_hundred % 10)];
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
            total += MAGNITUDE[&i];
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
