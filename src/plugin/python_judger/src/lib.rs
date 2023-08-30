use libnoj::Judger;
use libnoj::{register_plugin, Context, Env};

struct PythonJudger;

impl Judger for PythonJudger {
    fn do_before_run(&mut self, e: &mut Env) {
        todo!()
    }
    fn do_in_run(&mut self, c: Context) {
        todo!()
    }
    fn do_after_run(&mut self, e: &mut Env) {
        todo!()
    }
    fn judge_result(&mut self, e: &mut Env) {
        todo!()
    }
    fn is_interactive(&self) -> bool {
        todo!()
    }
    fn get_plugin_name(&self) -> &'static str {
        todo!()
    }
}

impl PythonJudger {
    fn create_instance() -> Box<dyn Judger> {
        Box::new(PythonJudger {})
    }
}

#[no_mangle]
fn plugin_init() {
    register_plugin(PythonJudger::create_instance);
}
