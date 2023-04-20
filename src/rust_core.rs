use dlopen::wrapper::{Container, WrapperApi};
use dlopen_derive::WrapperApi;

const MODULE_LIBRARY: &str = "target/release/libmodule.so";

#[derive(WrapperApi)]
pub struct ModuleApi {
    run: extern fn(),
}

pub fn load(name: &str) -> Container<ModuleApi> {
    let mut library = std::path::PathBuf::new();
    library.push(crate::module_config::MODULE_DIRECTORY);
    library.push(name);
    library.push(MODULE_LIBRARY);

    unsafe { Container::load(library) }.unwrap()
}
