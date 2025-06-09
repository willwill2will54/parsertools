# Parsertools
I wrote this library for the "write a compiler" coursework every compilers course gives you.
I'm open sourcing it so that I can better explain parser combinators to friends.

# HEY YOU
## Yes, you with the coursework, looking for shortcuts.
Write this yourself. My implementation is less than 500 lines of code (and I kinda over did it tbh), its not that hard, and you'll have fun doing it.
To give you inspiration for your API design, here's an example of how use my one (p.s. I left out some important things for optimising performance so your code will run really really slowly if you just copy mine):
```rust
use parsertools::{Parser, lazy, pred, tok};

type Token = char;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Hello {
    to: Vec<String>,
    from: String,
}

fn main() {
    let parser = hello_parser();
    for line in std::io::stdin().lines().map(|line| line.unwrap()) {
        let chars: Vec<_> = line.chars().collect();
        let result = parser.parse_all(&chars);

        match result {
            Ok(hello) => println!("Hello from {} to {}", hello.from, hello.to.join(" and ")),
            Err(err) => println!("Error: {}", err),
        }
    }
}

fn hello_parser() -> Parser<'static, Token, Hello> {
    word_parser()
        .then(string_parser(" says hello to "))
        .map(|(name, _)| name)
        .then(names_parser())
        .map(|(name, names)| Hello {
            to: names,
            from: name,
        })
}

fn names_parser() -> Parser<'static, Token, Vec<String>> {
    word_parser().map(|name| vec![name]).or(word_parser()
        .then(string_parser(" and "))
        .then(lazy(names_parser))
        .map(|((first_name, _), rest)| {
            let mut names = vec![first_name];
            names.extend(rest);
            names
        }))
}

fn word_parser() -> Parser<'static, Token, String> {
    single_letter()
        .then(lazy(word_parser))
        .map(|(first_letter, rest)| first_letter + &rest)
        .or(single_letter())
}

fn single_letter() -> Parser<'static, Token, String> {
    pred(|token: &char| {
        if *token != ' ' {
            Some(token.to_string())
        } else {
            None
        }
    })
}

fn string_parser(input: &str) -> Parser<'static, Token, ()> {
    let chars: Vec<char> = input.chars().collect();
    string_parser_inner(&chars)
}

fn string_parser_inner(value: &[char]) -> Parser<'static, Token, ()> {
    if value.is_empty() {
        panic!("Empty string");
    } else if value.len() == 1 {
        tok(value[0])
    } else {
        tok(value[0])
            .then(string_parser_inner(&value[1..]))
            .map(|_| ())
    }
}
```
