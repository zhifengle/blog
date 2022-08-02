import React from 'react';

const LogStateContext = React.createContext();
const LogDispatcherContext = React.createContext();

export function useLogState() {
  const context = React.useContext(LogStateContext);
  if (context === undefined) {
    throw new Error('useLogState must be used within a LogStateProvider');
  }
  return context;
}

export function useLogDispatcher() {
  const context = React.useContext(LogDispatcherContext);
  if (context === undefined) {
    throw new Error(
      'useLogDispatcher must be used within a LogDispatcherContext'
    );
  }
  return context;
}

export function useLogs() {
  return [useLogState(), useLogDispatcher()];
}

function composeProviders(...providers) {
  return ({ children }) =>
    providers.reduce((prev, Provider) => <Provider>{prev}</Provider>, children);
}
