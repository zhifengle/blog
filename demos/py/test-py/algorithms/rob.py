def rob(nums):
    a, b = 0, 0
    for i in range(0, len(nums)):
        temp = b
        b = max(nums[i]+a, b)
        a = temp
    return b
