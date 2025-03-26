use crate::shuffle::shuffle;

pub fn encrypt(length: usize, seed: &str) -> Vec<(usize, usize)> {
    if length == 0 {return vec![];}
    if length == 1 {return vec![(0, 1)];}
    let mut result = vec![];
    let count = length - 2;
    let array = shuffle(count, seed);
    let mut map = vec![0usize; count];
    for i in 0..count {map[array[i]] = i;}
    result.push((0, 2));
    for i in 0..count {result.push((map[i], 3));}
    result.push((length - 2, 2));
    result
}

pub fn decrypt(length: usize, seed: &str) -> Vec<usize> {
    if length == 0 {return vec![];}
    if length == 1 {return vec![0];}
    let mut result = vec![];
    let count = (length - 4) / 3;
    let array = shuffle(count, seed);
    result.push(0);
    for i in 0..count {result.push(array[i] * 3 + 3);}
    result.push(length - 1);
    result
}

pub const fn n2m(n: usize) -> usize {
    if n == 0 {return 0;}
    if n == 1 {return 1;}
    n * 3 - 2
}