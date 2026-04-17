/// The value of a nix expression
pub enum NixVal {
    /// Submodule DSL
    Submodule(Box<Module>),

    /// Has to be done in nix land
    Evaluatable(String),
}

pub struct MkOption {
    /// Default value when no definition is given
    pub default: NixVal,

    /// Textual substitute for the default in docs
    pub default_text: String,

    /// Example value for the manual
    pub example: String,

    /// String describing the option (required for docs generation)
    pub description: String,

    /// Related packages for the manual
    pub related_packages: Vec<String>,

    /// Option type — provides type-checking and value merging
    pub nix_type: NixVal,

    /// Function to transform the option value after merge
    pub apply: NixVal,

    /// Boolean — hides option from generated documentation
    pub internal: bool,

    /// Visibility control: true (default), false (hide + sub-options),"shallow" (hide sub-options only), "transparent" (hide self but not sub-options)
    pub visible: NixVal,

    /// Boolean — option can only be set once
    pub read_only: bool,
}

pub struct Module {
    /// Option definitions for the module
    pub options: Vec<MkOption>,

    /// The generated name for the module
    pub name: NixVal,
}

/// Schemix expression
pub enum Expr {
    /// Module definition
    Module(Module),

    /// Code comment
    Comment(String),
}
