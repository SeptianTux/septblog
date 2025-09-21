pub mod view;
pub mod viewmodel;
pub mod model;
pub mod db;
pub mod utils;
pub mod error;

fn main() -> std::io::Result<()> {
    view::main::fire()
}