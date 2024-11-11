<p align="center">
    <img src="https://github.com/user-attachments/assets/c265f7e7-f8c1-43f1-b9de-370bf7d2cd1b">
</p>
<p align="center">
    <a href="https://github.com/dekirisu/buns" style="position:relative"><img src="https://img.shields.io/badge/github-dekirisu/buns-ee6677"></a><a href="https://crates.io/crates/buns" style="position:relative"><img src="https://img.shields.io/crates/v/buns"></a>
</p>

A simple way to write repeatable code anywhere, by defining buns and toppings. 🍞

## Sandwich / Compose
This can be seen as `format!()`, but for code:
1. **Buns**: Write the code, use `^0 ^1 .. ^N` as (Topping) placeholders
2. **Topping**: Write code inserts: `#0^1^..^N`, where numbers = any code
3. **Why tho?** The point is, you can repeat 2. and make infinite sandwiches. 🥪

```rust
buns::sandwich!{ 
    const ^0: u32 = ^1; // Buns
    #TEST^10 #OMEGA^59  // Toppings
}
// Will generate:
// const TEST: u32 = 10;
// const OMEGA: u32 = 59;
```

## Prepare / Preset
This can be seen as a simplified `macro_rules!{}`, where you prepare named **Buns** and add the **Toppings** later using the generated macro (The code (Buns) is automatically added to the macro documentation):
```rust
buns::prepare!{
    burger           // Name
    let a = ^0 + ^0; // Buns
    println!("{a}"); // "
}

fn main(){
    burger!{#1 #2 #4+4 #4 #2*2} // Toppings
    // prints: 2 4 16 8 8
}
```

## Flexibility
You can use any other magical token macro like [paste](https://github.com/dtolnay/paste) to add functionality:
```rust
buns::sandwich!{
    paste::paste!{const [<^1 _ ^0:upper>]: ^0 = ^2;}
    #u32^BREAD^100 #f32^BREAD^12.0
}
// Will generate:
// const BREAD_U32: u32 = 100;
// const BREAD_F32: f32 = 12.9;
```

---
### License
<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
