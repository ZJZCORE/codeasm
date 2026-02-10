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

    pub fn uninit_var(name: impl Display, ty: Type) -> Self {
        Self(format!("var {name} {ty}"))
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
        args: impl IntoIterator<Item = (impl Display, Type)>,
        rets: impl IntoIterator<Item = Type>,
        body: Block,
    ) -> Self {
        let args =
            args.into_iter().map(|(arg, ty)| format!("{arg} {ty}")).collect::<Vec<_>>().join(", ");
        let mut rets = rets.into_iter().map(|r| r.to_string()).collect::<Vec<_>>().join(", ");
        if !rets.is_empty() {
            rets = format!(" {rets}")
        }
        Self(format!("func {name}({args}){rets} {body}"))
    }

    /// e.g. `func (r Receiver) Function(arg1 int, ...) int { ... }`
    pub fn method(
        name: impl Display,
        receiver: (impl Display, Type),
        args: impl IntoIterator<Item = (impl Display, Type)>,
        rets: impl IntoIterator<Item = Type>,
        body: Block,
    ) -> Self {
        let receiver = format!("{} {}", receiver.0, receiver.1);
        let args =
            args.into_iter().map(|(arg, ty)| format!("{arg} {ty}")).collect::<Vec<_>>().join(", ");
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
    pub decls: Vec<Decl>,
}

impl Package {
    pub fn new(name: impl Into<Box<str>>) -> Self {
        Self { name: name.into(), decls: Vec::new() }
    }

    pub fn push(&mut self, decl: Decl) -> &mut Self {
        self.decls.push(decl);
        self
    }

    /// save defined package to file.
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, self.to_string())
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let decls = self.decls.iter().map(|d| format!("{d}\n")).collect::<String>();
        write!(f, "package {}\n{decls}", self.name)
    }
}
