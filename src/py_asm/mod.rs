pub mod exprs;
pub mod stmts;
pub mod types;
pub mod decls;

pub use decls::{DefArgs, File};
pub use exprs::Expr;
pub(super) use std::fmt::{Display, Write};
pub use stmts::{Block, Stmt};
pub use types::Type;

mod tests {
    #[test]
    fn helloworld() {
        use super::*;
        let main_body = Block::new()
            .push(Expr::raw("print").call(["Hello World".into()], Vec::<(&str, _)>::new()).into());
        let file = File::new()
            .push(Stmt::func("main", DefArgs::new(), Type::unknow(), main_body))
            .push(Stmt::if_([(
                Expr::raw("__name__").binop("==", "__main__".into()),
                Block::new().push(Expr::raw("main").call([], Vec::<(&str, _)>::new()).into())
            )], Block::new()));
        print!("{file}")
    }
}
