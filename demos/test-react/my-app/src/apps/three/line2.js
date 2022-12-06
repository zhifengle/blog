import * as THREE from 'three'
import React, { useLayoutEffect, useRef } from 'react'
import { Canvas } from '@react-three/fiber'

function Line({ start, end }) {
  const ref = useRef()
  useLayoutEffect(() => {
    ref.current.geometry.setFromPoints([start, end].map((point) => new THREE.Vector3(...point)))
  }, [start, end])
  return (
    <line ref={ref}>
      <bufferGeometry />
      <lineBasicMaterial color="hotpink" />
    </line>
  )
}

export default function App() {
  return (
    <Canvas>
      {/* <ambientLight intensity={0.5} /> */}
      {/* <spotLight position={[10, 10, 10]} angle={0.15} penumbra={1} /> */}
      {/* <pointLight position={[-10, -10, -10]} /> */}
      <Line start={[0, 0, 0]} end={[1, 0, 0]} />
      <Line start={[1, 0, 0]} end={[1, 1, 0]} />
    </Canvas>
  )
}