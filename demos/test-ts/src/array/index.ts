export function flatten<T>(a: T | T[]): T[] {
  return Array.isArray(a) ? [].concat.apply([], a.map(flatten)) : a
}

// TODO type
export function deepCopy<T>(original: any): any {
  if (Array.isArray(original)) {
    const copy = []
    for (const [idx, val] of original.entries()) {
      copy[idx] = deepCopy(val)
    }
    return copy
  } else if (original && typeof original === 'object') {
    const copy: any = {}
    for (const [key, val] of original.entries()) {
      copy[key] = deepCopy(val)
    }
    return copy
  }
  return original
}
