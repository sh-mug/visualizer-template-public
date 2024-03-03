#![allow(non_snake_case, unused_macros)]
use svg::node::element::{Group, Rectangle, Style, Title, Text};
use svg::node::Text as SvgText;
// use web_sys::console::log_1;
use itertools::Itertools;
use proconio::{input, marker::Chars};
use rand::prelude::*;
use std::ops::RangeBounds;
use std::cmp::min;

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

#[derive(Clone, Debug)]
pub struct Input {
    pub ty: u64,
    pub n: usize,
    pub a: Vec<Vec<i32>>,
    pub vs: Vec<Vec<char>>,
    pub hs: Vec<Vec<char>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.ty, self.n)?;
        for i in 0..self.n {
            writeln!(f, "{}", self.vs[i].iter().collect::<String>())?;
        }
        for i in 0..self.n - 1 {
            writeln!(f, "{}", self.hs[i].iter().collect::<String>())?;
        }
        for i in 0..self.n {
            writeln!(f, "{}", self.a[i].iter().join(" "))?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        ty: u64, n: usize,
        vs: [Chars; n],
        hs: [Chars; n - 1],
        a: [[i32; n]; n],
    }
    Input { ty, n, a, vs, hs }
}

pub fn parse_input_fixed(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        ty: u64, n: usize,
        vs: [Chars; n],
        hs: [Chars; n - 1],
    }
    for i in 0..n {
        assert_eq!(vs[i].len(), n - 1);
    }
    for i in 0..n - 1 {
        assert_eq!(hs[i].len(), n);
    }
    Input {
        ty,
        n,
        a: vec![],
        vs,
        hs,
    }
}

pub fn read<T: Copy + PartialOrd + std::fmt::Display + std::str::FromStr, R: RangeBounds<T>>(
    token: Option<&str>,
    range: R,
) -> Result<T, String> {
    if let Some(v) = token {
        if let Ok(v) = v.parse::<T>() {
            if !range.contains(&v) {
                Err(format!("Out of range: {}", v))
            } else {
                Ok(v)
            }
        } else {
            Err(format!("Parse error: {}", v))
        }
    } else {
        Err("Unexpected EOF".to_owned())
    }
}

const DIRS: [char; 4] = ['U', 'D', 'L', 'R'];
const DIJ: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];

pub struct Output {
    pub start: (usize, usize, usize, usize),
    pub out: Vec<(bool, usize, usize)>,
}

pub fn parse_output(input: &Input, f: &str) -> Result<Output, String> {
    let mut out = vec![];
    let mut ss = f.split_whitespace();
    let start = (
        read(ss.next(), 0..input.n)?,
        read(ss.next(), 0..input.n)?,
        read(ss.next(), 0..input.n)?,
        read(ss.next(), 0..input.n)?,
    );
    while let Some(mv) = ss.next() {
        let do_swap = if mv == "1" {
            true
        } else if mv != "0" {
            return Err(format!("Invalid action: {}", mv));
        } else {
            false
        };
        let dir1 = read(ss.next(), '.'..='Z')?;
        let dir2 = read(ss.next(), '.'..='Z')?;
        let dir1 = if dir1 == '.' {
            !0
        } else if let Some(dir1) = DIRS.iter().position(|&d| d == dir1) {
            dir1
        } else {
            return Err(format!("Invalid direction: {}", dir1));
        };
        let dir2 = if dir2 == '.' {
            !0
        } else if let Some(dir2) = DIRS.iter().position(|&d| d == dir2) {
            dir2
        } else {
            return Err(format!("Invalid direction: {}", dir2));
        };
        out.push((do_swap, dir1, dir2));
    }
    if out.len() > 4 * input.n * input.n {
        return Err("Too many actions".to_owned());
    }
    Ok(Output { start, out })
}

const FIXED: [&'static str; 20] = [
    include_str!("../in_fixed/0.txt"),
    include_str!("../in_fixed/1.txt"),
    include_str!("../in_fixed/2.txt"),
    include_str!("../in_fixed/3.txt"),
    include_str!("../in_fixed/4.txt"),
    include_str!("../in_fixed/5.txt"),
    include_str!("../in_fixed/6.txt"),
    include_str!("../in_fixed/7.txt"),
    include_str!("../in_fixed/8.txt"),
    include_str!("../in_fixed/9.txt"),
    include_str!("../in_fixed/10.txt"),
    include_str!("../in_fixed/11.txt"),
    include_str!("../in_fixed/12.txt"),
    include_str!("../in_fixed/13.txt"),
    include_str!("../in_fixed/14.txt"),
    include_str!("../in_fixed/15.txt"),
    include_str!("../in_fixed/16.txt"),
    include_str!("../in_fixed/17.txt"),
    include_str!("../in_fixed/18.txt"),
    include_str!("../in_fixed/19.txt"),
];

pub fn gen(seed: u64) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let ty = seed % 20;
    let mut input = parse_input_fixed(FIXED[ty as usize]);
    let mut nums = (1..=input.n * input.n).collect_vec();
    nums.shuffle(&mut rng);
    input.a = mat![0; input.n; input.n];
    for i in 0..input.n {
        for j in 0..input.n {
            input.a[i][j] = nums[i * input.n + j] as i32;
        }
    }
    input
}

fn can_move(N: usize, h: &Vec<Vec<char>>, v: &Vec<Vec<char>>, i: usize, j: usize, dir: usize) -> bool {
    let (di, dj) = DIJ[dir];
    let i2 = i + di;
    let j2 = j + dj;
    if i2 >= N || j2 >= N {
        return false;
    }
    if di == 0 {
        v[i][j.min(j2)] == '0'
    } else {
        h[i.min(i2)][j] == '0'
    }
}

