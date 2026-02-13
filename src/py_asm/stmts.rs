use super::*;

#[derive(Clone, Default)]
pub struct Block(pub String);

impl Block {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(self, stmt: Stmt) -> Self {
        Self(format!("{self}    {stmt}\n"))
    }

    /// push stmt to `&mut Block`
    pub fn pushs(&mut self, stmt: Stmt) -> &mut Self {
        writeln!(self.0, "    {stmt}").unwrap();
        self
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone)]
pub struct Stmt(pub String);

impl Stmt {
    pub fn raw(raw: impl ToString) -> Self {
        Self(raw.to_string())
    }

    pub fn pass() -> Self {
        Self("pass".into())
    }

    /// break
    pub fn brk() -> Self {
        Self("break".into())
    }

    /// continue
    pub fn cont() -> Self {
        Self("continue".into())
    }

    pub fn del(expr: Expr) -> Self {
        Self(format!("del {expr}"))
    }

    pub fn ret(val: Expr) -> Self {
        Self(format!("return {val}"))
    }

    pub fn assert(cond: Expr, info: Option<Expr>) -> Self {
        let info = info.map_or(String::new(), |i| format!(", {i}"));
        Self(format!("assert {cond}, {info}"))
    }

    pub fn raise(err: Expr) -> Self {
        Self(format!("raise {err}"))
    }

    pub fn yield_(expr: Option<Expr>) -> Self {
        Self(format!("yield {}", expr.map_or(String::new(), |e| e.to_string())))
    }

    pub fn try_(
        body: Block,
        excepts: impl IntoIterator<Item = (Type, impl ToString, Block)>,
        else_: Block,
        finally: Block,
    ) -> Self {
        let mut res = format!("try:\n{body}");
        for (exc_ty, exc_name, body) in excepts {
            let exc_name = exc_name.to_string();
            let exc = if exc_name.is_empty() {
                exc_ty.to_string()
            } else {
                format!("{exc_ty} as {exc_name}")
            };
            write!(res, "except {exc}:\n{body}").unwrap()
        }
        if !else_.0.is_empty() {
            write!(res, "else:\n{else_}").unwrap();
        }
        if !finally.0.is_empty() {
            write!(res, "finally:\n{finally}").unwrap();
        }
        Self(res)
    }

    pub fn with(expr: Expr, var: impl ToString, body: Block, else_: Block) -> Self {
        let mut var = var.to_string();
        if !var.is_empty() {
            var = format!(" as {var}")
        }
        if else_.0.is_empty() {
            Self(format!("with {expr}{var}:\n{body}"))
        } else {
            Self(format!("with {expr}{var}:\n{body}else:\n{else_}"))
        }
    }
    pub fn async_with(expr: Expr, var: impl ToString, body: Block, else_: Block) -> Self {
        Self(format!("async {}", Self::with(expr, var, body, else_)))
    }

    pub fn if_(cases: impl IntoIterator<Item = (Expr, Block)>, else_: Block) -> Self {
        let mut res = String::new();
        let mut cases = cases.into_iter();
        let (cond, block) = cases.next().expect("`cases` is empty");
        write!(res, "if {cond}:\n{block}").unwrap();
        for (cond, block) in cases {
            write!(res, "elif {cond}:\n{block}").unwrap();
        }
        if !else_.0.is_empty() {
            write!(res, "else:\n{else_}").unwrap();
        }
        Self(res)
    }

    pub fn while_(cond: Expr, body: Block, else_: Block) -> Self {
        if else_.0.is_empty() {
            Self(format!("while {cond}:\n{body}"))
        } else {
            Self(format!("while {cond}:\n{body}else:\n{else_}"))
        }
    }

    pub fn for_(
        var_iters: impl IntoIterator<Item = (impl Display, Expr)>,
        body: Block,
        else_: Block,
    ) -> Self {
        let (mut vars, mut iters) = (String::new(), String::new());
        for (var, iter) in var_iters {
            if vars.is_empty() {
                write!(vars, "{var}").unwrap();
                write!(iters, "{iter}").unwrap()
            } else {
                write!(vars, ", {var}").unwrap();
                write!(iters, ", {iter}").unwrap()
            }
        }
        if else_.0.is_empty() {
            Self(format!("for {vars} in {iters}:\n{body}"))
        } else {
            Self(format!("for {vars} in {iters}:\n{body}else:\n{else_}"))
        }
    }
    pub fn async_for(
        var_iters: impl IntoIterator<Item = (impl Display, Expr)>,
        body: Block,
        else_: Block,
    ) -> Self {
        Self(format!("async {}", Self::for_(var_iters, body, else_)))
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
