pub use context::Context;
pub use env::Env;
pub use lang::Lang;

mod env;
mod context;
mod lang;

pub trait Judger {
    fn do_before_run(&mut self, e: &mut Env);
    fn do_in_run(&mut self, c: Context);
    fn do_after_run(&mut self, e: &mut Env);
    fn judge_result(&mut self, e: &mut Env);
    fn is_interactive(&self) -> bool;
}