pub fn compute_score(input: &Input, out: &Output) -> (i64, String) {
    let (mut score, err, _) = compute_score_details(input, out.start, &out.out);
    if err.len() > 0 {
        score = 0;
    }
    (score, err)
}

fn compute_diff(input: &Input, a: &Vec<Vec<i32>>) -> i64 {
    let mut diff = 0;
    for i in 0..input.n {
        for j in 0..input.n {
            for dir in 1..=2 {
                if can_move(input.n, &input.hs, &input.vs, i, j, dir) {
                    let d = (a[i][j] - a[i + DIJ[dir].0][j + DIJ[dir].1]) as i64;
                    diff += d * d;
                }
            }
        }
    }
    diff
}

pub fn compute_score_details(
    input: &Input,
    start: (usize, usize, usize, usize),
    out: &[(bool, usize, usize)],
) -> (i64, String, (Vec<Vec<i32>>, (usize, usize), (usize, usize))) {
    let mut a = input.a.clone();
    let mut p1 = (start.0, start.1);
    let mut p2 = (start.2, start.3);
    let before = compute_diff(&input, &a);
    for &(do_swap, dir1, dir2) in out {
        if do_swap {
            let tmp = a[p1.0][p1.1];
            a[p1.0][p1.1] = a[p2.0][p2.1];
            a[p2.0][p2.1] = tmp;
        }
        if dir1 != !0 {
            if !can_move(input.n, &input.hs, &input.vs, p1.0, p1.1, dir1) {
                return (0, format!("Invalid move: {}", DIRS[dir1]), (a, p1, p2));
            }
            p1.0 += DIJ[dir1].0;
            p1.1 += DIJ[dir1].1;
        }
        if dir2 != !0 {
            if !can_move(input.n, &input.hs, &input.vs, p2.0, p2.1, dir2) {
                return (0, format!("Invalid move: {}", DIRS[dir2]), (a, p1, p2));
            }
            p2.0 += DIJ[dir2].0;
            p2.1 += DIJ[dir2].1;
        }
    }
    let after = compute_diff(&input, &a);
    let score = ((1e6 * (f64::log2(before as f64) - f64::log2(after as f64))).round() as i64).max(1);
    (score, String::new(), (a, p1, p2))
}

pub fn rect(x: usize, y: usize, w: usize, h: usize, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}

fn group(title: String) -> Group {
    Group::new().add(Title::new().add(SvgText::new(title)))
}

pub fn vis(input: &Input, output: &Output, turn: usize) -> (i64, String, String) {
    let (score, err, (state, p1, p2)) = compute_score_details(input, output.start, &output.out[..turn]);

    let w = min(30, 1000 / input.n);
    let h = min(30, 1000 / input.n);
    let W = input.n * w;
    let H = input.n * h;
    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-5, -5, W + 10, H + 10))
        .set("width", W + 10)
        .set("height", H + 10)
        .set("style", "background-color:white");

    doc = doc.add(Style::new(format!(
        "text {{text-anchor: middle;dominant-baseline: central; font-size: {}}}",
        6
    )));
    for y in 0..input.n {
        for x in 0..input.n {
            let mut grp = group(format!("a[{}][{}]={}", y, x, state[y][x]));
            grp = grp.add(
                rect(
                    x * w,
                    y * h,
                    w,
                    h,
                    "white"
                )
                .set("stroke", "lightgray")
                .set("stroke-width", 1)
                .set("class", "box"),
            );
            doc = doc.add(grp);
            doc = doc.add(
                Text::new()
                    .set("x", x * w + w / 2)
                    .set("y", y * h + h / 2)
                    .set("font-size", w / 3)
                    .add(svg::node::Text::new(state[y][x].to_string())),
            );
        }
    }

    // wall
    for y in 0..input.n - 1 {
        for x in 0..input.n {
            if input.hs[y][x] == '1' {
                doc = doc.add(
                    rect(
                        x * w,
                        (y + 1) * h,
                        w,
                        1,
                        "black"
                    )
                    .set("class", "wall"),
                );
            }
        }
    }
    for y in 0..input.n {
        for x in 0..input.n - 1 {
            if input.vs[y][x] == '1' {
                doc = doc.add(
                    rect(
                        (x + 1) * w,
                        y * h,
                        1,
                        h,
                        "black"
                    )
                    .set("class", "wall"),
                );
            }
        }
    }

    // 外周も壁
    doc = doc.add(
        rect(
            0,
            0,
            W,
            1,
            "black"
        )
        .set("class", "wall"),
    );
    doc = doc.add(
        rect(
            0,
            H - 1,
            W,
            1,
            "black"
        )
        .set("class", "wall"),
    );
    doc = doc.add(
        rect(
            0,
            0,
            1,
            H,
            "black"
        )
        .set("class", "wall"),
    );
    doc = doc.add(
        rect(
            W - 1,
            0,
            1,
            H,
            "black"
        )
        .set("class", "wall"),
    );

    // p1: Takahashi
    let mut grp = group(format!("Takahashi: a[{}][{}]={}", p1.0, p1.1, state[p1.0][p1.1]));
    grp = grp.add(
        rect(
            p1.1 * w,
            p1.0 * h,
            w,
            h,
            "lightpink"
        )
        .set("fill-opacity", 0.5)
        .set("class", "p1"),
    );
    doc = doc.add(grp);

    // p2: Aoki
    let mut grp = group(format!("Aoki: a[{}][{}]={}", p2.0, p2.1, state[p2.0][p2.1]));
    grp = grp.add(
        rect(
            p2.1 * w,
            p2.0 * h,
            w,
            h,
            "lightblue"
        )
        .set("fill-opacity", 0.5)
        .set("class", "p2"),
    );
    doc = doc.add(grp);

    (score, err, doc.to_string())
}
