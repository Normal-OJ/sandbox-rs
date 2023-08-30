pub use context::Context;
pub use env::Env;
pub use lang::Lang;
pub use plugin::find_plugin;
pub use plugin::register_plugin;

mod context;
mod env;
mod lang;
mod plugin;

pub trait Judger {
    fn do_before_run(&mut self, e: &mut Env);
    fn do_in_run(&mut self, c: Context);
    fn do_after_run(&mut self, e: &mut Env);
    fn judge_result(&mut self, e: &mut Env);
    fn is_interactive(&self) -> bool;
    fn get_plugin_name(&self) -> &'static str;
}
