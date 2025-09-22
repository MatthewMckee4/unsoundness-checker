# Unsoundness Checker

There are a few aims of this repository:
- To document unsoundness in the Python type system.
- To discuss why some type checkers don't detect certain unsoundness.
- To provide a tool for detecting (some) unsoundness in Python code.
- To talk about the limitations of the tool.

What is unsoundness (and soundness)?

Type systems are considered sound when they can guarantee the absence of type errors at runtime.
However, in Python, `TypeError`s can occur for various reasons, and the type system cannot prevent all of these cases.

Most type checkers do a very good job of preventing this unsoundness, but there are still several cases where unsoundness
can occur and users can run into `TypeError`s.

## Contributing

This project is my 4th year individual project at the University of Glasgow.
This repository is only public to show my progress.
Please do not attempt to contribute to this.
