<script context="module">
  let cube;
  let camera;
  let controls;
  let renderer;

  const positionCamera = (x, y, z, up) => {
    camera.position.set(x, y, z);
    camera.up.set(...up);
    controls.target.set(0, 0, 0);
    controls.update();
  };

  export const move_cube = (x, y, z) => cube.position.set(x, y, z);

  export const point_xy_plane = () => positionCamera(0, 0, 10, [0, 1, 0]);

  export const point_xz_plane = () => positionCamera(0, 10, 0, [0, 0, 1]);

  export const point_yz_plane = () => positionCamera(10, 0, 0, [1, 0, 0]);
</script>

<script>
  import { onMount } from "svelte";
  import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
  import { throttle } from "lodash-es";
  import * as THREE from "three";

  let scene, renderer, gridHelper, controls, axesHelper;

  const resizeUpdateInterval = 500;

  const onResize = throttle(
    () => {
      const width = window.innerWidth - 100;
      const height = window.innerHeight - 100;
      camera.aspect = width / height;
      camera.updateProjectionMatrix();
      renderer.setSize(width, height);
    },
    resizeUpdateInterval,
    { trailing: true }
  );

  const init = () => {
    // Camera setup
    camera = new THREE.OrthographicCamera(
      window.innerWidth / -100,
      window.innerWidth / 100,
      window.innerHeight / 100,
      window.innerHeight / -100,
      -1000,
      1000
    );

    // Scene setup
    scene = new THREE.Scene();

    // Grid helper
    gridHelper = new THREE.GridHelper(20, 20, 0x888888, 0x444444);
    scene.add(gridHelper);

    // Cube
    const geometry = new THREE.BoxGeometry();
    const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
    cube = new THREE.Mesh(geometry, material);
    scene.add(cube);

    // Axes helper
    axesHelper = new THREE.AxesHelper(5);
    axesHelper.lineWidth = 125;
    axesHelper.position.y += 0.001;
    scene.add(axesHelper);

    // Renderer
    renderer = new THREE.WebGLRenderer({ antialias: true, canvas: el });
    renderer.setSize(window.innerWidth - 100, window.innerHeight - 100);

    // OrbitControls
    controls = new OrbitControls(camera, renderer.domElement);
    controls.update();

    // Event listeners
    window.addEventListener("resize", onResize);
  };

  const animate = () => {
    renderer.render(scene, camera);
    controls.update();
    requestAnimationFrame(animate);
  };

  export const createScene = (el) => {
    init();
    animate();
  };

  let el;
  onMount(() => {
    el = createScene();
  });
</script>

<canvas bind:this={el} />
