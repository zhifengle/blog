import itertools


def fib():
    a = b = 1
    yield a
    yield b
    while True:
        a, b = b, a + b
        yield b


def fib2(n):  # n is the number of Fibonacci numbers to generate
    a = b = 1
    count = 0  # keep track of how many numbers are generated
    yield a
    yield b
    count += 2  # increment count by 2 after yielding a and b
    while count < n:  # loop until count reaches n
        a, b = b, a + b
        yield b
        count += 1  # increment count by 1 after yielding b


def fib3(limit):  # limit is the maximum value of Fibonacci numbers to generate
    a = b = 1
    yield a
    yield b
    while True:
        a, b = b, a + b
        if b > limit:  # check if b exceeds limit
            break  # stop generating numbers
        yield b

def chunk_generator(generator, chunk_size):
    """
    Chunk a generator into sub-generators of a given size.
    """
    while True:
        chunk = list(itertools.islice(generator, chunk_size))
        if not chunk:
            break
        yield chunk

if __name__ == '__main__':
    # top100 = list(itertools.islice(fib(), 100))
    # print(top100)
    my_generator = (x for x in range(21))
    for chunk in chunk_generator(my_generator, 5):
        print(chunk)

