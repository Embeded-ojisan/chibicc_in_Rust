/*
参考

https://qiita.com/penguinshunya/items/cd96803b74635aebefd6

https://what-alnk.hatenablog.com/entry/2017/07/31/214844
*/
use std::io::stdin;
use std::str::FromStr;


pub fn read_single_String() -> String {
    read::<String>()
}

/*
pub fn read_and_return_Iter() -> impl Iterator<Item = usize> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    let iter = 
        buf
        .split_whitespace()
        .map(|n| usize::from_str(n).unwrap());
    iter.clone()
}
*/

pub fn read_double_turple_usize() -> (usize, usize) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    let mut it = 
        buf
            .split_whitespace()
            .map(|n| usize::from_str(n).unwrap());

    (it.next().unwrap(), it.next().unwrap())
}


pub fn read_single_usize() -> usize {
    read::<usize>()
}

pub fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .parse()
        .ok()
        .unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    let mut v2 = Vec::new();
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin()
            .read_line(&mut s)
            .ok();
        let v = s.trim()
                    .split_whitespace()
                    .map(|e| e.parse().ok().unwrap())
                    .collect();
        v2.push(v);
    }
    v2
}

/*
pub fn read_usize_block() -> usize {
    ;
}
*/