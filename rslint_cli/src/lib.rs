mod files;
mod cli;
mod panic_hook;

pub use self::{
    files::*,
    cli::ExplanationRunner,
    panic_hook::*,
};
pub use rslint_core::{DiagnosticBuilder, Diagnostic, Outcome};

use codespan_reporting::term::Config;
use codespan_reporting::term::{
    emit,
    termcolor::{ColorChoice, StandardStream, self},
};
use rayon::prelude::*;
use rslint_core::{lint_file, CstRuleStore};

pub(crate) const DOCS_LINK_BASE: &str = "https://raw.githubusercontent.com/RDambrosio016/RSLint/dev/docs/rules";

pub fn codespan_config() -> Config {
    let mut base = Config::default();
    base.chars.multi_top_left = '┌';
    base.chars.multi_bottom_left = '└';
    base
}

pub fn run(glob: String, verbose: bool) {
    let res = glob::glob(&glob);
    if let Err(err) = res {
        lint_err!("Invalid glob pattern: {}", err);
        return;
    }

    let walker = FileWalker::from_glob(res.unwrap());
    let results = walker
        .files
        .par_iter()
        .map(|(id, file)| {
            lint_file(
                *id,
                &file.source,
                file.kind == JsFileKind::Module,
                CstRuleStore::new().builtins(),
                verbose,
            )
        })
        .collect::<Vec<_>>();

    let failures = results.iter().filter(|res| res.outcome() == Outcome::Failure).count();
    let warnings = results.iter().filter(|res| res.outcome() == Outcome::Warning).count();
    let successes = results.iter().filter(|res| res.outcome() == Outcome::Success).count();

    for result in results.iter() {
        for diagnostic in result.diagnostics() {
            emit(
                &mut StandardStream::stderr(ColorChoice::Always),
                &codespan_config(),
                &walker,
                diagnostic,
            )
            .expect("Failed to throw diagnostic");
        }
    }

    output_overall(failures, warnings, successes);
    if let Outcome::Failure = Outcome::merge(results.iter().map(|res| res.outcome())) {
        println!("\nhelp: for more information about the errors try the explain command: `rslint explain <rules>`");
    }
}

fn output_overall(failures: usize, warnings: usize, successes: usize) {
    use std::io::Write;
    use termcolor::{Color, ColorSpec, WriteColor};

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))).unwrap();
    write!(&mut stdout, "\nOutcome: ").unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
    write!(&mut stdout, "{}", failures).unwrap();
    stdout.reset().unwrap();
    write!(&mut stdout, " fail, ").unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
    write!(&mut stdout, "{}", warnings).unwrap();
    stdout.reset().unwrap();
    write!(&mut stdout, " warn, ").unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
    write!(&mut stdout, "{}", successes).unwrap();
    stdout.reset().unwrap();
    write!(&mut stdout, " success\n").unwrap();
}

#[macro_export]
macro_rules! lint_diagnostic {
    ($severity:ident, $($format_args:tt)*) => {
        use $crate::DiagnosticBuilder;
        use codespan_reporting::{
            files::SimpleFiles,
            term::{termcolor::{ColorChoice::Always, self}, emit}
        };

        let diag = DiagnosticBuilder::$severity(0, "", format!($($format_args)*));
        emit(
            &mut termcolor::StandardStream::stderr(Always),
            &$crate::codespan_config(),
            &SimpleFiles::<String, String>::new(),
            &diag.into()
        ).expect("Failed to throw linter diagnostic");
    }
}

/// Construct a simple linter error and immediately throw it to stderr
#[macro_export]
macro_rules! lint_err {
    ($($format_args:tt)*) => {{
        $crate::lint_diagnostic!(error, $($format_args)*);
    }};
}

/// Construct a simple linter warning and immediately throw it to stderr
#[macro_export]
macro_rules! lint_warn {
    ($($format_args:tt)*) => {{
        $crate::lint_diagnostic!(warning, $($format_args)*);
    }};
}

/// Construct a simple linter note and immediately throw it to stderr
#[macro_export]
macro_rules! lint_note {
    ($($format_args:tt)*) => {{
        $crate::lint_diagnostic!(note_diagnostic, $($format_args)*);
    }};
}