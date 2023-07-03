#![allow(incomplete_features)]
#![feature(generic_const_exprs, internal_output_capture, exit_status_error)]
#![deny(
    clippy::use_self,
    clippy::doc_markdown,
    unused_qualifications,
    unreachable_pub
)]

use crate::stdx::CommandExt;

mod commands;
mod formatter;
mod stdx;

pub(crate) type Result<T = (), E = miette::Report> = miette::Result<T, E>;

fn main() -> Result {
    use itertools::Itertools as _;
    use miette::IntoDiagnostic as _;
    use wca::Type;

    let aggregator = stdx::cli()
        .command(commands::format)
        .command(commands::with.arg("path", Type::Path))
        .build();

    let args = std::env::args().skip(1).join(" ");
    aggregator.perform(args).into_diagnostic()?;

    Ok(())
}
