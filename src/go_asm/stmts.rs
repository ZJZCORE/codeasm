use super::*;

#[derive(Clone)]
pub struct Block(pub Vec<Stmt>);

impl Block {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, stmt: Stmt) -> &mut Self {
        self.0.push(stmt);
        self
    }

    pub fn fmt_no_brace(&self, f: &mut impl Write) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|s| format!("    {s}\n")).collect::<String>())
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n{}}}", self.0.iter().map(|s| format!("    {s}\n")).collect::<String>())
    }
}

#[derive(Clone)]
pub struct Stmt(String);

impl Stmt {
    pub fn raw(raw: impl ToString) -> Self {
        Self(raw.to_string())
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

    pub fn if_(cases: impl IntoIterator<Item = (Expr, Block)>, else_: Option<Block>) -> Self {
        let mut res = String::new();
        let mut cases = cases.into_iter();
        let (cond, block) = cases.next().expect("`cases` is empty");
        write!(res, "if {cond} {block}").unwrap();
        for (cond, block) in cases {
            write!(res, " else if {cond} {block}").unwrap();
        }
        if let Some(else_) = else_ {
            write!(res, " else {else_}").unwrap();
        }
        Self(res)
    }

    pub fn switch(
        val: Expr,
        cases: impl IntoIterator<Item = (Expr, Block)>,
        default: Option<Block>,
    ) -> Self {
        let mut res = format!("switch {val} {{");
        for (v, block) in cases {
            writeln!(res, "case {v}:").unwrap();
            block.fmt_no_brace(&mut res).unwrap();
        }
        if let Some(default) = default {
            writeln!(res, "default:").unwrap();
            default.fmt_no_brace(&mut res).unwrap();
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

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
