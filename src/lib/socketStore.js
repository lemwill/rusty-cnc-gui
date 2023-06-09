import { writable } from "svelte/store";
import * as protobuf from "protobufjs";

export let MessageFromInterface, MessageFromCnc;
export let socket;
export let JogMessage;

protobuf.load("src/protobuf/jog_message.proto", function (err, root) {
  if (err) throw err;
  JogMessage = root.lookupType("Jog");
  MessageFromInterface = root.lookupType("MessageFromInterface");
  MessageFromCnc = root.lookupType("MessageFromCnc");
});

// Writable store to hold the position
export const position = writable({ x: 0, y: 0, z: 0 });

// Function to send a jog message to the server
export function jog(axis, direction) {
  let message = MessageFromInterface.create({
    jog: JogMessage.create({ axis: axis, direction }),
  });
  let buffer = MessageFromInterface.encode(message).finish();
  socket.send(buffer);
}

function connectWebSocket() {
  console.log("Connecting to WebSocket");
  socket = new WebSocket("ws://localhost:8081/ws");

  socket.addEventListener("message", handleWebSocketMessage);

  socket.addEventListener("open", () => console.log("WebSocket connected"));

  socket.addEventListener("close", () => {
    console.log("WebSocket disconnected");
    setTimeout(connectWebSocket, 3000);
  });

  socket.addEventListener("error", (error) =>
    console.error("WebSocket error:", error)
  );
}

function handleWebSocketMessage(event) {
  if (event.data instanceof Blob) {
    let fileReader = new FileReader();
    fileReader.onload = function () {
      let arrayBuffer = this.result,
        byteArray = new Uint8Array(arrayBuffer);
      let decodedMessage = MessageFromCnc.decode(byteArray);
      console.log(decodedMessage);
      position.set(decodedMessage.status.position);
    };
    fileReader.readAsArrayBuffer(event.data);
  } else {
    console.log("Data is not in Blob format.");
  }
}

connectWebSocket();
