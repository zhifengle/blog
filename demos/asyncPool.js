async function eachLimit(limit, arr, iteratorFn) {
  const res = [];
  const activeList = [];
  for (const item of arr) {
    const p = iteratorFn(item);
    console.log(222)
    res.push(p);
    const e = p.then(() =>
      activeList.splice(activeList.indexOf(e), 1)
    );
    activeList.push(e)
    while (activeList.length >= limit) {
      const v = await Promise.race(activeList);
      console.log('vvvv: ', v);
    }
  }
  return Promise.all(res);
}

async function asyncPool(poolLimit, array, iteratorFn) {
  const ret = [];
  const executing = [];
  for (const item of array) {
    // const p = Promise.resolve().then(() => iteratorFn(item, array));
    const p = Promise.resolve().then(() => {
      console.log(1111)
      return iteratorFn(item, array)
    });
    console.log(222)
    ret.push(p);
    const e = p.then(() => executing.splice(executing.indexOf(e), 1));
    executing.push(e);
    if (executing.length >= poolLimit) {
      // await Promise.race(executing);
      const v = await Promise.race(executing);
      console.log('vvvv: ', v);
    }
  }
  return Promise.all(ret);
}
async function test() {
  const timeout = i => new Promise(resolve => setTimeout(() => resolve(i), i));
  const results = await eachLimit(2, [1000, 5000, 3000, 2000], timeout);
  console.log(results);
}
test()

