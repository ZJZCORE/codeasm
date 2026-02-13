use super::*;

#[derive(Clone)]
pub struct Decl(pub String);

impl Decl {
    pub fn raw(raw: impl ToString) -> Self {
        Self(raw.to_string())
    }

    pub fn import(lib: impl Display) -> Self {
        Self(format!("import \"{lib}\""))
    }

    pub fn variable(name: impl Display, val: Expr) -> Self {
        Self(format!("var {name} = {val}"))
    }

    pub fn uninit_var(bind: Type) -> Self {
        Self(format!("var {bind}"))
    }

    pub fn const_(name: impl Display, val: Expr) -> Self {
        Self(format!("const {name} = {val}"))
    }

    /// e.g. `type TypeName struct{}`
    pub fn type_(name: impl Display, ty: Type) -> Self {
        Self(format!("type {name} {ty}"))
    }

    /// e.g. `func Function(arg1 int, ...) int { ... }`
    pub fn func(
        name: impl Display,
        args: impl IntoIterator<Item = Type>,
        rets: impl IntoIterator<Item = Type>,
        body: Block,
    ) -> Self {
        let args = args.into_iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ");
        let mut rets = rets.into_iter().map(|r| r.to_string()).collect::<Vec<_>>().join(", ");
        if !rets.is_empty() {
            rets = format!(" {rets}")
        }
        Self(format!("func {name}({args}){rets} {body}"))
    }

    /// e.g. `func (r Receiver) Function(arg1 int, ...) int { ... }`
    pub fn method(
        name: impl Display,
        receiver: Type,
        args: impl IntoIterator<Item = Type>,
        rets: impl IntoIterator<Item = Type>,
        body: Block,
    ) -> Self {
        let args = args.into_iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ");
        let mut rets = rets.into_iter().map(|r| r.to_string()).collect::<Vec<_>>().join(", ");
        if !rets.is_empty() {
            rets = format!(" {rets}")
        }
        Self(format!("func ({receiver}) {name}({args}){rets} {body}"))
    }
}

impl Display for Decl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone)]
pub struct Package {
    pub name: Box<str>,
    pub decls: String,
}

impl Package {
    pub fn new(name: impl Into<Box<str>>) -> Self {
        Self { name: name.into(), decls: String::new() }
    }

    pub fn push(self, decl: Decl) -> Self {
        Self { name: self.name, decls: format!("{}{decl}\n", self.decls) }
    }

    /// push decl to `&mut Package`
    pub fn pushs(&mut self, decl: Decl) -> &mut Self {
        writeln!(self.decls, "{decl}").unwrap();
        self
    }

    /// save defined package to file.
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, self.to_string())
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "package {}\n{}", self.name, self.decls)
    }
}
