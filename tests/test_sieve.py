from xensieve import Sieve


def test_sieve_repr_a() -> None:
    s = Sieve("3@2")
    assert str(s) == "Sieve{3@2}"

def test_sieve_contains_a() -> None:
    s1 = Sieve("5@0")
    assert 5 in s1
    assert 10 in s1
    assert 15 in s1

def test_sieve_invert_a() -> None:
    s1 = Sieve("5@0")
    s2 = ~s1
    assert str(s2) == "Sieve{!(5@0)}"
    assert 4 in s2

def test_sieve_xor_a() -> None:
    s1 = Sieve("3@2")
    s2 = Sieve("5@1")
    s3 = s1 ^ s2
    assert str(s3) == "Sieve{3@2^5@1}"

def test_sieve_or_a() -> None:
    s1 = Sieve("3@2")
    s2 = Sieve("5@1")
    s3 = s1 | s2
    assert str(s3) == "Sieve{3@2|5@1}"

def test_sieve_and_a() -> None:
    s1 = Sieve("3@2")
    s2 = Sieve("5@1")
    s3 = s1 & s2
    assert str(s3) == "Sieve{3@2&5@1}"




