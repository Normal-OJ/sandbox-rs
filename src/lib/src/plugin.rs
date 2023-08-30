use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::Judger;

static PLUGIN_LIST: Lazy<Mutex<Option<fn() -> Box<dyn Judger>>>> = Lazy::new(|| Mutex::new(None));

#[no_mangle]
pub fn register_plugin(create_hook: fn() -> Box<dyn Judger>) {
    *PLUGIN_LIST.lock().unwrap() = Some(create_hook);
}

pub fn find_plugin() -> Option<Box<dyn Judger>> {
    let binding = PLUGIN_LIST.lock().unwrap();
    let plugin_hook = *binding;

    if let Some(hook) = plugin_hook {
        Some(hook())
    } else {
        None
    }
}
