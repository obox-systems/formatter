use std::path::PathBuf;

use miette::{IntoDiagnostic as _, WrapErr as _};
use wca::{Args, Context, Props};

use crate::{parse_args, Result};

pub(crate) fn format(cx: Context, _args: Args, _props: Props) -> Result {
    let source = cx.get_ref::<PathBuf>();

    match source {
        Some(path) => {
            for entry in jwalk::WalkDir::new(path) {
                let entry = entry.into_diagnostic()?;

                let path = entry.path();
                if path.is_dir() {
                    continue;
                }

                let source = std::fs::read_to_string(&path)
                    .into_diagnostic()
                    .with_context(|| format!("reading `{}`", path.display()))?;

                let contents = crate::formatter::format(&source);
                std::fs::write(&path, contents)
                    .into_diagnostic()
                    .with_context(|| format!("writing `{}`", path.display()))?;
            }
        }
        None => todo!("WTF??"),
    }

    Ok(())
}

pub(crate) fn with(cx: Context, args: Args, _props: Props) -> Result {
    let mut args = args.0.into_iter();
    parse_args!(args, path: PathBuf);

    cx.insert(path);

    Ok(())
}
