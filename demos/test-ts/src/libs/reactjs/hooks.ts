import { Func } from '../../types';

let memorizedStated: any[] = [];
let cursor: number = 0;

export function useState<S>(initialVale: S | (() => S)): [S, Func] {
  memorizedStated[cursor] = memorizedStated[cursor] || initialVale;

  const currentCursor = cursor;
  function setState(newState: S | (() => S)) {
    memorizedStated[currentCursor] = newState;
    // render()
  }

  return [memorizedStated[cursor++], setState];
}

export function useEffect<S>(cb: Func, depArray: S[]) {
  const hasNoDeps = !depArray;
  const deps = memorizedStated[cursor];
  const hasChangedDeps = deps
    ? !depArray.every((el: S, i) => el === deps[i])
    : true;
  if (hasNoDeps || hasChangedDeps) {
    cb();
    memorizedStated[cursor] = depArray;
  }
  cursor++;
}
