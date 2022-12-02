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

    const scene = new THREE.Scene();
    // 正交相机。 参数 ??
    const camera = new THREE.OrthographicCamera(-s * k, s * k, s, -s, 1, 1000);
    const geometry = new THREE.BoxGeometry(16, 16, 16);
    const material = new THREE.MeshNormalMaterial();
    const cube = new THREE.Mesh(geometry, material);
    // 移动位置
    cube.translateX(-14)

    camera.position.set(-1.66, 2.21, 18.1);
    camera.lookAt(scene.position)
    scene.add(cube);

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
    renderScene()

    return () => {
      window.removeEventListener('resize', handleResize);
      mount.current.removeChild(renderer.domElement);
      // 清理工作省略。。 ref cube-demo.js
    };
  }, []);

  return <div className="vis" ref={mount} />;
};

export default Vis;
