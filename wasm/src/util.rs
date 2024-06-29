#![allow(non_snake_case, unused_macros)]
use svg::node::element::{Group, Rectangle, Style, Title, Circle, Line};
use svg::node::Text as SvgText;
use web_sys::console::log_1;

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
    pub target: Vec<(i32, i32)>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (x, y) in &self.target {
            writeln!(f, "{} {}", x, y)?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let N = f.chars().filter(|&c| c == '\n').count()
                   + if f.chars().last() != Some('\n') { 1 } else { 0 };
    log_1(&format!("parse_input N: {}", N).into());
    let f = proconio::source::once::OnceSource::from(f);
    proconio::input! {
        from f,
        target: [(i32, i32); N],
    }
    Input { target }
}

pub fn parse_input_fixed(f: &str) -> Input {
    let N = f.chars().filter(|&c| c == '\n').count()
                   + if f.chars().last() != Some('\n') { 1 } else { 0 };
                   log_1(&format!("parse_input_fixed N: {}", N).into());
    let f = proconio::source::once::OnceSource::from(f);
    proconio::input! {
        from f,
        target: [(i32, i32); N],
    }
    Input { target }
}

pub struct Output {
    pub acc: Vec<(i32, i32)>,
}

pub fn parse_output(f: &str) -> Result<Output, String> {
    let mut acc = vec![];
    let ss = f.trim().chars().collect::<Vec<char>>();
    
    for mv in ss {
        let (dx, dy) = match mv {
            '1' => (-1, -1),
            '2' => (0, -1),
            '3' => (1, -1),
            '4' => (-1, 0),
            '5' => (0, 0),
            '6' => (1, 0),
            '7' => (-1, 1),
            '8' => (0, 1),
            '9' => (1, 1),
            _ => return Err(format!("Invalid action: {}", mv)),
        };
        acc.push((dx, dy));
    }
    Ok(Output { acc })
}

const FIXED: [&'static str; 25] = [
    include_str!("../in_fixed/spaceship1"),
    include_str!("../in_fixed/spaceship2"),
    include_str!("../in_fixed/spaceship3"),
    include_str!("../in_fixed/spaceship4"),
    include_str!("../in_fixed/spaceship5"),
    include_str!("../in_fixed/spaceship6"),
    include_str!("../in_fixed/spaceship7"),
    include_str!("../in_fixed/spaceship8"),
    include_str!("../in_fixed/spaceship9"),
    include_str!("../in_fixed/spaceship10"),
    include_str!("../in_fixed/spaceship11"),
    include_str!("../in_fixed/spaceship12"),
    include_str!("../in_fixed/spaceship13"),
    include_str!("../in_fixed/spaceship14"),
    include_str!("../in_fixed/spaceship15"),
    include_str!("../in_fixed/spaceship16"),
    include_str!("../in_fixed/spaceship17"),
    include_str!("../in_fixed/spaceship18"),
    include_str!("../in_fixed/spaceship19"),
    include_str!("../in_fixed/spaceship20"),
    include_str!("../in_fixed/spaceship21"),
    include_str!("../in_fixed/spaceship22"),
    include_str!("../in_fixed/spaceship23"),
    include_str!("../in_fixed/spaceship24"),
    include_str!("../in_fixed/spaceship25"),
];

pub fn gen(seed: u64) -> Input {
    let ty = (seed + 24) % 25;
    let input = parse_input_fixed(FIXED[ty as usize]);
    input
}

pub fn compute_score(out: &Output) -> (i64, String) {
    let score = out.acc.len() as i64;
    (score, "".to_string())
}

// (pos, (vx, vy), lines, visited?, all_visited) 
pub fn compute_state(input: &Input, out: &Output, turn: usize) -> ((i32, i32), (i32, i32), Vec<(i32, i32, i32, i32)>, Vec<bool>) {
    let N = input.target.len();
    let mut x = 0;
    let mut y = 0;
    let mut vx = 0;
    let mut vy = 0;
    let mut lines = vec![];
    let mut visited = vec![false; N];
    for i in 0..turn {
        let (ax, ay) = out.acc[i];
        vx += ax;
        vy += ay;
        let nx = x + vx;
        let ny = y + vy;
        for j in 0..N {
            if nx == input.target[j].0 && ny == input.target[j].1 {
                visited[j] = true;
            }
        }
        lines.push((x, y, nx, ny));
        x = nx;
        y = ny;
    }
    ((x, y), (vx, vy), lines, visited)
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

fn to_canvas_pos(x: i32, y: i32, dot_per_unit: f64, canvas_size: i32) -> (f64, f64) {
    // (x * dot_per_unit as i32 + canvas_size / 2, y * dot_per_unit as i32 + canvas_size / 2)
    (x as f64 * dot_per_unit + canvas_size as f64 / 2.0, -1.0 * y as f64 * dot_per_unit + canvas_size as f64 / 2.0)
}

pub fn arrow(x1: usize, y1: usize, x2: usize, y2: usize, color: &str) -> Line {
    Line::new()
        .set("x1", x1)
        .set("y1", y1)
        .set("x2", x2)
        .set("y2", y2)
        .set("stroke", color)
        .set("stroke-width", 3)
        .set("stroke-linecap", "round")
        .set("stroke-linecap", "round")
        .set("marker-end", "url(#arrowhead)")
}

pub fn vis(input: &Input, output: &Output, turn: usize) -> (i64, String, String, String, String) {
    let (score, err) = compute_score(output);
    let (pos, vel, lines, visited) = compute_state(input, output, turn);

    // max(abs(x), abs(y)) を描画範囲に
    let mut draw_range = 1;
    input.target.iter().for_each(|(x, y)| {
        draw_range.setmax(x.abs());
        draw_range.setmax(y.abs());
    });

    let W = 800;
    let H = 800;
    let dot_per_unit = W as f64 / (draw_range * 2) as f64;
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

    doc = doc.add(SvgText::new(
        r#"<defs>
            <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="3" refY="2" orient="auto">
                <polygon points="0 0, 4 2, 0 4" fill="black"/>
            </marker>
        </defs>"#,
    ));
    
    // spaceship
    let mut grp = group(format!("spaceship: ({}, {})", pos.0, pos.1));
    let (x, y) = to_canvas_pos(pos.0, pos.1, dot_per_unit, W);
    let spaceship_size = 4;
    grp = grp.add(
        rect(x as usize - spaceship_size, y as usize - spaceship_size, 2 * spaceship_size, 2 * spaceship_size, "black")
    );
    doc = doc.add(grp);
    for (x1, y1, x2, y2) in lines {
        let (x1, y1) = to_canvas_pos(x1, y1, dot_per_unit, W);
        let (x2, y2) = to_canvas_pos(x2, y2, dot_per_unit, W);
        doc = doc.add(arrow(x1 as usize, y1 as usize, x2 as usize, y2 as usize, "black"));
    }

    // targets
    for i in 0..input.target.len() {
        let (x, y) = to_canvas_pos(input.target[i].0, input.target[i].1, dot_per_unit, W);
        let mut grp = group(format!("({}, {})", input.target[i].0, input.target[i].1));
        let color = if visited[i] { "cyan" } else { "red" };
        grp = grp.add(
            Circle::new()
                .set("cx", x)
                .set("cy", y)
                .set("r", 3)
                .set("fill", color),
        );
        doc = doc.add(grp);
    }



    let pos_str = format!("{} {}", pos.0, pos.1);
    let vel_str = format!("{} {}", vel.0, vel.1);
    (score as i64, pos_str, vel_str, err, doc.to_string())
}
