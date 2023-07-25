use libnoj::*;

struct DefaultJudger;

impl Judger for DefaultJudger {
    fn do_before_run(&mut self, _e: &mut Env) {}
    fn do_in_run(&mut self, _c: Context) {}
    fn do_after_run(&mut self, _e: &mut Env) {}
    fn judge_result(&mut self, _e: &mut Env) {}
    fn is_interactive(&self) -> bool {
        return false;
    }
    fn get_plugin_name(&self) -> &'static str {
        "default-judger"
    }
}

impl DefaultJudger {
    fn create_instance() -> Box<dyn Judger> {
        Box::new(DefaultJudger {} )
    }
}