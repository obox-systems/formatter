use itertools::Itertools as _;
use wca::stdx as wca_cpp;

fn main() {
    // Create a new instance of the `cli` using `wca_cpp::cli(())`, and then build the CLI.
    // The `cli` is an instance of a command-line interface that can parse and execute commands.
    let cli = wca_cpp::cli(()).build();

    // Get the command-line arguments and join them into a single space-separated string.
    // We skip the first argument, which is the name of the executable itself.
    let args = std::env::args().skip(1).join(" ");

    // Perform the command specified by the joined command-line arguments using the `cli` instance.
    // If there is an error during command execution, the error message is returned.
    if let Err(message) = cli.perform(args) {
        // If an error occurred, print the error message to the standard error stream.
        eprintln!("{message}");
    }
}
