import { useEffect, useRef } from 'react';
import * as THREE from 'three';

const Vis = () => {
  const mount = useRef(null);

  useEffect(() => {
    let width = mount.current.clientWidth;
    let height = mount.current.clientHeight;
    const k = width / height;
    const s = 24;

    const renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setClearColor('white', 1);
    renderer.setSize(width, height);
    mount.current.appendChild(renderer.domElement);

    // ----------------------------

    // 正交相机。 参数 ??
    const camera = new THREE.PerspectiveCamera(45, width / height, 1, 500);
    camera.position.set(0, 0, 100);
    camera.lookAt(0, 0, 0);
    // ----------------
    const scene = new THREE.Scene();
    const material = new THREE.LineBasicMaterial({ color: 0x0000ff });
    const points = [];
    points.push(new THREE.Vector3(-10, 0, 0));
    points.push(new THREE.Vector3(0, 10, 0));
    points.push(new THREE.Vector3(10, 0, 0));
    // points.push(new THREE.Vector3(0, 0, -20));
    const geometry = new THREE.BufferGeometry().setFromPoints(points)
    const line = new THREE.Line(geometry, material)
    scene.add(line)

    const renderScene = () => {
      renderer.render(scene, camera);
    };

    const handleResize = () => {
      width = mount.current.clientWidth;
      height = mount.current.clientHeight;
      renderer.setSize(width, height);
      camera.aspect = width / height;
      camera.updateProjectionMatrix();
      renderScene();
    };

    window.addEventListener('resize', handleResize);
    renderScene();

    return () => {
      window.removeEventListener('resize', handleResize);
      mount.current.removeChild(renderer.domElement);
      // 清理工作省略。。 ref cube-demo.js
    };
  }, []);

  return <div className="vis" ref={mount} />;
};

export default Vis;
