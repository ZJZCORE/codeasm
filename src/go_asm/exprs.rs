use super::*;
use std::ops::*;

#[derive(Clone)]
pub struct Expr(pub String);

impl Expr {
    pub fn raw(raw: impl ToString) -> Self {
        Self(raw.to_string())
    }

    pub fn str(str: impl ToString) -> Self {
        Self(format!("\"{}\"", str.to_string().replace("\"", "\\\"")))
    }

    pub fn attr(self, attr: impl Display) -> Self {
        Self(format!("{self}.{attr}"))
    }

    /// Create a type assertion expression. e.g. `x.(int)`
    pub fn type_as(self, ty: Type) -> Self {
        Self(format!("{self}.({ty})"))
    }

    /// Create a type switch expression. e.g. `x.(type)`
    pub fn type_sw(self) -> Self {
        Self(format!("{self}.(type)"))
    }

    pub fn range(self) -> Self {
        Self(format!("range {self}"))
    }

    pub fn binop(self, op: &str, rhs: Expr) -> Self {
        Self(format!("({self} {op} {rhs})"))
    }

    pub fn index(self, index: Expr) -> Self {
        Self(format!("{self}[{index}]"))
    }

    /// Create a ​keyed composite literal. e.g. `A{a: 1, b: 2, ...}`
    pub fn complit(ty: Type, vals: impl IntoIterator<Item = (impl Display, Expr)>) -> Self {
        let vals = vals
            .into_iter()
            .map(|(name, val)| format!("{name}: {val}"))
            .collect::<Vec<_>>()
            .join(", ");
        Self(format!("{ty}{{{vals}}}"))
    }

    /// Create a ​unkeyed composite literal. e.g. `A{1, 2, ...}`
    pub fn ord_complit(ty: Type, vals: impl IntoIterator<Item = Expr>) -> Self {
        let vals = vals.into_iter().map(|val| val.to_string()).collect::<Vec<_>>().join(", ");
        Self(format!("{ty}{{{vals}}}"))
    }

    /// Create a closure. e.g. `func(){}`
    pub fn closure(
        args: impl IntoIterator<Item = Type>,
        rets: impl IntoIterator<Item = Type>,
        body: Block,
    ) -> Self {
        let args = args.into_iter().map(|arg| arg.to_string()).collect::<Vec<_>>().join(", ");
        let mut rets = rets.into_iter().map(|r| r.to_string()).collect::<Vec<_>>().join(", ");
        if !rets.is_empty() {
            rets = format!(" {rets}")
        }
        Self(format!("func({args}){rets} {body}"))
    }

    /// Create a function call expr.
    pub fn call(self, args: impl IntoIterator<Item = Expr>) -> Self {
        let args: Vec<_> = args.into_iter().map(|e| e.to_string()).collect();
        Self(format!("{self}({})", args.join(", ")))
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
        Self(format!("!{self}"))
    }
}
impl Into<Expr> for String {
    fn into(self) -> Expr {
        Expr::str(self)
    }
}
impl Into<Expr> for &str {
    fn into(self) -> Expr {
        Expr::str(self)
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
impl_into!(i32, i64, i128, u32, u64, u128, f32, f64, bool, Type);
