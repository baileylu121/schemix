use std::ops::Range;

use ariadne::{Color, Config, Label, Report, ReportKind, Source};
use chumsky::error::{RichPattern, RichReason};
use chumsky::prelude::*;

pub fn print_errors(src: &str, errors: Vec<Rich<char>>) {
    for err in errors {
        let span: Range<usize> = err.span().into_range();

        let mut report = Report::build(ReportKind::Error, span.clone())
            .with_config(Config::default().with_char_set(ariadne::CharSet::Unicode));

        match err.reason() {
            RichReason::ExpectedFound { expected, found } => {
                let found_msg = match found {
                    Some(c) => format!("found '{}'", **c),
                    None => "found end of input".to_string(),
                };

                let expected_msg = if expected.is_empty() {
                    "something else".to_string()
                } else {
                    expected
                        .iter()
                        .map(|p: &RichPattern<char>| p.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                };

                report.set_message(format!("{found_msg}, expected {expected_msg}"));
                report.add_label(
                    Label::new(span)
                        .with_message(format!("{found_msg}, expected {expected_msg}"))
                        .with_color(Color::Red),
                );
            }
            RichReason::Custom(msg) => {
                report.set_message(msg.clone());
                report.add_label(
                    Label::new(span)
                        .with_message(msg.clone())
                        .with_color(Color::Yellow),
                );
            }
        }

        for (label, label_span) in err.contexts() {
            let label_span: Range<usize> = label_span.into_range();
            report.add_label(
                Label::new(label_span)
                    .with_message(label.to_string())
                    .with_color(Color::Blue),
            );
        }

        report.finish().eprint(Source::from(src)).unwrap();
    }
}
