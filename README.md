# InspectError

A handy extension trait for `Result` that brings `Result::inspect_err` to stable Rust in the form of `InspectError::inspect_error`.

Instead of this...

```rs
// ...
    .map_err(|err| {
        eprintln!("Something went wrong: '{err}'.");
        err
    })
// ...
```

...write this:

```rs
// ...
    .inspect_error(|err| eprintln!("Something went wrong: '{err}'."))
// ...
```

# Usage

Bring the `InspectError` trait into scope. This will let you call `.inspect_error()` on a `Result`.


```rs
use inspect_error::InspectError;

fn read_magic_number_from_db() -> Result<i32, &'static str> {
    // Pretend this is a real `Result`.
    let magic_number = Err("couldn't connect to the database")
        .inspect_error(|err| eprintln!("Something went wrong: '{err}'."))?;

    // Do something with the magic number.

    Ok(magic_number)
}

let output = read_magic_number_from_db();
assert_eq!(output, Err("couldn't connect to the database"));
```
