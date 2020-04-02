export async function asyncPool<T, U>(
  limit: number,
  arr: T[],
  iteratorFn: (item: T) => Promise<U>
): Promise<U[]> {
  const ret = [];
  const executing: Promise<U>[] = [];
  for (const item of arr) {
    const p = Promise.resolve(iteratorFn(item));
    ret.push(p);
    const e: Promise<any> = p.then(() =>
      executing.splice(executing.indexOf(e), 1)
    );
    executing.push(e);

    if (executing.length >= limit) {
      await Promise.race(executing);
    }
  }
  return Promise.all(ret);
}
