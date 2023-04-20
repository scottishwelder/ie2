mod module_config;
mod rust_core;
use clap::Parser;

#[derive(Parser, Debug)]
struct CLI {
    /// Module name. As defined on the configuration file.
    module: String,
}

pub fn run() {
    let cli = CLI::parse();
    let module_config = module_config::load_module_config(&cli.module);
    let module = rust_core::load(cli.module);
    module.run();
    println!("{:?}", module_config);
}
