use super::*;

pub struct IfaceFn(pub String);

impl IfaceFn {
    pub fn new(
        name: impl Display,
        args: impl IntoIterator<Item = (impl Display, Type)>,
        ret: Option<Type>,
    ) -> Self {
        let args =
            args.into_iter().map(|(arg, ty)| format!("{arg} {ty}")).collect::<Vec<_>>().join(", ");
        let ret = ret.map_or(String::new(), |r| r.to_string());
        Self(format!("{name}({args}) {ret}"))
    }
}

#[derive(Debug, Clone)]
pub struct Type(pub String);

impl Type {
    pub fn bool() -> Self {
        Self("bool".into())
    }

    pub fn string() -> Self {
        Self("string".into())
    }

    pub fn int(bit: u8) -> Self {
        Self(format!("int{bit}"))
    }

    pub fn uint(bit: u8) -> Self {
        Self(format!("uint{bit}"))
    }

    pub fn map(k: Self, v: Self) -> Self {
        Self(format!("map[{k}]{v}"))
    }

    pub fn custom(s: impl ToString) -> Self {
        Self(s.to_string())
    }

    pub fn chan(self) -> Self {
        Self(format!("chan {self}"))
    }

    pub fn ptr(self) -> Self {
        Self(format!("*{self}"))
    }

    pub fn fn_ptr(self, args: impl IntoIterator<Item = Type>) -> Self {
        let args = args.into_iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ");
        Self(format!("func({args}) {self}"))
    }

    pub fn array(self, size: u32) -> Self {
        Self(format!("[{size}]{self}"))
    }

    pub fn interface(
        embeds: impl IntoIterator<Item = impl Display>,
        methods: impl IntoIterator<Item = IfaceFn>,
    ) -> Self {
        let embeds: String = embeds.into_iter().map(|e| format!("{}\n", e)).collect();
        let members: String = methods.into_iter().map(|m| format!("{}\n", m.0)).collect();
        if members.is_empty() && embeds.is_empty() {
            Self("interface{}".into())
        } else {
            Self(format!("interface {{\n{embeds}{members}}}"))
        }
    }

    pub fn struct_(
        embeds: impl IntoIterator<Item = impl Display>,
        fields: impl IntoIterator<Item = (impl Display, Type)>,
    ) -> Self {
        let embeds: String = embeds.into_iter().map(|e| format!("{}\n", e)).collect();
        let members: String = fields.into_iter().map(|f| format!("{} {}\n", f.0, f.1)).collect();
        if members.is_empty() && embeds.is_empty() {
            Self("struct{}".into())
        } else {
            Self(format!("struct {{\n{embeds}{members}}}"))
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
