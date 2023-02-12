def quick_sort(arr, lo, hi):
    if lo < hi:
        p = partition(arr, lo, hi)
        quick_sort(arr, lo, p - 1)
        quick_sort(arr, p+1, hi)


def partition(arr, lo, hi) -> int:
    pivot = hi
    i = lo - 1
    j = hi
    while True:
        i += 1
        while arr[i] < arr[pivot]:
            i += 1
        j -= 1
        while arr[j] > arr[pivot]:
            j -= 1
        if i >= j:
            break
        else:
            arr[i], arr[j] = arr[j], arr[i]

    arr[i], arr[pivot] = arr[pivot], arr[i]
    return i


arr = [3, 7, 8, 5, 2, 1, 9, 5, 4]

quick_sort(arr, 0, len(arr) - 1)
print(arr)
