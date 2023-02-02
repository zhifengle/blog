function flatten(arr) {
  return arr.reduce(function (flat, res) {
    return flat.concat(Array.isArray(res) ? flatten(res) : res);
  }, []);
}

function flatten2(arr) {
  while (arr.some((item) => Array.isArray(item))) {
    arr = [].concat(...arr);
  }
  return arr;
}

Function.prototype.bind2 = function (ctx, ...args) {
  // typeof this; otherwise throw
  const self = this;
  const noop = function () {};
  const _bind = function (...innerArgs) {
    return self.apply(this instanceof noop ? this : ctx, [
      ...args,
      ...innerArgs,
    ]);
  };
  noop.prototype = this.prototype;
  _bind.prototype = new noop();
  return _bind;
};

function New(ctor, ...args) {
  const obj = new Object();
  obj.__proto__ = ctor.prototype;
  const ret = ctor.apply(obj, args);
  return typeof ret === "object" ? ret : obj;
}

function throttle(func, wait) {
  let timer;
  let ctx;
  let previous;

  const later = function () {
    previous = +new Date();
    timer = null;
    func.apply(ctx, args);
  };

  const _throttle = function (...args) {
    const now = +new Date();
    const remaining = wait - (now - previous);
    ctx = this;
    if (remaining <= 0 || remaining > wait) {
      if (timer) {
        clearTimeout(timer);
        timer = null;
      }
      previous = now;
      func.apply(ctx, args);
    } else if (!timer) {
      timer = setTimeout(later, remaining);
    }
  };
  return _throttle;
}

function advantage(arr1, arr2) {
  arr1.sort((a, b) => a - b);
  arr2.sort((a, b) => a - b);
  let res = 0;
  let ab = 0;
  let ae = arr1.length - 1;
  let bb = 0;
  let be = arr2.length - 1;
  while (ae >= ab && be >= bb) {
    // 上对上
    if (arr1[ae--] > arr2[be--]) {
      res++;
      // 下对上
    } else if (arr1[ae] < arr2[be]) {
      res--;
      ab++;
      be--;
    } else {
      // 下对下
      if (arr1[ab++] > arr2[bb++]) {
        res++;
      } else {
        if (arr1[ab++] < arr2[be--]) {
          res--;
        }
      }
    }
  }
  return res;
}

const coins = [0, 1, 5, 10, 25, 50];
function count2(amount, kinds) {
  if (amount < 0 || kinds === 0) return 0;
  return count2(amount - coins[kinds], kinds) + count2(amount, kinds - 1);
}

function quick(arr, lo, hi) {
  if (lo < hi) {
    const p = part(arr, lo, hi);
    quick(arr, lo, p);
    quick(arr, p + 1, hi);
  }
}

function part(arr, lo, hi) {
  const pivot = arr[(lo + hi) >> 1];
  let i = lo - 1;
  let j = hi + 1;
  while (true) {
    do {
      i++;
    } while (arr[i] < pivot);
    do {
      j--;
    } while (arr[j] > pivot);
    if (i >= j) return j;
    [arr[i], arr[j]] = [arr[j], arr[i]];
  }
}
// arr = [3, 7, 8, 5, 2, 1, 9, 5, 4]
// quick(arr, 0, arr.length - 1);
// console.log(arr);

