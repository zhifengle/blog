import React, { useEffect, useState } from 'react';
import './App.css';
import init, { add } from 'wasm-lib';

function App() {
  const [count, setCount] = useState(0);
  useEffect(() => {
    init().then(() => {
      setCount(add(1, 1));
    });
  }, []);
  return (
    <div className="App">
      <p>{count}</p>
    </div>
  );
}

export default App;
