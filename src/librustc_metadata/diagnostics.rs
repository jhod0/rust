// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_snake_case)]

register_long_diagnostics! {
E0454: r##"
A link name was given with an empty name. Erroneous code example:

```
#[link(name = "")] extern {} // error: #[link(name = "")] given with empty name
```

The rust compiler cannot link to an external library if you don't give it its
name. Example:

```
#[link(name = "some_lib")] extern {} // ok!
```
"##,

E0455: r##"
Linking with `kind=framework` is only supported when targeting OS X,
as frameworks are specific to that operating system.

Erroneous code example:

```compile_fail"
#[link(name = "FooCoreServices",  kind = "framework")] extern {}
// OS used to compile is Linux for example
```

To solve this error you can use conditional compilation:

```
#[cfg_attr(target="macos", link(name = "FooCoreServices", kind = "framework"))]
extern {}
```

See more: https://doc.rust-lang.org/book/conditional-compilation.html
"##,

E0458: r##"
An unknown "kind" was specified for a link attribute. Erroneous code example:

```
#[link(kind = "wonderful_unicorn")] extern {}
// error: unknown kind: `wonderful_unicorn`
```

Please specify a valid "kind" value, from one of the following:
 * static
 * dylib
 * framework
"##,

E0459: r##"
A link was used without a name parameter. Erroneous code example:

```
#[link(kind = "dylib")] extern {}
// error: #[link(...)] specified without `name = "foo"`
```

Please add the name parameter to allow the rust compiler to find the library
you want. Example:

```
#[link(kind = "dylib", name = "some_lib")] extern {} // ok!
```
"##,

E0463: r##"
A plugin/crate was declared but cannot be found. Erroneous code example:

```
#![feature(plugin)]
#![plugin(cookie_monster)] // error: can't find crate for `cookie_monster`
extern crate cake_is_a_lie; // error: can't find crate for `cake_is_a_lie`
```

You need to link your code to the relevant crate in order to be able to use it
(through Cargo or the `-L` option of rustc example). Plugins are crates as
well, and you link to them the same way.
"##,

E0466: r##"
Invalid macro import declarations.

Causes of this error:

```ignore
#[macro_use(a_macro(another_macro))]    // error: invalid import declaration
extern crate some_crate;

#[macro_use(i_want = "some_macros")]    // error: invalid import declaration
extern crate another_crate;
```

This is a syntax error at the level of attribute declarations.

The proper syntax for macro imports is the following:

```ignore
// // some_crate contains:
// #[macro_export]
// macro_rules! get_tacos {
//     ...
// }
//
// #[macro_export]
// macro_rules! bring_beer {
//     ...
// }
#[macro_use(get_tacos, bring_beer)]     // imports macros get_tacos and
extern crate some_crate;                // bring_beer from some_crate
```

If you would like to import all exported macros, write `macro_use` with no
arguments.
"##,

E0467: r##"
Invalid or no macros listed for reexport.

Causes of this error:

```ignore
#[macro_reexport]                   // error: no macros listed for export
extern crate macros_for_good;
```
```ignore
#[macro_reexport(fun_macro = "foo")] // error: not a macro identifier
extern crate macros_for_good;
```

This is a syntax error at the level of attribute declarations.

Currently, `macro_reexport` requires at least one macro name to be listed.
Unlike `macro_use`, listing no names does not reexport all macros from the
given crate.

Decide which macros you would like to export and list them properly.

These are proper reexport declarations:

```ignore
#[macro_reexport(some_macro, another_macro)]
extern crate macros_for_good;
```
"##,

E0468: r##"
A non-root module attempts to import macros from another crate.

Example of erroneous code:

```ignore
mod foo {
    #[macro_use(helpful_macro)] // error: must be at crate root to import
    extern crate some_crate;    //        macros from another crate
    helpful_macro!(...)
}

fn main() {
    // ...
}
```

Only `extern crate` imports at the crate root level (i.e., in lib.rs) are
allowed to import macros.

Either move the macro import to crate root or do without the foreign macros.

This will work:

```ignore
#[macro_use(helpful_macro)]
extern crate some_crate;
mod foo {
    helpful_macro!(...)
}

fn main() {
    //...
}
```

"##,

E0469: r##"
A macro listed for import was not found.

Example of erroneous code:

```ignore
/// // crate some_crate contains:
/// #[macro_export]
/// macro_rules! eat {
///     ...
/// }
/// macro_rules! drink {
///     ...
/// }

// error: drink is a private macro of some_crate
// error: be_merry does not exist in some_crate
#[macro_use(drink, be_merry)]
extern crate some_crate;
```

Either the listed macro is not contained in the imported crate, or it is not
exported from the given crate.

This could be caused by a typo. Did you misspell the macro's name?

Double-check the names of the macros listed for import, and that the crate
in question exports them.

A working version of the above:

```ignore
/// // crate some_crate contains:
/// #[macro_export]
/// macro_rules! eat {
///     ...
/// }
/// #[macro_export]
/// macro_rules! drink {
///     ...
/// }

#[macro_use(eat, drink)]
extern crate some_crate;
```
"##,

E0470: r##"
A macro listed for reexport was not found.

Example of erroneous code:

```ignore
/// // crate some_crate contains:
/// #[macro_export]
/// macro_rules! eat {
///     ...
/// }
/// macro_rules! drink {
///     ...
/// }

// error: drink is a private macro of some_crate
// error: be_merry does not exist in some_crate
#[macro_reexport(drink, be_merry)]
extern crate some_crate;
```

Either the listed macro is not contained in the imported crate, or it is not
exported from the given crate.

This could be caused by a typo. Did you misspell the macro's name?

Double-check the names of the macros listed for reexport, and that the crate
in question exports them.

A working version of the above:

```ignore
/// // crate some_crate contains:
/// #[macro_export]
/// macro_rules! eat {
///     ...
/// }
/// #[macro_export]
/// macro_rules! drink {
///     ...
/// }

#[macro_reexport(eat, drink)]
extern crate some_crate;
```
"##,

}

register_diagnostics! {
    E0456, // plugin `..` is not available for triple `..`
    E0457, // plugin `..` only found in rlib format, but must be available...
    E0514, // metadata version mismatch
    E0460, // found possibly newer version of crate `..`
    E0461, // couldn't find crate `..` with expected target triple ..
    E0462, // found staticlib `..` instead of rlib or dylib
    E0464, // multiple matching crates for `..`
    E0465, // multiple .. candidates for `..` found
    E0519, // local crate and dependency have same (crate-name, disambiguator)
    E0523, // two dependencies have same (crate-name, disambiguator) but different SVH
}
