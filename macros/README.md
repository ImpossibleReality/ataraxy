# Macro Crate
The macro crate exports the command macro, as well as IDE support macros to improve development experience with command types.

*Note: It isn't recommended to use this crate directly but rather to use the re-exported command macro in the main create*

### IDE Support

Because the `#[command]` macro changes function signatures, some IDEs do not track this (looking at you CLion), and report subsequent `Framework::command(cmd)` calls as invalid. To fix this I added a procedural macro in the crate that changes the signature of the `Framework::command` function from `Any` to an acceptable command type.