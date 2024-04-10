# xensieve-py


<a href="https://github.com/flexatone/xensieve-py/actions/workflows/ci.yml">
    <img style="display: inline!important" src="https://img.shields.io/github/actions/workflow/status/flexatone/xensieve-py/ci.yml?branch=default&label=CI&logo=Github"></img>
</a>

<!-- <a href="https://codecov.io/gh/flexatone/xensieve-py">
    <img style="display: inline!important" src="https://codecov.io/gh/flexatone/xensieve-py/branch/default/graph/badge.svg"></img>
</a> -->


An implementation of the Xenakis Sieve, providing a Sieve from a string expression that filters integer sequences into iterators of integers, Boolean states, or interval widths. Sieves are built from Residuals, defined as a modulus (M) and a shift (S), notated `M@S`. Sieve string expressions, and Sieve structs, support complementation, intersection, symmetric difference, and union operations on Residuals with operators `!`, `&`, `^` and `|`, respectively.

The Xenakis Sieve is a tool for generating discrete interval patterns. Such patterns have boundless applications in creative domains: the Xenakis Sieve can be used to generate scales or multi-octave pitch sequences, rhythms and polyrhythms, and used to control countless other aspects of pictorial or architectural design.

This Python implementation wraps a Rust implementation and follows the Python implementation in Ariza (2005), with significant performance and interface enhancements: https://direct.mit.edu/comj/article/29/2/40/93957

Code: https://github.com/flexatone/xensieve-py



# Rust Implementation

The Python implementation is built with PyO3, which wraps the Rust core library `xensieve`.

Code: https://github.com/flexatone/xensieve-rs

Docs: https://docs.rs/xensieve

Crate: https://crates.io/crates/xensieve



# Strategies for Creating Sieves

First, we can examine the output of Sieves built from a single Residual. As shown above, a Residual is defined as a modulus (M) and a shift (S), notated `M@S`. In the diagram below, three Residuals are shown: `5@0`, `4@2`, and `30@10`. As can be seen, for every M units, a value is articulated at the shift S. The final example shows an application of the unary inversion operator `!30@10`.

![Residual diagram](https://raw.githubusercontent.com/flexatone/xensieve-sandbox/default/images/residual-a.svg)

Complex Sieves combine Residuals with logical operators such as complementation, intersection, symmetric difference, and union. In the example below, Residuals `5@0` and `4@2` are combined by union with the expression `5@0|4@2`. Combining many Residuals by union is a practical approach to building sequences. The final example, `(5@0|4@2)&!30@10`, shows "removing" selected values from these unioned components by intersecting them with an inverted Residual (`!30@10`)

![Sieve diagram](https://raw.githubusercontent.com/flexatone/xensieve-sandbox/default/images/sieve-a.svg)

While all Sieves are, by definition, periodic, combinations of Residuals can result in sequences with great local complexity and inner patterning.



# The `xensieve.Sieve` Inteface

The Sieves shown above can be created with `xensieve.Sieve` and used to produce iterators of integers, Boolean states, or interval widths. The `Sieve` constructor accepts arbitrarily complex Sieve expressions.

```python
>>> from xensieve import Sieve

>>> s1 = Sieve("5@0")
>>> s2 = Sieve("30@10")
>>> s3 = Sieve("(5@0|4@2)&!30@10")
```

The `iter_value()` method takes a range (defined by start and stop integers) that can be used to "drive" the Sieve. The iterator yields the subset of integers contained within the Sieve.

```python

>>> s1.iter_value(0, 50)
<builtins.IterValue object at 0x7f538abdb9c0>
>>> list(s1.iter_value(0, 50))
[0, 5, 10, 15, 20, 25, 30, 35, 40, 45]
>>> list(s2.iter_value(0, 50))
[10, 40]
>>> list(s3.iter_value(0, 50))
[0, 2, 5, 6, 14, 15, 18, 20, 22, 25, 26, 30, 34, 35, 38, 42, 45, 46]
```

The `xensieve.Sieve` features two alternative iterators to permit using Sieves in different contexts. The `iter_state()` iterator returns, for each provided integer, the resulting Boolean state.

```python
>>> list(s1.iter_state(0, 10))
[True, False, False, False, False, True, False, False, False, False]
>>> list(s3.iter_state(0, 10))
[True, False, True, False, False, True, True, False, False, False]
```

The `iter_interval()` iterator returns, for sequential pairs of provided integers that are within the Sieve, the resulting interval.

```python
>>> list(s2.iter_interval(0, 50))
[30]
>>> list(s3.iter_interval(0, 50))
[2, 3, 1, 8, 1, 3, 2, 2, 3, 1, 4, 4, 1, 3, 4, 3, 1]
```

The `xensieve.Sieve` instance implements `__contains__()` such that `in` can be used to test if arbitrary integers are contained within the Sieve:

```python
>>> 5 in s1
True
>>> 6 in s1
False
>>> 10 in s3
False
>>> 30 in s3
True
```

The `xensieve.Sieve` instance supports the same operators permitted in Sieve expressions, such that instances can be combined to build complex Sieves.

```python
>>> s4 = (Sieve("5@0") | Sieve("4@2")) & ~Sieve("30@10")
>>> s4
Sieve{5@0|4@2&!(30@10)}
>>> list(s4.iter_value(0, 100)) == list(s3.iter_value(0, 100))
True
```

# What is New in `xensieve`

## 0.8.0

Updated Rust back-end to 0.8.0.