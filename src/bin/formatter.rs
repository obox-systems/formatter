use itertools::Itertools;
use wca::stdx as wca_cpp;

fn main() {
    let cli = wca_cpp::cli(()).build();

    let args = std::env::args().skip(1).join(" ");
    if let Err(message) = cli.perform(args) {
        eprintln!("{message}");
    }
}
