pub use env::Env;
pub use lang::Lang;
pub use plugin::find_plugin;
pub use plugin::register_plugin;

mod env;
mod lang;
mod plugin;

pub trait Judger {
    fn do_config_judge_process(&mut self, _e: &Env, pid: u32);
    fn get_plugin_name(&self) -> &'static str;
}
