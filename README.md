# Unsoundness Checker

We have a few aims with the tool and the site:
- To formally document unsoundness in the Python type system.
- To discuss why type checkers don't detect certain unsoundness.
- To provide a tool for detecting (some) unsoundness in Python code.
- To talk about the limitations of the tool.

What is unsoundness (and soundness)?

Type systems are considered sound when they can guarantee the absence of type errors at runtime.
However, in Python, `TypeError`s can occur for various reasons, and the type system cannot prevent all of these cases.

Most type checkers do a very good job of preventing most unsoundness, but there are still several cases where unsoundness
can occur and users can run into `TypeError`s.

## References

The Python type system is known to be partially unsound. And there have been some discussions about different examples of unsoundness.

- [Discussion of examples of unsoundness](https://discuss.python.org/t/collecting-examples-of-unsoundness/97568)
- [Collection of examples of unsoundness](https://github.com/JelleZijlstra/unsoundness)

It is important to note these resources. But they are trying to accomplish something different.

Examples are useful to show how unsoundness can occur in code,
but what we are trying to do here is categorise the different types of unsoundness.

## Acknowledgments

The core of this codebase is built on top of `ty` and `ruff`, the code from which is available at [github.com/astral-sh/ruff](https://github.com/astral-sh/ruff).

We use the `ruff` python parser.

We use `ty` to infer the types of expressions and statements.

We also take the structure of the rules (in [`rule.rs`](crates/unsoundness_checker/src/rule.rs)) from `ty`.

## Contributing

This project is my 4th year individual project at the University of Glasgow.
This repository is only public to show my progress.
Please do not attempt to contribute to this.
