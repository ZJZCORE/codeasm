# codeasm

This library can translate AST into source code for multiple programming languages.

Currently, we support the following programming languages:
- Go

## Examples

### Go Example

```rust
let mut pkg = Package::new("main");
let mut main_body = Block::new();
main_body
    .push(Expr::raw("fmt").attr("Println").call(["Hello world!".into()]).into());
pkg
    .push(Decl::import("fmt"))
    .push(Decl::func("main", Vec::<(&str, _)>::new(), [], main_body));
println!("{pkg}")
```

Generated golang code:

```go
package main
import "fmt"
func main() {
    fmt.Println("Hello world!")
}
```
