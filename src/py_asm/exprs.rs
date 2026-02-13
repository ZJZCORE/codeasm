use super::*;
use std::ops::*;

#[derive(Clone)]
pub struct Expr(pub String);

impl Expr {
    pub fn raw(raw: impl ToString) -> Self {
        Self(raw.to_string())
    }

    pub fn ellipsis() -> Self {
        Self("...".into())
    }

    pub fn str(kind: &str, str: impl ToString) -> Self {
        Self(format!("{kind}\"{}\"", str.to_string().replace("\"", "\\\"")))
    }

    pub fn list(elts: impl IntoIterator<Item = Expr>) -> Self {
        let elts = elts.into_iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ");
        Self(format!("[{elts}]"))
    }

    pub fn tuple(elts: impl IntoIterator<Item = Expr>) -> Self {
        let elts = elts.into_iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ");
        Self(format!("({elts})"))
    }

    pub fn dict(elts: impl IntoIterator<Item = (Expr, Expr)>) -> Self {
        let elts =
            elts.into_iter().map(|(k, v)| format!("{k}: {v}")).collect::<Vec<_>>().join(", ");
        Self(format!("{{{elts}}}"))
    }

    pub fn set(elts: impl IntoIterator<Item = Expr>) -> Self {
        let elts = elts.into_iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ");
        Self(format!("{{{elts}}}"))
    }

    pub fn lambda(args: impl IntoIterator<Item = impl ToString>, body: Block) -> Self {
        let args = args.into_iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ");
        Self(format!("lambda {args}:\n{body}"))
    }

    pub fn listcomp(
        self,
        gens: impl IntoIterator<Item = (impl Display, Self)>,
        conds: impl IntoIterator<Item = Self>,
    ) -> Self {
        let gens: String =
            gens.into_iter().map(|(var, it)| format!(" for {var} in {it}")).collect();
        let conds: String = conds.into_iter().map(|cond| format!(" if {cond}")).collect();
        Self(format!("[{self}{gens}{conds}]"))
    }

    pub fn dictcomp(
        self,
        v: Self,
        gens: impl IntoIterator<Item = (impl Display, Self)>,
        conds: impl IntoIterator<Item = Self>,
    ) -> Self {
        let gens: String =
            gens.into_iter().map(|(var, it)| format!(" for {var} in {it}")).collect();
        let conds: String = conds.into_iter().map(|cond| format!(" if {cond}")).collect();
        Self(format!("{{{self}: {v}{gens}{conds}}}"))
    }

    pub fn setcomp(
        self,
        gens: impl IntoIterator<Item = (impl Display, Self)>,
        conds: impl IntoIterator<Item = Self>,
    ) -> Self {
        let gens: String =
            gens.into_iter().map(|(var, it)| format!(" for {var} in {it}")).collect();
        let conds: String = conds.into_iter().map(|cond| format!(" if {cond}")).collect();
        Self(format!("{{{self}{gens}{conds}}}"))
    }

    pub fn generator(
        self,
        gens: impl IntoIterator<Item = (impl Display, Self)>,
        conds: impl IntoIterator<Item = Self>,
    ) -> Self {
        let gens: String =
            gens.into_iter().map(|(var, it)| format!(" for {var} in {it}")).collect();
        let conds: String = conds.into_iter().map(|cond| format!(" if {cond}")).collect();
        Self(format!("({self}{gens}{conds})"))
    }

    pub fn attr(self, attr: impl Display) -> Self {
        Self(format!("{self}.{attr}"))
    }

    pub fn binop(self, op: &str, rhs: Expr) -> Self {
        Self(format!("({self} {op} {rhs})"))
    }

    pub fn index(self, index: Expr) -> Self {
        Self(format!("{self}[{index}]"))
    }

    pub fn await_(expr: Expr) -> Self {
        Self(format!("await {expr}"))
    }

    pub fn call(
        self,
        args: impl IntoIterator<Item = Self>,
        kwargs: impl IntoIterator<Item = (impl Display, Self)>,
    ) -> Self {
        let args = args.into_iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ");
        let kwargs: String =
            kwargs.into_iter().map(|(kw, v)| format!(", {kw}={v}")).collect();
        Self(format!("{self}({args}{kwargs})"))
    }
}

impl Neg for Expr {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(format!("-{self}"))
    }
}
impl Not for Expr {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(format!("not {self}"))
    }
}
impl Into<Expr> for String {
    fn into(self) -> Expr {
        Expr::str("", self)
    }
}
impl Into<Expr> for &str {
    fn into(self) -> Expr {
        Expr::str("", self)
    }
}
impl Into<Expr> for bool {
    fn into(self) -> Expr {
        if self { Expr::raw("True") } else { Expr::raw("False") }
    }
}
impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

macro_rules! impl_binop { ($($t:ident, $f:ident, $op:expr),*) => {$(
    impl $t for Expr {
        type Output = Self;
        fn $f(self, rhs: Self) -> Self::Output { self.binop($op, rhs) }
    }
)*}}
macro_rules! impl_into { ($($t:ty),*) => {$(
    impl Into<Expr> for $t { fn into(self) -> Expr { Expr::raw(self) } }
)*}}
impl_binop!(Add, add, "+", Sub, sub, "-", Mul, mul, "*", Div, div, "/");
impl_binop!(Rem, rem, "%", BitAnd, bitand, "&", BitOr, bitor, "|");
impl_binop!(BitXor, bitxor, "^", Shl, shl, "<<", Shr, shr, ">>");
impl_into!(i32, i64, i128, u32, u64, u128, f32, f64, Type);
