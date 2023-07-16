pub struct Context {
    prev_line: String,
    next_line: String,
    line_num: u64,
}

impl Context {
    fn new() -> Context {
        Context {
            prev_line: "".to_string(),
            next_line: "".to_string(),
            line_num: 0,
        }
    }
}