export function groupBy<T>(xs: T[], key: any): {} {
  return xs.reduce(function(rv: {}, obj: T) {
    // @ts-ignore
    (rv[obj[key]] = rv[obj[key]] || []).push(obj);
    return rv;
  }, {});
}
