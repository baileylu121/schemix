use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use askama::Template;
use thiserror::Error;

/// The value of a nix expression
#[derive(Debug, PartialEq, Eq)]
pub enum NixVal<'src> {
    /// Submodule DSL
    // Submodule(Box<Module<'src>>),

    /// Has to be done in nix land
    Evaluatable(&'src str),
}

impl<'src> Display for NixVal<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let output = match self {
            Self::Evaluatable(val) => val,
        };

        write!(f, "{}", output)
    }
}

#[derive(Debug, Default, Clone)]
pub enum Visibility {
    #[default]
    Visible,
    Invisible,
    Shallow,
    Transparent,
}

#[derive(Error, Debug)]
pub enum VisibilityError {
    #[error(
        "
        Unknown visibility quantifier `{0}`.

        Allowed visibility quantifiers:
         - @visible
         - @invisible
         - @shallow
         - @transparent
    "
    )]
    Unknown(String),
}

impl FromStr for Visibility {
    type Err = VisibilityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "@visible" => Ok(Self::Visible),
            "@invisible" => Ok(Self::Invisible),
            "@shallow" => Ok(Self::Shallow),
            "@transparent" => Ok(Self::Transparent),
            _ => Err(VisibilityError::Unknown(s.to_string())),
        }
    }
}

impl Display for Visibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let output = match self {
            Self::Visible => "true",
            Self::Invisible => "false",
            Self::Shallow => "\"shallow\"",
            Self::Transparent => "\"transparent\"",
        };

        write!(f, "{}", output)
    }
}

/// Equivalent to `lib.mkOption`
#[derive(Default, Debug, Template)]
#[template(path = "option.nix_template")]
pub struct MkOption<'src> {
    /// Option name (e.g. "enable", "port")
    pub name: &'src str,

    /// Default value when no definition is given
    pub default: Option<NixVal<'src>>,

    /// Textual substitute for the default in docs
    pub default_text: Option<&'src str>,

    /// Example value for the manual
    pub example: Option<&'src str>,

    /// &'src str describing the option (required for docs generation)
    pub description: Option<&'src str>,

    /// Related packages for the manual
    pub related_packages: Vec<&'src str>,

    /// Option type — provides type-checking and value merging
    pub nix_type: Option<NixVal<'src>>,

    /// Function to transform the option value after merge
    pub apply: Option<NixVal<'src>>,

    /// Boolean — hides option from generated documentation
    pub internal: bool,

    /// Visibility control: true (default), false (hide + sub-options),"shallow" (hide sub-options only), "transparent" (hide self but not sub-options)
    pub visible: Visibility,

    /// Boolean option can only be set once
    pub read_only: bool,
}

/// Schemix module definition
#[derive(Debug, Template)]
#[template(path = "module.nix_template")]
pub struct Module<'src> {
    /// Option definitions for the module
    pub options: Vec<MkOption<'src>>,

    /// The generated name for the module
    pub name: NixVal<'src>,

    /// The modules description
    pub description: Option<&'src str>,
}

/// Schemix expression
#[derive(Debug, Template)]
#[template(path = "expr.nix_template")]
pub enum Expr<'src> {
    /// Module definition
    Module(Module<'src>),

    /// Code comment
    Comment(&'src str),
}
