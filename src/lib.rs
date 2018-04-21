//!
//! Utilizing the html injection feature of rustdoc,
//! we can enable the use of $`\KaTeX`$ in documentation!
//!
//! For inline math, use ``$`1+1`$``, which renders as $`1+1`$.
//!
//! For display math, use:
//!
//! ````markdown
//! ```math
//! f(x) = \int_{-\infty}^\infty
//!   \hat f(\xi)\,e^{2 \pi i \xi x}
//!   \,d\xi
//! ```
//! ````
//!
//! (example taken from <https://khan.github.io/KaTeX/>), which renders as:
//!
//! ```math
//! f(x) = \int_{-\infty}^\infty
//!   \hat f(\xi)\,e^{2 \pi i \xi x}
//!   \,d\xi
//! ```
//!
//! To enable this on docs.rs, you need to add the following to your Cargo.toml:
//!
//! ```toml
//! [package.metadata.docs.rs]
//! rustdoc-args = [
//!     "--html-in-header",
//!     ".cargo/registry/src/github.com-???/katex-doc-0.0.0/katex.html",
//! ]
//! ```
