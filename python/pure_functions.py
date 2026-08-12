"""
mm2-stdlib — single file for Python pure functions.

How to add a function:
  1. Write a normal Python function.
  2. Put @pure above it (or @pure(name="other_name")).
  3. That is all. Importing this module registers it with MORK.

Example:
    @pure
    def add(x: float, y: float) -> float:
        return x + y
"""

from __future__ import annotations

 # Bridge to the embedded Rust/PyO3 module registered by mm2-stdlib.

try:
    from mork_python import register_py_fn
except ImportError as e:
    raise ImportError(
        "mork_python is not available.\n"
        "Build mm2-stdlib with features = [\"python\"] and make sure "
        "the extension module can be imported."
    ) from e




def register(name: str, fn) -> None:
    """Register a callable under a pure-function name visible to MORK."""
    if not callable(fn):
        raise TypeError(f"expected callable, got {type(fn)!r}")
    register_py_fn(name, fn)


def pure(fn=None, *, name: str | None = None):
    """
    Decorator that registers the function as a MORK pure function.

    Usage:
        @pure
        def add(x, y):
            return x + y

        @pure(name="my_add")
        def add(x, y):
            return x + y
    """
    def decorator(f):
        register(name or f.__name__, f)
        return f

    if fn is not None:
        return decorator(fn)
    return decorator

# Add your own functions below this line

@pure
def add(x: float, y: float) -> float:
    """(add x y) → x + y"""
    return float(x) + float(y)


@pure(name="py_add")
def py_add(x: float, y: float) -> float:
    """(py_add x y) → x + y, used as a dedicated Python smoke test."""
    return float(x) + float(y)


@pure
def clamp(x: int, lo: int, hi: int) -> int:
    """(clamp x lo hi) → value of x limited to [lo, hi]"""
    x, lo, hi = int(x), int(lo), int(hi)
    return max(lo, min(x, hi))


