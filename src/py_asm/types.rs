use super::*;

#[derive(Clone)]
pub struct Type(pub String);

impl Type {
    pub fn raw(raw: impl ToString) -> Self {
        Self(raw.to_string())
    }

    pub fn generic_raw(raw: impl Display, generics: impl IntoIterator<Item = Self>) -> Self {
        let generics = generics.into_iter().map(|g| g.to_string()).collect::<Vec<_>>().join(", ");
        Self(format!("{raw}[{generics}]"))
    }

    pub fn unknow() -> Self {
        Self(String::new())
    }

    pub fn int() -> Self {
        Self("int".into())
    }

    pub fn bool() -> Self {
        Self("bool".into())
    }

    pub fn float() -> Self {
        Self("float".into())
    }

    pub fn str() -> Self {
        Self("str".into())
    }

    pub fn bytes() -> Self {
        Self("bytes".into())
    }

    pub fn complex() -> Self {
        Self("complex".into())
    }

    pub fn tuple(elts: impl IntoIterator<Item = Self>) -> Self {
        let elts = elts.into_iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ");
        Self(format!("tuple[{elts}]"))
    }

    pub fn var_tuple(self) -> Self {
        Self(format!("tuple[{self}, ...]"))
    }

    pub fn list(self) -> Self {
        Self(format!("list[{self}]"))
    }

    pub fn set(self) -> Self {
        Self(format!("set[{self}]"))
    }

    pub fn dict(self, v: Self) -> Self {
        Self(format!("dict[{self}, {v}]"))
    }

    /// Add a name.
    pub fn bind(self, name: impl Display) -> Self {
        Self(format!("{name}: {self}"))
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
