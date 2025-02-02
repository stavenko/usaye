mod api;
mod api_error;
mod api_response;
mod cfg;
mod cli;
mod commands;
mod providers;
#[cfg(test)]
mod tests;

fn main() {
    env_logger::init();
    println!("USAYE server started");
    cli::Cli::run().unwrap();
}
