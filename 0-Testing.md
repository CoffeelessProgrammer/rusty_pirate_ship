# Unit Testing in Rust
**Explore:** [Home](/README.md) [Basics](/0-Basics.md)

## Cheatsheet
- `cargo test <testname_substring?>`
- `cargo test -- --show-output`
- `cargo test -- --ignored`
- `cargo test -- --test-threads=1`


### Example
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
```

## Resources
- [Testing | Rust By Example](https://doc.rust-lang.org/rust-by-example/testing.html)
- [Ch 11.3 Test Organization | The Rust Book](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
- [Guide to Testing in Rust | ZTM](https://zerotomastery.io/blog/complete-guide-to-testing-code-in-rust/)
- [Integration Testing | Shuttle.dev](https://www.shuttle.dev/blog/2024/03/21/testing-in-rust#rust-integration-testing)