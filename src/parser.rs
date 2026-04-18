use chumsky::prelude::*;

use crate::ast::{Expr, MkOption, Module, NixVal, Visibility};

type Extra<'src> = extra::Err<Rich<'src, char>>;

pub fn parser<'src>() -> impl Parser<'src, &'src str, Vec<Expr<'src>>, Extra<'src>> {
    choice((comment_parser(), module_parser()))
        .repeated()
        .collect()
}

fn comment_parser<'src>() -> impl Parser<'src, &'src str, Expr<'src>, Extra<'src>> {
    just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded()
        .map(|(src, _)| Expr::Comment(src))
        .labelled("comment")
}

fn module_parser<'src>() -> impl Parser<'src, &'src str, Expr<'src>, Extra<'src>> {
    text::keyword("module")
        .padded()
        .ignore_then(nix_val())
        .padded()
        .then(
            option_parser()
                .separated_by(just(','))
                .collect::<Vec<_>>()
                .padded()
                .delimited_by(just("{"), just("}")),
        )
        .map(|(name, options)| Expr::Module(Module { name, options }))
        .labelled("module definition")
}

fn option_parser<'src>() -> impl Parser<'src, &'src str, MkOption<'src>, Extra<'src>> {
    description()
        .then(set((visibility(), internal(), read_only())))
        .then(ident())
        .then(nix_val().or_not())
        .then(default_value())
        .then(example_value())
        .map(
            |(
                (
                    (((description, (visible, internal, read_only)), name), nix_type),
                    default_or_text,
                ),
                example,
            )| {
                let (default, default_text) = match default_or_text {
                    Some(NixValOrText::NixVal(val)) => (Some(val), None),
                    Some(NixValOrText::Text(val)) => (None, Some(val)),
                    None => (None, None),
                };

                MkOption {
                    name,
                    default,
                    default_text,
                    example,
                    description,
                    related_packages: Vec::new(),
                    nix_type,
                    apply: None,
                    internal,
                    visible,
                    read_only,
                }
            },
        )
        .labelled("option")
}

fn internal<'src>() -> impl Parser<'src, &'src str, bool, Extra<'src>> {
    text::keyword("@internal")
        .padded()
        .or_not()
        .map(|opt| opt.is_some())
        .labelled("if internal is enabled")
}

fn read_only<'src>() -> impl Parser<'src, &'src str, bool, Extra<'src>> {
    text::keyword("@read_only")
        .padded()
        .or_not()
        .map(|opt| opt.is_some())
        .labelled("if read only is enabled")
}

fn visibility<'src>() -> impl Parser<'src, &'src str, Visibility, Extra<'src>> {
    let options = choice((
        text::keyword("@visible"),
        text::keyword("@invisible"),
        text::keyword("@shallow"),
        text::keyword("@transparent"),
    ));

    options
        .padded()
        .or_not()
        .map(|x| x.and_then(|s: &str| s.parse().ok()).unwrap_or_default())
        .labelled("visibility")
}

fn ident<'src>() -> impl Parser<'src, &'src str, &'src str, Extra<'src>> {
    none_of(':')
        .repeated()
        .to_slice()
        .then_ignore(just(':'))
        .padded()
        .labelled("identifier")
}

fn description<'src>() -> impl Parser<'src, &'src str, Option<&'src str>, Extra<'src>> {
    just("///")
        .ignore_then(text::whitespace())
        .then(none_of('\n').repeated().to_slice())
        .padded()
        .map(|(_, val)| val)
        .or_not()
        .labelled("comment")
}

#[derive(Debug, PartialEq, Eq)]
enum NixValOrText<'src> {
    Text(&'src str),
    NixVal(NixVal<'src>),
}

fn default_value<'src>() -> impl Parser<'src, &'src str, Option<NixValOrText<'src>>, Extra<'src>> {
    just('=')
        .padded()
        .ignore_then(choice((
            nix_val().map(NixValOrText::NixVal),
            text().map(NixValOrText::Text),
        )))
        .or_not()
        .labelled("default attribute value")
}

fn example_value<'src>() -> impl Parser<'src, &'src str, Option<&'src str>, Extra<'src>> {
    just('|')
        .padded()
        .ignore_then(text())
        .or_not()
        .labelled("example attribute value")
}

fn nix_val<'src>() -> impl Parser<'src, &'src str, NixVal<'src>, Extra<'src>> {
    just('`')
        .ignore_then(none_of('`').repeated().to_slice())
        .then_ignore(just('`'))
        .map(NixVal::Evaluatable)
        .labelled("nix literal value")
}

fn text<'src>() -> impl Parser<'src, &'src str, &'src str, Extra<'src>> {
    just('"')
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .labelled("string literal value")
}

mod tests {
    use super::*;

