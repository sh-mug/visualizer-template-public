#![allow(non_snake_case, unused_macros)]
use proconio::input;
use rand::prelude::*;
use std::collections::BTreeSet;
use svg::node::element::{Rectangle, Style};
use web_sys::console::log_1;

#[derive(Clone, Debug)]
pub struct Input {
    pub n: usize,
    pub xyr: Vec<(usize, usize, usize)>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.n)?;
        for (x, y, r) in &self.xyr {
            writeln!(f, "{} {} {}", x, y, r)?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        n: usize,
        xyr: [(usize, usize, usize); n]
    }
    Input { n, xyr }
}

pub struct Output {
    pub abcd: Vec<(usize, usize, usize, usize)>,
}

pub fn parse_output(n: usize, f: &str) -> Output {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        abcd: [(usize, usize, usize, usize); n]
    }
    Output { abcd }
}

// copied from official input generator
pub fn gen(seed: u64) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let n = (50.0 * 4.0f64.powf(rng.gen::<f64>())).round() as usize;
    let mut ps = vec![];
    let mut used = BTreeSet::new();
    for _ in 0..n {
        loop {
            let x = rng.gen_range(0, 10000);
            let y = rng.gen_range(0, 10000);
            if used.insert((x, y)) {
                ps.push((x, y));
                break;
            }
        }
    }
    let mut q = rand::seq::index::sample(&mut rng, 10000 * 10000 - 1, n - 1)
        .into_iter()
        .map(|a| a + 1)
        .collect::<Vec<_>>();
    q.sort();
    q.insert(0, 0);
    q.push(10000 * 10000);
    let mut r = vec![];
    for i in 0..n {
        r.push(q[i + 1] - q[i]);
    }

    let xyr = ps.into_iter().zip(r).map(|((x, y), r)| (x, y, r)).collect();
    Input { n, xyr }
}

fn calculate_score(input: &Input, output: &Output) -> usize {
    // score is sum for each i of
    //  if rectangle (ai, bi) -- (ci, di) does not contain (xi+0.5, yi+0.5) then 0
    //  else, let si be the space of the rectangle (ai, bi) -- (ci, di),
    //   then 1-(1-min(ri,si)/max(ri,si))^2
    log_1(&format!("{:?}", input).into());
    let mut score = 0f64;
    for i in 0..input.n {
        let (x, y, r) = input.xyr[i];
        let (a, b, c, d) = output.abcd[i];
        score += if a <= x && x <= c && b <= y && y <= d {
            let s = ((c - a) * (d - b)) as f64;
            1f64 - (1f64 - (s.min(r as f64) / s.max(r as f64))).powi(2)
        } else {
            0f64
        };
    }
    (1e9 * score / input.n as f64).round() as usize
}

fn generate_color(s: usize, r: usize) -> String {
    let hue = (s as f64 / r as f64) * -180.0 % 360.0;
    format!("hsl({}, 100%, 50%)", hue)
}

pub fn rect(x: f32, y: f32, w: f32, h: f32, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}

pub fn vis(input: &Input, output: &Output) -> (i64, String, String) {
    let score = calculate_score(input, output);

    let W = 1000;
    let H = 1000;
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
    for i in 0..input.n {
        let (x, y, r) = input.xyr[i];
        let (a, b, c, d) = output.abcd[i];
        // draw rectangle (a, b) -- (c, d)
        let s = (c - a) * (d - b);
        let rect_color = generate_color(s, r);
        doc = doc.add(rect(
            a as f32 * 0.1,
            b as f32 * 0.1,
            (c - a) as f32 * 0.1,
            (d - b) as f32 * 0.1,
            &rect_color,
        ));
        // draw point (x, y)
        doc = doc.add(rect(
            (x as f32 + 0.5) * 0.1 - 2.5,
            (y as f32 + 0.5) * 0.1 - 2.5,
            5.0,
            5.0,
            "black",
        ));
        // draw line (x, y) -- ((a+c)/2, (b+d)/2)
        doc = doc.add(
            svg::node::element::Line::new()
                .set("x1", x as f32 * 0.1)
                .set("y1", y as f32 * 0.1)
                .set("x2", (a + c) as f32 * 0.05)
                .set("y2", (b + d) as f32 * 0.05)
                .set("stroke", "black")
                .set("stroke-width", 0.5),
        );
    }

    (score as i64, "".to_string(), doc.to_string())
}
