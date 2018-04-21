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
//! To enable this on docs.rs, you should be able to just add the following to your Cargo.toml:
//!
//! ```toml
//! [package.metadata.docs.rs]
//! rustdoc-args = [
//!     "--html-in-header",
//!     ".cargo/registry/src/github.com-1ecc6299db9ec823/katex-doc-0.1.0/katex.html",
//! ]
//! ```
//!
//! If that doesn't work for other crates, download [katex.html] from this repository,
//! then put it at the root of your repository, include it in your publish,
//! then replace the reference to `katex-doc-ver.si.on` with your crate.
//!
//! [katex.html]: <https://github.com/CAD97/katex-doc/blob/master/katex.html>
//!
