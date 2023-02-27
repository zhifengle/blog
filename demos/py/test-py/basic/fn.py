# define a function that squares a number
def square(x):
    return x**2


# define a function that checks if a number is odd
def is_odd(x):
    return x % 2 == 1


# create a list of numbers
numbers = [1, 2, 3, 4, 5]

# use map() to apply square() to each element in numbers
squared = map(square, numbers)

# use filter() to filter out even numbers from squared
odd_squared = filter(is_odd, squared)

# use sorted() to sort odd_squared in descending order
sorted_odd_squared = sorted(odd_squared, reverse=True)

# convert the sorted object into a list
sorted_odd_squared = list(sorted_odd_squared)

# print the sorted odd squared list
print(sorted_odd_squared)
