# Unsoundness Checker

A tool for reporting possible typing unsoundness in Python

There are a few aims of this repo:
- To document unsoundness in the Python type system.
- To provide a tool for detecting unsoundness in Python code.
- To also discuss why some type checkers don't detect certain unsoundness.

What is unsoundness (and soundness)?

Type systems are considered sound when they can guarantee the absence of type errors at runtime.
However, in Python, `TypeError`s can occur for various reasons, and the type system cannot prevent all of these cases.

Most type checkers do a very good job of preventing this unsoundness, but there are still several cases where unsoundness
can occur and users can run into `TypeError`s.
