export function quickSort<T>(arr: T[], lo: number, hi: number) {
  if (hi < lo) return
  const p = partition(arr, lo, hi)
  quickSort(arr, lo, p - 1)
  quickSort(arr, p + 1, hi)
}

function partition<T>(arr: T[], lo: number, hi: number): number {
  const pivot = arr[hi]
  let i = lo
  for (let j = lo; j <= hi; j++) {
    if (arr[j] < pivot) {
      const temp = arr[i]
      arr[i] = arr[j]
      arr[j] = temp
      i++
    }
  }
  const temp = arr[i]
  arr[i] = arr[hi]
  arr[hi] = temp
  return i
}
