/*!
Combined crate for all Atelier sub-crates incorporated as features. Atelier is a Rust native
library, and tools, for the AWS [Smithy](https://github.com/awslabs/smithy) Interface Definition
Language.

The aim of this crate is to provide a single client interface over a set of crates that provide
different Atelier capabilities. The following table shows the mapping from individual crate to the
combined module path in this library. The column _Default_ indicates those that are included in the
default feature, although the core will be included regardless of any feature selection.

| Feature name | Default | Individual crate                                    | Target module path                | Purpose                                               |
|--------------|---------|-----------------------------------------------------|-----------------------------------|-------------------------------------------------------|
| N/A          | **Yes** | [atelier_core](https://docs.rs/atelier_core)        | `atelier_lib::core`               | Core models only.                                     |
| "json"       | Yes     | [atelier_json](https://docs.rs/atelier_json)        | `atelier_lib::format::json`       | Reading and Writing JSON AST representation.          |
| "openapi"    | No      | [atelier_openapi](https://docs.rs/atelier_openapi)  | `atelier_lib::format::openapi`    | Reading and Writing OpenAPI representations.          |
| "smithy"     | Yes     | [atelier_smithy](https://docs.rs/atelier_smithy)    | `atelier_lib::format::smithy`     | Reading and Writing the Smithy native representation. |
| "uml"        | No      | In this crate                                       | `atelier_lib::format::plant_uml`  | Writing models in a diagram form. |

*/

#![warn(
    // ---------- Stylistic
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    // ---------- Public
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
)]

pub use atelier_core as core;

pub use atelier_select as select;

pub mod action;

pub mod assembler;

#[cfg(any(
    feature = "json",
    feature = "openapi",
    feature = "smithy",
    feature = "uml"
))]
pub mod format;
