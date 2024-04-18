use std::collections::*;
use std::io::stdin;

struct Cin {
    tokens: VecDeque<String>,
}

impl Cin {
    pub fn new() -> Self {
        let tokens = VecDeque::new();
        Self { tokens }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        if self.tokens.is_empty() {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            for s in buffer.split_whitespace() {
                self.tokens.push_back(s.to_string());
            }
        }
        let fr = self.tokens.pop_front().unwrap();
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();
    let t = cin.next::<i64>();

    for _ in 0..t {
        let n = cin.next::<usize>();

        let mut a = vec![];

        let mut ones = vec![false; n + 1];
        let mut my_2 = 0;

        for i in 0..n {
            a.push(cin.next::<usize>());

            if !ones[a[i]] {
                ones[a[i]] = true;
            } else {
                my_2 += 1;
            }
        }
        println!("{}", my_2);
    }
}
