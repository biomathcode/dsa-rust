## Patterns in rust


### Match with Some and None
the match expression is a powerful and flexible way to handle different patterns. It is often used with Some and None variants of the Option enum to handle the presence or absence of a value. 


Pattern: 
```rs
match () => {
    Some {

    }
    None {

    }
}
```

Example: 
```rs
fn main() {
    let maybe_number: Option<i32> = Some(42);

    match maybe_number {
        Some(number) => {
            println!("Found a number: {}", number);
            // Code to execute when the value is present
        }
        None => {
            println!("No number found");
            // Code to execute when the value is absent
        }
    }
}
```

### If Let

The if let pattern in Rust is a concise way to match and destructure enums or other types in a single line, avoiding the need for a more verbose match statement:

Pattern

```rs
if let Some(value) = some_result {
    // Code to execute if the pattern matches
} else {
    // Code to execute if the pattern doesn't match
}
```


Example: 
```rs
enum MyEnum {
    Variant1(i32),
    Variant2(String),
    Variant3,
}

fn main() {
    let my_value = MyEnum::Variant1(42);

    if let MyEnum::Variant1(number) = my_value {
        println!("Found Variant1 with number: {}", number);
    } else {
        println!("Didn't match Variant1");
    }
}
```



### Option Enum

