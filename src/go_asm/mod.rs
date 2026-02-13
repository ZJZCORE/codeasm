pub mod exprs;
pub mod package;
pub mod stmts;
pub mod types;

pub use exprs::Expr;
pub use package::{Decl, Package};
pub(super) use std::fmt::{Display, Write};
pub use stmts::{Block, Stmt};
pub use types::{IfaceFn, Type};

mod tests {
    #[test]
    fn helloworld() {
        use super::*;
        let main_body = Block::new()
            .push(Expr::raw("fmt").attr("PrintLn").call(["Hello World".into()]).into());
        let pkg = Package::new("main")
            .push(Decl::import("fmt"))
            .push(Decl::func("main", [], [], main_body));
        print!("{pkg}")
    }
}