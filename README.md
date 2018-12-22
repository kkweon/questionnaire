# Questionnaire [WIP]

Motivated by [inquirer.js](https://github.com/SBoudrias/Inquirer.js/)


## Example

```rust
fn main() {
    prompt("what's your name?")
        .and_then(|name| {
            // do something with name ...
            ok(())
        })
        .wait();
}
```
