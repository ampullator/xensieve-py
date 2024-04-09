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

#-------------------------------------------------------------------------------

def test_iter_value_a() -> None:
    s1 = Sieve("6@2|7@0")
    post = list(s1.iter_value(0, 100))
    assert post == [0, 2, 7, 8, 14, 20, 21, 26, 28, 32, 35, 38, 42, 44, 49, 50, 56, 62, 63, 68, 70, 74, 77, 80, 84, 86, 91, 92, 98]

def test_iter_interval_a() -> None:
    s1 = Sieve("7@2|9@1")
    post = list(s1.iter_interval(0, 100))
    assert post == [1, 7, 1, 6, 3, 4, 5, 2, 7, 7, 2, 5, 4, 3, 6, 1, 7, 1, 6, 3, 4, 5, 2]

def test_iter_state_a() -> None:
    s1 = Sieve("4@2|5@1")
    post = list(s1.iter_state(0, 20))
    assert post == [False, True, True, False, False, False, True, False, False, False, True, True, False, False, True, False, True, False, True, False]
