use dlopen::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
struct PluginApi {
    plugin_init: unsafe extern "C" fn(),
}

pub fn load_plugin(path: &str) {
    let mut cont: Container<PluginApi> = unsafe { Container::load(path) }.unwrap();
    unsafe {
        cont.plugin_init();
    }
}
