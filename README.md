# XaaS: Use KaTeX on docs.rs!

Thanks to [pwnies](https://docs.rs/pwnies/0.0.14/pwnies/) for bringing this method to where I noticed it.
Technically relies on [this bug](https://github.com/onur/docs.rs/issues/167) in docs.rs.
[See the result on docs.rs!](https://docs.rs/katex-doc/)

Add the following to your `Cargo.toml`:

```toml
[package.metadata.docs.rs]
rustdoc-args = [
    "--html-in-header",
    ".cargo/registry/src/github.com-1ecc6299db9ec823/katex-doc-0.1.0/katex.html",
]
```

And use ``$`inline\ math`$`` or

````markdown
```math
display-style\ math
```
````

If it so happens that that path does not always work (because this crate's build isn't in docs.rs cache),
download [katex.html](https://github.com/CAD97/katex-doc/blob/master/katex.html) from this repository,
then put it at the root of your repository, include it in your publish,
and replace the reference to `katex-doc-ver.si.on` with your crate.

For local documentation builds, acquire `katex.html` and use `cargo rustdoc --open -- --html-in-header katex.html`. If
you know how to get `cargo doc` to set flags for rustdoc for just one crate when it's in the dependency graph, so that
KaTeX-enabled docs can render properly when building docs as a dependency, please open an issue so we can document it.
