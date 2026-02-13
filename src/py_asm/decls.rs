use super::*;

#[derive(Clone, Default)]
pub struct DefArgs(pub String);

impl DefArgs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_arg(self, name: impl Display, default: Option<Expr>) -> Self {
        let default = default.map_or(String::new(), |d| format!("={d}"));
        if self.0.is_empty() {
            Self(format!("{name}{default}"))
        } else {
            Self(format!("{name}{default}, "))
        }
    }

    pub fn push_vararg(self, name: impl Display) -> Self {
        if self.0.is_empty() { Self(format!("*{name}")) } else { Self(format!("*{name}, ")) }
    }

    pub fn push_kwarg(self, name: impl Display) -> Self {
        if self.0.is_empty() { Self(format!("**{name}")) } else { Self(format!("**{name}, ")) }
    }
}

impl Display for DefArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.0)
    }
}

impl Stmt {
    /// decorator. e.g. `@dataclasses.dataclass`
    pub fn dec(expr: Expr) -> Self {
        Self(format!("@{expr}"))
    }

    pub fn import(module: impl Display) -> Self {
        Self(format!("import {module}"))
    }
    pub fn from_import(module: impl Display, item: impl Display) -> Self {
        Self(format!("from {module} import {item}"))
    }

    pub fn assign(name: impl Display, val: Expr) -> Self {
        Self(format!("{name} = {val}"))
    }

    pub fn global(var: impl Display) -> Self {
        Self(format!("global {var}"))
    }

    pub fn nonlocal(var: impl Display) -> Self {
        Self(format!("nonlocal {var}"))
    }

    pub fn func(name: impl Display, args: DefArgs, ret: Type, body: Block) -> Self {
        let ret = if ret.0.is_empty() { String::new() } else { format!(" -> {ret}") };
        Self(format!("def {name}{args}{ret}:\n{body}"))
    }

    pub fn async_func(name: impl Display, args: DefArgs, ret: Type, body: Block) -> Self {
        Self(format!("async {}", Self::func(name, args, ret, body)))
    }

    pub fn class(
        name: impl Display,
        inherit: impl IntoIterator<Item = impl ToString>,
        body: Block,
    ) -> Self {
        let inherit = inherit.into_iter().map(|i| i.to_string()).collect::<Vec<_>>().join(", ");
        if inherit.is_empty() {
            Self(format!("class {name}:\n{body}"))
        } else {
            Self(format!("class {name}({inherit}):\n{body}"))
        }
    }
}

#[derive(Clone, Default)]
pub struct File(pub String);

impl File {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(self, stmt: Stmt) -> Self {
        Self(format!("{self}{stmt}\n"))
    }

    /// push stmt to `&mut File`
    pub fn pushs(&mut self, stmt: Stmt) -> &mut Self {
        writeln!(self.0, "{stmt}").unwrap();
        self
    }

    /// Save to file.
    pub fn save(self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, self.to_string())
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
