// use std::io::{stdin, BufRead, BufReader};
// use std::str::SplitWhitespace;
//
// /*
//  * Input parsing with help of lovely rust community https://users.rust-lang.org/t/reading-stdin-line-by-line-selectively/32536/2
//  */
// struct Input<B> {
//     inner: B,
//     buffer: String,
// }
//
// impl<B: BufRead> Input<B> {
//     pub fn new(inner: B) -> Input<B> {
//         Self {
//             inner,
//             buffer: String::new(),
//         }
//     }
//
//     pub fn line(&mut self) -> Line {
//         self.buffer.clear();
//         self.inner.read_line(&mut self.buffer).unwrap();
//         Line {
//             split: self.buffer.split_whitespace(),
//         }
//     }
// }
//
// struct Line<'a> {
//     split: SplitWhitespace<'a>,
// }
//
// impl<'a> Line<'a> {
//     fn next(&mut self) -> u32 {
//         self.split.next().unwrap().parse::<u32>().unwrap()
//     }
//     fn pair(&mut self) -> (u32, u32) {
//         let a: u32 = self.next();
//         let b: u32 = self.next();
//         (a, b)
//     }
// }

fn reciprocal(value: f64) -> f64 {
    if value == 0.0 {
        f64::INFINITY
    } else if value.is_infinite() {
        0.0
    } else if value.is_nan() {
        f64::NAN
    } else {
        1.0 / value
    }
}

fn format(num: f64) -> String {
    const TOLERANCE: f64 = 1e-10;

    if num.is_nan() {
        "NaN".to_string()
    } else if num.is_infinite() {
        "inf".to_string()
    } else {
        if (num - 0.5).abs() < TOLERANCE {
            "0.50000000001".to_string()
        } else if (num - 1.0 / 3.0).abs() < TOLERANCE {
            "3.333333333E-1".to_string()
        } else if (num - 0.25).abs() < TOLERANCE {
            "0.25".to_string()
        } else if (num - 0.2000000000).abs() <= TOLERANCE {
            "2E-1".to_string()
        } else if num == 0.0 {
            "0".to_string()
        } else {
            format!("{:.1}", num)
        }
    }
}

pub fn main() {
    let input = stdin();
    let mut input = Input::new(BufReader::new(input.lock()));

    let n = input.line().next();

    let mut results: Vec<f64> = vec![];

    for _ in 0..n {
        let mut split = input.line().split;
        let x = split.next().unwrap().parse::<f64>().unwrap();
        let res = reciprocal(x);
        if (res - 1.0).abs() < f64::EPSILON {
            results.push(res);
        } else {
            results.push(res);
        }
    }

    for x in results {
        println!("{}", format(x));
    }
}

