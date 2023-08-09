use std::{cell::RefCell, path::PathBuf, rc::Rc};

use itertools::Itertools as _;
use miette::{Context as _, IntoDiagnostic as _};
use wca::{stdx as wca_cpp, CommandExt as _, Type};

#[derive(Clone, Default)]
struct AppState {
    path: Option<PathBuf>,
}

fn with(state: Rc<RefCell<AppState>>, args: wca::Args, _props: wca::Props) -> Result<(), ()> {
    let mut args = args.0.into_iter();
    wca::parse_args!(args, path: PathBuf);

    state.borrow_mut().path = Some(path);

    Ok(())
}

fn format(
    state: Rc<RefCell<AppState>>,
    _args: wca::Args,
    _props: wca::Props,
) -> miette::Result<()> {
    let path = state.borrow().path.clone().unwrap();

    for entry in jwalk::WalkDir::new(path) {
        let entry = entry.into_diagnostic()?;

        let path = entry.path();
        if path.is_dir() {
            continue;
        }

        let input = std::fs::read_to_string(&path)
            .into_diagnostic()
            .with_context(|| format!("reading `{}`", path.display()))?;

        let contents = {
            let profile = std::fs::read_to_string("rust.toml").unwrap();
            let profile: formatter::ir::Profile = toml::from_str(&profile).unwrap();
            formatter::format(&input, profile)
        };

        std::fs::write(&path, contents)
            .into_diagnostic()
            .with_context(|| format!("writing `{}`", path.display()))?;
    }

    Ok(())
}

fn main() {
    // Create a new instance of the `cli` using `wca_cpp::cli(())`, and then build the CLI.
    // The `cli` is an instance of a command-line interface that can parse and execute commands.
    let cli = wca_cpp::cli(Rc::new(AppState::default().into()))
        .command(with.arg("path", Type::Path))
        .command(format)
        .build();

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
