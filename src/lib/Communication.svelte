<script context="module">
  import * as protobuf from "protobufjs";

  let JogMessage;
  let socket;
  let PositionMessage;
  let MessageFromInterface;
  let MessageFromCnc;

  protobuf.load("src/protobuf/jog_message.proto", function (err, root) {
    if (err) throw err;
    JogMessage = root.lookupType("Jog");
    MessageFromInterface = root.lookupType("MessageFromInterface");
    MessageFromCnc = root.lookupType("MessageFromCnc");
    PositionMessage = root.lookupType("Status");
  });

  const jog = (axis, direction) => {
    let message = MessageFromInterface.create({
      jog: JogMessage.create({ axis, direction }),
    });
    //let message = JogMessage.create({ axis, direction });
    let buffer = MessageFromInterface.encode(message).finish();
    socket.send(buffer);
  };

  export const jog_x_plus = () => jog(JogMessage.Axis.X, 1);
  export const jog_x_minus = () => jog(JogMessage.Axis.X, -1);
  export const jog_y_plus = () => jog(JogMessage.Axis.Y, 1);
  export const jog_y_minus = () => jog(JogMessage.Axis.Y, -1);
  export const jog_z_plus = () => jog(JogMessage.Axis.Z, 1);
  export const jog_z_minus = () => jog(JogMessage.Axis.Z, -1);
</script>

<script>
  import { onMount } from "svelte";
  import { move_cube } from "./Toolpath.svelte";

  let x = 0;
  let y = 0;
  let z = 0;

  const handleWebSocketMessage = (event) => {
    if (event.data instanceof Blob) {
      let fileReader = new FileReader();
      fileReader.onload = function () {
        let arrayBuffer = this.result,
          byteArray = new Uint8Array(arrayBuffer);
        let decodedMessage = MessageFromCnc.decode(byteArray);
        console.log(decodedMessage);
        x = decodedMessage.status.position.x;
        y = decodedMessage.status.position.y;
        z = decodedMessage.status.position.z;
        move_cube(
          decodedMessage.status.position.x,
          decodedMessage.status.position.y,
          decodedMessage.status.position.z
        );
      };
      fileReader.readAsArrayBuffer(event.data);
    } else {
      console.log("Data is not in Blob format.");
    }
  };

  function connectWebSocket() {
    console.log("Connecting to WebSocket");
    socket = new WebSocket("ws://localhost:8081/ws");

    socket.addEventListener("message", handleWebSocketMessage);

    socket.addEventListener("open", () => console.log("WebSocket connected"));

    socket.addEventListener("close", () => {
      console.log("WebSocket disconnected");
      // Reconnect after a delay
      setTimeout(connectWebSocket, 3000); // 3 seconds
    });

    socket.addEventListener("error", (error) =>
      console.error("WebSocket error:", error)
    );
  }

  onMount(connectWebSocket);
</script>

<div>
  <p>
    X: {x.toFixed(3)} Y: {y.toFixed(3)} Z: {z.toFixed(3)}
  </p>
</div>
