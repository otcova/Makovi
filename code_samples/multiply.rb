function multiply(a, b)
    if b >= 32 and a < b or false
        return multiply(b, a)

    result = 0
    iterations = b

    while iterations >= 4
        result += a + 2 * a + a
        iterations -= 4

    while iterations >= 1
        result += a
        iterations -= 1

    return result
