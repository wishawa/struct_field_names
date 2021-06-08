# struct_field_names

[<img alt="crates.io" src="https://img.shields.io/crates/v/struct_field_names?style=for-the-badge" height="20">](https://crates.io/crates/struct_field_names)

Provides `StructFieldNames` derive macro.
```rust
#[derive(StructFieldNames)]
struct SomeStruct {
    field_one: i32,
    field_two: Vec<bool>,
}
```
generates
```rust
struct SomeStructFieldStaticStr {
    field_one: &'static str,
    field_two: &'static str,
}
impl SomeStruct {
    const FIELD_NAMES: SomeStructFieldStaticStr = SomeStructFieldStaticStr {
        field_one: "field_one",
        field_two: "field_two",
    };
}
```
which can be used like
```rust
let field_one_name: &'static str = SomeStruct::FIELD_NAMES.field_one;
println!("{}", field_one_name);
```
.

This is useful mostly for typo-proofing.

Credits to the [field_types crate](https://crates.io/crates/field_types). A lot of code here is copied from there.

# Usage
Use the `StructFieldNames` derive macro like this:
```rust
#[derive(StructFieldNames)]
struct SomeStruct {
    field_one: i32,
    field_two: Vec<bool>,
}
```
then access the field name as `&'static str` like this:
```rust
let field_one_name: &'static str = SomeStruct::FIELD_NAMES.field_one;
```

Use `#[struct_field_names(skip)]` to skip fields.
With
```rust
#[derive(StructFieldNames)]
struct Struct {
	field_one: bool,
	#[struct_field_names(skip)]
	field_two: usize,
}
```
`SomeStruct::FIELD_NAMES.field_two` won't exist.

Visibility of the field names struct follows your struct.

With 
```rust
#[derive(StructFieldNames)]
pub struct PublicStruct {
	pub public_field: i32,
	private_field: i32
}
#[derive(StructFieldNames)]
struct PrivateStruct {
	pub public_field: i32,
	private_field: i32
}
```
only `PublicStruct::FIELD_NAMES.public_field` would be available to the outside world.