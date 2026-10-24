"""Reverse Integer with the boundary checked before the push, O(number of digits).

Coding Literacy - overflow is a language decision.

The loop is three steps in every language: pop the last digit (x % 10), drop it
(x // 10), push it onto the result (rev * 10 + d). What happens on the push that
crosses 2^31 is not in the algorithm. Python's int has no width, so that push
simply succeeds and the 32-bit range exists only if you write the check.

reverse() writes it before the push, comparing rev against limit // 10 and the
digit against limit % 10, so rev itself never leaves the 32-bit range. That
makes this Python a faithful spec of what the Rust and Scala siblings must do.

reverse_via_string() is the short version most people write first. It is
correct, and it builds the forbidden value: for 1534236469 the int() call
produces 9646324351, which needs more than 32 bits, and the range check runs
after the fact.

Python's // and % floor toward negative infinity (-123 // 10 == -13,
-123 % 10 == 7, -1 // 10 == -1), so the loop runs on abs(x) and reapplies the
sign at the end. abs(-2**31) is 2**31, and so is limit for a negative x: that
one value is where this leans on Python's width (it fits an unsigned 32-bit
magnitude, and rev never exceeds 2147483641); the fixed-width siblings keep
the sign in the digits instead.
"""

INT_MAX = 2**31 - 1  # 2147483647


def reverse_via_string(x: int) -> int:
    sign = -1 if x < 0 else 1
    rev = sign * int(str(abs(x))[::-1])  # for 1534236469 this builds 9646324351
    return rev if -(2**31) <= rev <= INT_MAX else 0  # checked after it was built


def reverse(x: int) -> int:
    sign = -1 if x < 0 else 1
    limit = INT_MAX + 1 if x < 0 else INT_MAX  # 2147483648 = abs(INT_MIN) for a negative x
    bound, last = limit // 10, limit % 10  # 214748364, and 7 or 8
    x = abs(x)  # floor // and % on a negative x would pop different digits
    rev = 0
    while x:
        d = x % 10
        x //= 10
        if rev > bound or (rev == bound and d > last):  # refuse the push before making it
            return 0
        rev = rev * 10 + d
    return sign * rev


if __name__ == "__main__":
    # A tiny demo so `uv run main.py` does something visible.
    for sample in (123, -123, 120, 1534236469, -2147483648):
        print(f"reverse({sample}) = {reverse(sample)}")
