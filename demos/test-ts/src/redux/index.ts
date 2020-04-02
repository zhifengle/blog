import { Func } from "../types";

interface IState {
  [prop: string]: any;
}

interface IStore {
  getState: Func;
  subscribe: Func;
  dispatch: Func;
}

function createStore(
  reducer: Func,
  preloadedState: IState,
  enhancer?: Func
): IStore {
  if (enhancer) {
    return enhancer(createStore)(reducer, preloadedState);
  }

  let currentReducer = reducer;
  let currentState = preloadedState;
  let currentListeners: any[] = [];
  let nextListeners = currentListeners;

  function getState(): IState {
    return {};
  }

  function subscribe() {}

  function dispatch() {}

  return {
    dispatch,
    subscribe,
    getState
    // replaceReducer,
    // observable
  };
}
