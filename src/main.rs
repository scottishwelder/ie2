use ie2::error::Error;

fn main() {
    let result = ie2::run();
    match result {
        Ok(_) => {}
        Err(Error::CommandFailed(command)) => {
            println!("Error: {command} command was not successful")
        }
        Err(Error::NotFound(file)) => println!("Error: file {file} was not found"),
        Err(Error::PermissionDenied(file)) => println!("Error: permission denied for file {file}"),
        Err(Error::Unknown(reason)) => println!("Unknown error {reason}"),
    }
}
