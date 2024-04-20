use wasm_bindgen::prelude::*;
mod util;

#[wasm_bindgen]
pub fn gen(seed: i32, problem: String) -> String {
    let problem = problem.chars().next().unwrap();
    util::gen(seed as u64, problem).to_string()
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
}

#[wasm_bindgen]
pub fn vis(_input: String, _output: String, turn: usize) -> Ret {
    let input = util::parse_input(&_input);
    let _output = _output.as_str();
    let output = match util::parse_output(&input, &_output) {
        Ok(output) => output,
        Err(err) => {
            return Ret {
                score: 0,
                err: err,
                svg: "".to_string(),
            }
        }
    };
    let (score, err, svg) = util::vis(&input, &output, turn);
    Ret {
        score: score as i64,
        err,
        svg,
    }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> usize {
    let input = util::parse_input(&_input);
    let output = util::parse_output(&input, &_output).unwrap();
    output.out.len()
}
