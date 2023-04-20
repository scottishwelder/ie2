pub mod error;
mod module_config;
mod rust_core;

use crate::error::Error;
use clap::Parser;
use console::Style;
use module_config::Core;
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct CLI {
    /// Program name.
    program: String,
    /// Module name.
    module: String,
}

const PROGRAM_DIRECTORY: &str = "programs";

pub fn run() -> Result<(), Error> {
    let cli = CLI::parse();

    front_end(&cli.program).map_err(|e| match e {
        Error::NotFound(_) => Error::NotFound("clang"),
        Error::PermissionDenied(_) => Error::PermissionDenied("clang"),
        Error::CommandFailed(_) => Error::CommandFailed("clang"),
        _ => Error::Unknown("calling clang"),
    })?;

    let config = module_config::load_module_config(&cli.module).unwrap();

    // let module = rust_core::load(&cli.module);

    //match config.core {
    //    Core::Native => {}
    //}

    //module.run();
    //println!("{:?}", config);
    Ok(())
}

fn front_end(program_name: &String) -> Result<(), Error> {
    let program_path = PathBuf::from_iter([PROGRAM_DIRECTORY, program_name]);
    let mut output_path = program_path.clone();
    output_path.set_extension("ll");

    handoff_warning("Clang");
    let exit_code = std::process::Command::new("clang")
        .args([
            OsStr::new("-o"),
            output_path.as_ref(),
            OsStr::new("-S"),
            OsStr::new("-emit-llvm"),
            program_path.as_ref(),
        ])
        .status()?
        .success();
    end_handoff_warning();

    if exit_code {
        Ok(())
    } else {
        Err(Error::CommandFailed(""))
    }
}

fn handoff_warning(name: &str) {
    let style = Style::new().cyan();
    let mut ho = name.to_owned();
    ho.push_str(" handoff");
    println!("{}", style.apply_to(ho));

    let mut line = "━".repeat(15);
    line.push_str(name);
    line.push_str(" stdout and stderr");
    line.push_str(&"━".repeat(15));
    println!("{}", style.apply_to(line));
}

fn end_handoff_warning() {
    let style = Style::new().cyan();
    println!("{}", style.apply_to("━".repeat(50)));
}

fn _middle() {}
fn _back() {}