    #[test]
    fn parses_sample_module() {
        let src = r#"module `services.nginx` {
            /// Enable the service
            enable: `bool` = `false`,

            /// X value
            x: `int`
        }"#;
        let result = module_parser().parse(src);

        if result.has_errors() {
            crate::error::print_errors(src, result.into_errors());
            panic!("parse failed — see ariadne report above");
        }
    }

    #[test]
    fn parses_sample_option() {
        let src = "enable: `bool` = `false`";
        let result = option_parser().parse(src);

        if result.has_errors() {
            crate::error::print_errors(src, result.into_errors());
            panic!("parse failed — see ariadne report above");
        }

        let mk_option = result.into_result().unwrap();

        assert_eq!(mk_option.name, "enable");
        assert_eq!(mk_option.nix_type, Some(NixVal::Evaluatable("bool")));
        assert_eq!(mk_option.default, Some(NixVal::Evaluatable("false")));
    }

    #[test]
    fn parses_sample_option_with_description() {
        let src = "/// enable x service
            enable: `bool` = `false`";

        let result = option_parser().parse(src);

        if result.has_errors() {
            crate::error::print_errors(src, result.into_errors());
            panic!("parse failed — see ariadne report above");
        }

        let mk_option = result.into_result().unwrap();

        assert_eq!(mk_option.name, "enable");
        assert_eq!(mk_option.nix_type, Some(NixVal::Evaluatable("bool")));
        assert_eq!(mk_option.default, Some(NixVal::Evaluatable("false")));
        assert_eq!(mk_option.description, Some("enable x service"));
    }

    #[test]
    fn parses_valid_example() {
        let result = example_value().parse(r#"| "lib.foo.bar [ 1 2 3 ]""#);

        assert!(result.has_output());
        assert!(!result.has_errors());
        assert_eq!(result.into_result().unwrap(), Some("lib.foo.bar [ 1 2 3 ]"));
    }

    #[test]
    fn parses_valid_example_long_whitespace() {
        let result = example_value().parse(r#"|     "lib.foo.bar [ 1 2 3 ]""#);

        assert!(result.has_output());
        assert!(!result.has_errors());
        assert_eq!(result.into_result().unwrap(), Some("lib.foo.bar [ 1 2 3 ]"));
    }

    #[test]
    fn skips_empty_example() {
        let result = example_value().parse(r#""#);

        assert!(result.has_output());
        assert!(!result.has_errors());
        assert_eq!(result.into_result().unwrap(), None);
    }

    #[test]
    fn parses_valid_default() {
        let result = default_value().parse(r#"= `4`"#);

        assert!(result.has_output());
        assert!(!result.has_errors());
        assert_eq!(
            result.into_result().unwrap(),
            Some(NixValOrText::NixVal(NixVal::Evaluatable("4")))
        );
    }

    #[test]
    fn parses_valid_default_long_whitespace() {
        let result = default_value().parse(r#"= `lib.getExe pkgs.bash`"#);

        assert!(result.has_output());
        assert!(!result.has_errors());
        assert_eq!(
            result.into_result().unwrap(),
            Some(NixValOrText::NixVal(NixVal::Evaluatable(
                "lib.getExe pkgs.bash"
            )))
        );
    }

    #[test]
    fn skips_empty_default() {
        let result = default_value().parse(r#""#);

        assert!(result.has_output());
        assert!(!result.has_errors());
        assert_eq!(result.into_result().unwrap(), None);
    }

    #[test]
    fn parses_valid_description() {
        let result = description().parse(
            r#"/// is in doc comment
            "#,
        );

        assert!(result.has_output());
        assert!(!result.has_errors());
        assert_eq!(result.into_result().unwrap(), Some("is in doc comment"));
    }

    #[test]
    fn parses_valid_ident() {
        let result = ident().parse("enable:");

        assert!(result.has_output());
        assert!(!result.has_errors());
        assert_eq!(result.into_result().unwrap(), "enable");
    }

    #[test]
    fn skips_missing_ident() {
        let result = ident().parse("enable `bool` = `false`");

        assert!(!result.has_output());
        assert!(result.has_errors());
    }
}

// let submodule = just("submodule")
//     .padded_by(text::whitespace())
//     .ignore_then(just('{').padded_by(text::whitespace()))
//     .ignore_then(option_list())
//     .ignore_then(just('}').padded_by(text::whitespace()))
//     .map(|options: Vec<MkOption<'src>>| {
//         format!(
//             "lib.types.submodule {{ options = {{ {} }}; }};",
//             options
//                 .iter()
//                 .map(|opt| format!("{} = ...;", opt.name))
//                 .collect::<Vec<_>>()
//                 .join(" ")
//         )
//     })
//     .map(|s: String| Box::leak(s.into_boxed_str()) as &'src str);
