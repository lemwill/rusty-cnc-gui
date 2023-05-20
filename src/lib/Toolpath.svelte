<script context="module">
  let cube;
  let camera;
  let controls;

  export function move_cube(x, y, z) {
    cube.position.set(x, y, z);
  }

  export function point_xy_plane() {
    console.log("point_xy_plane");
    if (camera.position.z < 10) camera.position.set(0, 0, 10);
    else camera.position.set(0, 0, camera.position.z);
    camera.position.set(0, 0, 10);
    //camera.up.set(0, 1, 0);
    //camera.lookAt(new THREE.Vector3(0, 0, 0));

    //camera.updateProjectionMatrix();

    //controls.target.set(new THREE.Vector3(0, 0, 0));
    //controls.update();
  }

  export function point_xz_plane() {
    if (camera.position.y < 10) camera.position.set(0, 10, 0);
    else camera.position.set(0, camera.position.y, 0);
    camera.up.set(0, 0, 1);
    controls.target.set(0, 0, 0);

    //controls.target.set(new THREE.Vector3(0, 0, 0));
    controls.update();
  }

  export function point_yz_plane() {
    if (camera.position.x < 10) camera.position.set(10, 0, 0);
    else camera.position.set(camera.position.x, 0, 0);
    camera.up.set(1, 0, 0);
    controls.target.set(0, 0, 0);

    //controls.target.set(new THREE.Vector3(0, 0, 0));
    console.log("test");
    controls.update();
  }
</script>

