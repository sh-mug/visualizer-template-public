use wasm_bindgen::prelude::*;
mod util;

#[wasm_bindgen]
pub fn gen(seed: i32) -> String {
    util::gen(seed as u64).to_string()
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub pos: String,
    pub vel: String,
    pub err: String,
    pub svg: String,
}

#[wasm_bindgen]
pub fn vis(_input: String, _output: String, turn: usize) -> Ret {
    let input = util::parse_input(&_input);
    let _output = _output.as_str();
    let output = match util::parse_output(&_output) {
        Ok(output) => output,
        Err(err) => return Ret {
            score: 0,
            pos: "".to_string(),
            vel: "".to_string(),
            err: err,
            svg: "".to_string(),
        },
    };
    let (score, pos, vel, err, svg) = util::vis(&input, &output, turn);
    Ret {
        score: score,
        pos: pos,
        vel: vel,
        err: err,
        svg: svg,
    }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> usize {
    let output = util::parse_output(&_output).unwrap();
    output.acc.len()
}
