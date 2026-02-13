use super::*;

#[derive(Clone, Default)]
pub struct Block(pub String);

impl Block {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(self, stmt: Stmt) -> Self {
        Self(format!("{}    {stmt}\n", self.0))
    }

    /// push stmt to `&mut Block`
    pub fn pushs(&mut self, stmt: Stmt) -> &mut Self {
        writeln!(self.0, "{stmt}").unwrap();
        self
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n{}}}", self.0)
    }
}

#[derive(Clone)]
pub struct Stmt(pub String);

impl Stmt {
    pub fn raw(raw: impl ToString) -> Self {
        Self(raw.to_string())
    }

    /// break
    pub fn brk() -> Self {
        Self("break".into())
    }

    /// continue
    pub fn cont() -> Self {
        Self("continue".into())
    }

    /// fallthrough
    pub fn fall() -> Self {
        Self("fallthrough".into())
    }

    pub fn defer(call: Expr) -> Self {
        Self(format!("defer {call}"))
    }

    pub fn ret(vals: impl IntoIterator<Item = Expr>) -> Self {
        let vals = vals.into_iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ");
        Self(format!("return {vals}"))
    }

    pub fn variable(name: impl Display, val: Expr) -> Self {
        Self(format!("{name} := {val}"))
    }

    pub fn uninit_var(name: impl Display, ty: Type) -> Self {
        Self(format!("var {name} {ty}"))
    }

    pub fn variables(
        names: impl IntoIterator<Item = impl Display>,
        vals: impl IntoIterator<Item = Expr>,
    ) -> Self {
        let names = names.into_iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ");
        let vals = vals.into_iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ");
        Self(format!("{names} := {vals}"))
    }

    pub fn const_(name: impl Display, val: Expr) -> Self {
        Self(format!("const {name} = {val}"))
    }

    pub fn block(stmts: impl IntoIterator<Item = Stmt>) -> Self {
        Self(format!(
            "{{\n{}}}",
            stmts.into_iter().map(|s| format!("    {s}\n")).collect::<String>()
        ))
    }

    pub fn if_(cases: impl IntoIterator<Item = (Expr, Block)>, else_: Block) -> Self {
        let mut res = String::new();
        let mut cases = cases.into_iter();
        let (cond, block) = cases.next().expect("`cases` is empty");
        write!(res, "if {cond} {block}").unwrap();
        for (cond, block) in cases {
            write!(res, " else if {cond} {block}").unwrap();
        }
        if !else_.0.is_empty() {
            write!(res, " else {else_}").unwrap();
        }
        Self(res)
    }

    pub fn switch(
        val: Expr,
        cases: impl IntoIterator<Item = (Expr, Block)>,
        default: Block,
    ) -> Self {
        let mut res = format!("switch {val} {{");
        for (v, block) in cases {
            write!(res, "case {v}:\n{}", block.0).unwrap();
        }
        if !default.0.is_empty() {
            write!(res, "default:\n{}", default.0).unwrap();
        }
        Self(format!("{res}}}"))
    }

    /// e.g. `for i := 0; i <= 10; i++ { sum += i }`
    pub fn for_(init: Option<Stmt>, cond: Option<Expr>, post: Option<Stmt>, body: Block) -> Self {
        let has_init = init.is_some();
        let has_cond = cond.is_some();
        let has_post = post.is_some();

        match (has_init, has_cond, has_post) {
            (true, true, true) => {
                Self(format!("for {}; {}; {} {body}", init.unwrap(), cond.unwrap(), post.unwrap()))
            }
            (true, false, false) => Self(format!("for {} {body}", init.unwrap())),
            (false, true, false) => Self(format!("for {} {body}", cond.unwrap())),
            (false, false, false) => Self(format!("for {body}")),
            _ => panic!("invalid for loop"),
        }
    }
}

impl Into<Stmt> for Expr {
    fn into(self) -> Stmt {
        Stmt::raw(self)
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
