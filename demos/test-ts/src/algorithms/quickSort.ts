export function quickSort<T>(arr: T[], lo: number, hi: number) {
  if (hi < lo) return;
  const p = partition(arr, lo, hi);
  quickSort(arr, lo, p - 1);
  quickSort(arr, p + 1, hi);
}

function partition<T>(arr: T[], lo: number, hi: number): number {
  const pivot = arr[hi];
  let i = lo;
  for (let j = lo; j <= hi; j++) {
    if (arr[j] < pivot) {
      const temp = arr[i];
      arr[i] = arr[j];
      arr[j] = temp;
      i++;
    }
  }
  const temp = arr[i];
  arr[i] = arr[hi];
  arr[hi] = temp;
  return i;
}

export function quickSortHoare(arr: number[]) {
  quickSortHelper(arr, 0, arr.length - 1);
}

function quickSortHelper(arr: number[], lo: number, hi: number) {
  if (lo < hi) {
    const p = partitionHoare(arr, lo, hi);
    quickSortHelper(arr, lo, p - 1);
    quickSortHelper(arr, p + 1, hi);
  }
}

function partitionHoare(arr: number[], lo: number, hi: number): number {
  const p = hi;
  let i = lo - 1;
  let j = hi;
  while (true) {
    i++;
    while (arr[i] !== undefined && arr[i] < arr[p]) {
      i++;
    }
    j--;
    while (j >= 0 && arr[j] > arr[p]) {
      j--;
    }
    if (i >= j) {
      break;
    } else {
      [arr[i], arr[j]] = [arr[j], arr[i]];
    }
  }
  [arr[i], arr[p]] = [arr[p], arr[i]];
  return i;
}

export function quickSortArr(arr: number[]): number[] {
  const len = arr.length;
  if (len <= 1) {
    return arr;
  }
  const pivot = arr[0];
  const greater = [];
  const lesser = [];
  for (let i = 1; i < len; i++) {
    if (arr[i] > pivot) {
      greater.push(arr[i]);
    } else {
      lesser.push(arr[i]);
    }
  }
  return [...quickSortArr(lesser), pivot, ...quickSortArr(greater)];
}
