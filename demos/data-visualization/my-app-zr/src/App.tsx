import React, { useEffect, useRef, useState } from 'react';
import Editor from './editor';
import './App.css';

function App() {
  const canvasRef = useRef(null);
  useEffect(() => {
    if (canvasRef && canvasRef.current) {
      new Editor(canvasRef.current);
    }
  }, [canvasRef]);
  return (
    <div className="App">
      <canvas width={600} height={600} ref={canvasRef}></canvas>
      {/* <div style={{ width: '100vw', height: '100vh' }} ref={canvasRef}></div> */}
    </div>
  );
}

export default App;
