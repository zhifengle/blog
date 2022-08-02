import React, { useContext, useState } from 'react';
import './styles.css';

const LogContext = React.createContext();

// 当 LogProvider 中的 addLog 被子组件调用，LogProvider重渲染。
// Provider 的 value 发生改变，
// value 包含了 logs 和 setLogs 属性
// useContext(LogContext)的子组件重新渲染。

function LogProvider({ children }) {
  const [logs, setLogs] = useState([]);
  const addLog = (log) => setLogs((prevLogs) => [...prevLogs, log]);
  return (
    <LogContext.Provider value={{ logs, addLog }}>
      {children}
    </LogContext.Provider>
  );
}

function Logger1() {
  const { addLog } = useContext(LogContext);
  console.log('Logger1 render');
  return (
    <>
      <p>一个能发日志的组件1</p>
      <button onClick={() => addLog('logger1')}>发日志</button>
    </>
  );
}

function Logger2() {
  const { addLog } = useContext(LogContext);
  console.log('Logger2 render');
  return (
    <>
      <p>一个能发日志的组件2</p>
      <button onClick={() => addLog('logger2')}>发日志</button>
    </>
  );
}

function LogsPanel() {
  const { logs } = useContext(LogContext);
  console.log('Logger1 render');
  return logs.map((log, index) => <p key={index}>{log}</p>);
}

export default function App() {
  return (
    <LogProvider>
      <div className="app">
        <section>
          <Logger1 />
          <Logger2 />
        </section>
        <section>
          <LogsPanel />
        </section>
      </div>
    </LogProvider>
  );
}