<script>
  import { onMount } from "svelte";
  import { TrackballControls } from "three/addons/controls/TrackballControls.js";
  import { OrbitControls } from "three/addons/controls/OrbitControls.js";

  import { throttle } from "lodash-es";

  import * as THREE from "three";

  let scene, renderer, gridHelper, controls, axesHelper;

  function setCanvasDimensions(canvas, width, height, set2dTransform = false) {
    const ratio = window.devicePixelRatio;
    canvas.width = width * ratio;
    canvas.height = height * ratio;
    canvas.style.width = `${width}px`;
    canvas.style.height = `${height}px`;
    if (set2dTransform) {
      canvas.getContext("2d").setTransform(ratio, 0, 0, ratio, 0, 0);
    }
  }

  const resizeUpdateInterval = 500;

  window.addEventListener(
    "resize",
    throttle(
      () => {
        const width = window.innerWidth - 100;
        const height = window.innerHeight - 100;
        camera.aspect = width / height;
        camera.updateProjectionMatrix();
        renderer.setSize(width, height);
        //setCanvasDimensions(renderer.domElement, width, height);
      },
      resizeUpdateInterval,
      { trailing: true }
    )
  );
  function init() {
    // Create a camera
    /*
    camera = new THREE.PerspectiveCamera(
      75,
      window.innerWidth / window.innerHeight,
      0.1,
      1000
    );*/

    camera = new THREE.OrthographicCamera(
      window.innerWidth / -100,
      window.innerWidth / 100,
      window.innerHeight / 100,
      window.innerHeight / -100,
      -1000,
      1000
    );

    //camera.position.z = 10;
    //camera.lookAt(new THREE.Vector3(0, 0, 0));

    // Create a scene
    scene = new THREE.Scene();

    gridHelper = new THREE.GridHelper(20, 20, 0x888888, 0x444444);
    //gridHelper = new THREE.GridHelper(1000, 10, 0x888888, 0x444444);

    scene.add(gridHelper);

    //gridHelper = new THREE.InfiniteGridHelper(10, 100);
    //scene.add(gridHelper);

    // Create a cube
    const geometry = new THREE.BoxGeometry();
    const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
    cube = new THREE.Mesh(geometry, material);
    scene.add(cube);

    // Add axes helper
    axesHelper = new THREE.AxesHelper(5);
    axesHelper.lineWidth = 125;
    axesHelper.position.y += 0.001;

    scene.add(axesHelper);

    const axesHelper2 = new THREE.AxesHelper(5);
    axesHelper2.lineWidth = 125;
    axesHelper2.position.y -= 0.001;

    scene.add(axesHelper2);

    // Create a renderer
    renderer = new THREE.WebGLRenderer({ antialias: true, canvas: el });
    renderer.setSize(window.innerWidth - 100, window.innerHeight - 100);
    //renderer.setPixelRatio(window.devicePixelRatio * 1.1);
    //renderer.setClearColor(0xffffff, 1);

    // Set up TrackballControls
    //controls = new TrackballControls(camera, renderer.domElement);

    controls = new OrbitControls(camera, renderer.domElement);
    controls.update();

    //controls.rotateSpeed = 3;
    //controls.zoomSpeed = 1.2;
    //controls.panSpeed = 30;

    //controls.keys = ["KeyA", "KeyS", "KeyD"];

    //if (camera.position.y < 10) camera.position.set(0, 10, 0);
    //camera.position.set(0, camera.position.y, 0);
    //camera.up.set(0, 0, 1);
    ///camera.lookAt(new THREE.Vector3(0, 0, 0));

    // Add the renderer to the container element
  }

  function animate() {
    // Render the scene
    renderer.render(scene, camera);
    controls.update();
    // Animate the scene
    requestAnimationFrame(animate);
  }

  export const createScene = (el) => {
    init();
    addCube();
    animate();
  };
  let el;
  onMount(() => {
    el = createScene();
  });

  /*
import * as THREE from 'three';
import { onMount } from 'svelte';
import TrackballControls from 'three-trackballcontrols';

let camera, scene, renderer, gridHelper, controls, axesHelper;


function init() {

  // Set up scene and camera
  scene = new THREE.Scene();
  camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
  //camera = new THREE.OrthographicCamera(window.innerWidth / -2, window.innerWidth / 2, window.innerWidth / 2, window.innerWidth / -2, 1, 1000);

  camera.position.z = 10;
  camera.lookAt(new THREE.Vector3(0, 0, 0));

  // Set up renderer
  renderer = new THREE.WebGLRenderer({ antialias: true });
  //renderer.setPixelRatio(window.devicePixelRatio * 1.1);
  renderer.setClearColor(0xffffff, 1);

  
  renderer.setSize(window.innerWidth*0.5, window.innerWidth*0.5);
  //document.getElementById("container").appendChild(renderer.domElement);

  // Add grid helper
  gridHelper = new THREE.GridHelper(20, 40);
  scene.add(gridHelper);

  //const grid = new THREE.InfiniteGridHelper(10, 100);
  //scene.add(grid);

  // Add axes helper
  axesHelper = new THREE.AxesHelper(5);
  axesHelper.lineWidth = 125;
  axesHelper.position.y += 0.001;

  scene.add(axesHelper);


  const axesHelper2 = new THREE.AxesHelper(5);
  axesHelper2.lineWidth = 125;
  axesHelper2.position.y -= 0.001;

  scene.add(axesHelper2);

  // Set up TrackballControls
  controls = new TrackballControls(camera, renderer.domElement);
  controls.target.set(0, 0, 0);

/*
  // Create a button to orient the camera to the xy plane
  const xyButton = document.createElement("button");
  xyButton.innerHTML = "XY Plane";
  xyButton.addEventListener("click", function () {
      if (camera.position.z < 10)
          camera.position.set(0, 0, 10);
      camera.position.set(0, 0, camera.position.z);
      camera.up.set(0, 1, 0);
      camera.lookAt(new THREE.Vector3(0, 0, 0));
  });
  document.body.appendChild(xyButton);*/

  /*
  const xzButton = document.createElement("button");
  xzButton.innerHTML = "XZ Plane";
  xzButton.addEventListener("click", function () {
      if (camera.position.y < 10)
          camera.position.set(0, 10, 0);
      camera.position.set(0, camera.position.y, 0);
      camera.up.set(0, 0, 1);
      camera.lookAt(new THREE.Vector3(0, 0, 0));
  });
  document.body.appendChild(xzButton);*/

  /*
  // Set up G-Code file input
  const fileInput = document.getElementById("file-input");
  fileInput.addEventListener("change", function (e) {
      console.log("File selected")
      const file = e.target.files[0];
      const reader = new FileReader();
      reader.onload = function (e) {
          const gcode = e.target.result;
          // Parse G-Code file and trace toolpath
          let lines = gcode.split("\n");
          let x = 0, y = 0, z = 0;
          let lastX = 0, lastY = 0, lastZ = 0;
          for (let i = 0; i < lines.length; i++) {
              let line = lines[i];
              console.log(line)
              if (line.startsWith("G1")) {
                  // G1 - Linear Interpolation
                  const split = line.split(" ");
                  for (let j = 0; j < split.length; j++) {
                      if (split[j].startsWith("X")) {
                          x = parseFloat(split[j].substring(1));
                      } else if (split[j].startsWith("Y")) {
                          y = parseFloat(split[j].substring(1));
                      } else if (split[j].startsWith("Z")) {
                          z = parseFloat(split[j].substring(1));
                      }
                  }
                  const geometry = new THREE.Geometry();
                  geometry.vertices.push(
                      new THREE.Vector3(lastX, lastY, lastZ),
                      new THREE.Vector3(x, y, z)
                  );
                  lastX = x;
                  lastY = y;
                  lastZ = z;
                  const material = new THREE.LineBasicMaterial({ color: 0xff0000 });
                  const line2 = new THREE.Line(geometry, material);
                  scene.add(line2);
              }
              else if (line.startsWith("G25")) {
                  const split = line.split(" ");
                  let x, y, z, i, j, k;
                  for (let m = 0; m < split.length; m++) {
                      if (split[m].startsWith("X")) {
                          x = parseFloat(split[m].substring(1));
                      } else if (split[m].startsWith("Y")) {
                          y = parseFloat(split[m].substring(1));
                      } else if (split[m].startsWith("Z")) {
                          z = parseFloat(split[m].substring(1));
                      } else if (split[m].startsWith("I")) {
                          i = parseFloat(split[m].substring(1));
                      } else if (split[m].startsWith("J")) {
                          j = parseFloat(split[m].substring(1));
                      } else if (split[m].startsWith("K")) {
                          k = parseFloat(split[m].substring(1));
                      }
                  }

                  // Calculate the center point of the arc
                  const centerX = lastX + i;
                  const centerY = lastY + j;
                  const centerZ = lastZ + k;

                  // Calculate the start and end angles of the arc
                  const startAngle = Math.atan2(lastY - centerY, lastX - centerX);
                  const endAngle = Math.atan2(y - centerY, x - centerX);

                  // Define the number of steps to take along the arc
                  const steps = 50;

                  // Calculate the step size based on the start and end angles
                  const stepSize = (endAngle - startAngle) / steps;

                  // Generate the vertices for the arc
                  const vertices = [];
                  for (let n = 0; n <= steps; n++) {
                      const angle = startAngle + n * stepSize;
                      const vertexX = centerX + (lastX - centerX) * Math.cos(angle) - (lastY - centerY) * Math.sin(angle);
                      const vertexY = centerY + (lastX - centerX) * Math.sin(angle) + (lastY - centerY) * Math.cos(angle);
                      const vertexZ = lastZ + n * (z - lastZ) / steps;
                      vertices.push(new THREE.Vector3(vertexX, vertexY, vertexZ));
                  }

                  // Create a geometry from the vertices
                  const geometry = new THREE.Geometry();
                  geometry.vertices = vertices;

                  // Create a material for the line
                  const material = new THREE.LineBasicMaterial({ color: 0xff0000 });

                  // Create a line from the geometry and material
                  const arc = new THREE.Line(geometry, material);

                  // Add the line to the scene
                  scene.add(arc);

                  // Update the last position
                  lastX = x;
                  lastY = y;
                  lastZ = z;
              }

              else if (line.startsWith("G3")) {
                  const split = line.split(" ");
                  let x, y, z, i, j, k;
                  for (let m = 0; m < split.length; m++) {
                      if (split[m].startsWith("X")) {
                          x = parseFloat(split[m].substring(1));
                      } else if (split[m].startsWith("Y")) {
                          y = parseFloat(split[m].substring(1));
                      } else if (split[m].startsWith("Z")) {
                          z = parseFloat(split[m].substring(1));
                      } else if (split[m].startsWith("I")) {
                          i = parseFloat(split[m].substring(1));
                      } else if (split[m].startsWith("J")) {
                          j = parseFloat(split[m].substring(1));
                      } else if (split[m].startsWith("K")) {
                          k = parseFloat(split[m].substring(1));
                      }
                  }

                  // Calculate the center point of the arc
                  const centerX = lastX + i;
                  const centerY = lastY + j;
                  const centerZ = lastZ + k;

                  // Calculate the start and end angles of the arc
                  const startAngle = Math.atan2(lastY - centerY, lastX - centerX);
                  const endAngle = Math.atan2(y - centerY, x - centerX);

                  // Define the number of steps to take along the arc
                  const steps = 50;

                  // Calculate the step size based on the start and end angles
                  const stepSize = (endAngle - startAngle) / steps;

                  // Generate the vertices for the arc
                  const vertices = [];
                  for (let n = 0; n <= steps; n++) {
                      const angle = startAngle + n * stepSize;
                      const vertexX = centerX + (lastX - centerX) * Math.cos(angle) - (lastY - centerY) * Math.sin(angle);
                      const vertexY = centerY + (lastX - centerX) * Math.sin(angle) + (lastY - centerY) * Math.cos(angle);
                      const vertexZ = lastZ + n * (z - lastZ) / steps;
                      vertices.push(new THREE.Vector3(vertexX, vertexY, vertexZ));
                  }

                  // Create a geometry from the vertices
                  const geometry = new THREE.Geometry();
                  geometry.vertices = vertices;

                  // Create a material for the line
                  const material = new THREE.LineBasicMaterial({ color: 0xff0000 });

                  // Create a line from the geometry and material
                  const arc = new THREE.Line(geometry, material);

                  // Add the line to the scene
                  scene.add(arc);

                  // Update the last position
                  lastX = x;
                  lastY = y;
                  lastZ = z;
              }

          }
      }
      reader.readAsText(file);
  });
*/

  //}
  function addCube() {
    // Create a cube
    const geometry = new THREE.BoxGeometry();
    const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
    cube = new THREE.Mesh(geometry, material);

    // Add the cube to the scene
    scene.add(cube);
  }

  /*

// Render loop
function animate() {
    
    requestAnimationFrame(animate);
    controls.update();
    renderer.render(scene, camera);
};

onMount(() => {
  init();
  addCube();
  animate();
});*/
</script>

<canvas bind:this={el} />
