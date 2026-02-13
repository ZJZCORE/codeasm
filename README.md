# codeasm

This library can translate AST into source code for multiple programming languages.

Currently, we support the following programming languages:
- Go
- Python

## Examples

### Go Example

```rust
use codeasm::go_asm::*;

let main_body = Block::new()
    .push(Expr::raw("fmt").attr("Println").call(["Hello World".into()]).into());
let pkg = Package::new("main")
    .push(Decl::import("fmt"))
    .push(Decl::func("main", [], [], main_body));
print!("{pkg}")
```

Generated code:

```go
package main
import "fmt"
func main() {
    fmt.Println("Hello world!")
}
```

### Python Example

```rust
use codeasm::py_asm::*;

let main_body = Block::new()
    .push(Expr::raw("print").call(["Hello World".into()], Vec::<(&str, _)>::new()).into());

let if_cond = Expr::raw("__name__").binop("==", "__main__".into());
let if_body = Block::new().push(Expr::raw("main").call([], Vec::<(&str, _)>::new()).into());

let file = File::new()
    .push(Stmt::func("main", DefArgs::new(), Type::unknow(), main_body))
    .push(Stmt::if_([(if_cond, if_body)], Block::new()));
print!("{file}")
```

Generated code:

```py
def main():
    print("Hello World")

if (__name__ == "__main__"):
    main()
```
