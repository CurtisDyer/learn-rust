use std::collections::HashMap;
use std::fmt;

fn fib(n: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if cache.is_empty() {
        cache.extend([
            (0, 0),
            (1, 1),
            (2, 1),
        ]);
    }

    if let Some(n) = cache.get(&n) {
        return *n;
    }
    else {
        let term = fib(n - 2, cache) + fib(n - 1, cache);
        cache.insert(n, term);

        return term;
    }
}

fn word_list(src: &str) -> Vec<&str> {
    let mut list: Vec<&str> = vec![];
    let mut pos: usize = 0;

    for (i, &ch) in src.as_bytes().iter().enumerate() {
        if ch == b' ' {
            list.push(&src[pos..i]);
            pos = i + 1;
        }
    }

    if pos < src.len() {
        list.push(&src[pos..]);
    }

    return list;
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rect(w: {}; h: {})", self.width, self.height)
    }
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

fn main() {
    println!("Fibonacci");
    for foo in 0..=10 {
        println!("fib({foo}) = {}", fib(foo, &mut HashMap::new()));
    }

    println!("\nWords");
    for w in word_list("this is a sentence") {
        println!(" * Word: '{w}'");
    }

    println!("\nStruct Tests");
    let r1 = Rectangle::new(74, 69);
    let r2 = Rectangle::new(84, 69);
    let r3 = Rectangle::square(77);

    assert!(r1.width());
    assert!(r2.width());

    println!("Can {} fit {}? Ans: {}\nVice versa? Ans: {}", r1, r2, r1.can_hold(&r2), r2.can_hold(&r1));
    println!("Can {} hold {}? Ans: {}", r2, r3, r2.can_hold(&r3));
}