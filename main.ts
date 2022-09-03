import { serve } from "https://deno.land/std@0.153.0/http/server.ts";
import init, { Game } from "./pkg/nextjs_rust_playground_server.js";

await init();
const game = new Game();

console.log(game);
console.log(game.stuff);

console.log("game.state", game.state);

const PORT = 8080;

const sockets: Map<number, WebSocket> = new Map();
const playerInputs: Map<number, Uint8Array> = new Map();

const entities = { hej: 0 };

const TICKS_PER_SECOND = 1;
const TICK_DURATION_MS = 1000 / TICKS_PER_SECOND;

function gameloop() {
  //update state and send to all connected sockets
  entities.hej += 1;
  const data = JSON.stringify(entities);
  for (const [_, socket] of sockets) {
    socket.send(data);
  }
}

let intervalId: number | null = null;

function maybeStartGameLoop() {
  if (!intervalId) {
    intervalId = setInterval(gameloop, TICK_DURATION_MS);
    console.log("started game loop");
  }
}

function maybeStopGameLoop() {
  if ([...sockets.keys()].length < 1 && intervalId) {
    clearInterval(intervalId);
    intervalId = null;
    console.log("stopped game loop (no connected sockets)");
  }
}

/**  */
let socketId = 1;

await serve(
  (req: Request) => {
    if (req.headers.get("upgrade") != "websocket") {
      return new Response(null, { status: 501 });
    }

    const { socket, response } = Deno.upgradeWebSocket(req);

    const id = socketId;
    socketId += 1;
    console.log(`Client connected, it got id: ${id}`);

    socket.onopen = () => {
      sockets.set(id, socket);
      console.log("onopen");
      maybeStartGameLoop();
    };
    socket.onmessage = ({ data }) => {
      //data can be string | ArrayBuffer | Blob
      //console.log('typeof data == "string"', typeof data == "string");
      //console.log("data instanceof ArrayBuffer", data instanceof ArrayBuffer);
      //console.log("data instanceof Blob", data instanceof Blob);

      if (data instanceof ArrayBuffer) {
        const playerInput = new Uint8Array(data);
        playerInputs.set(id, playerInput);

        const [stepForward, stepBackward, stepLeft, stepRight] = playerInput;
        //console.log("stepForward:", stepForward);
        console.log("playerInput:", playerInput);
      }
    };
    socket.onerror = (e) => {
      sockets.delete(id);
      console.log("onerror:", e);
      maybeStopGameLoop();
    };
    socket.onclose = () => {
      sockets.delete(id);
      console.log("onclose");
      maybeStopGameLoop();
    };

    return response;
  },
  {
    port: PORT,
    onListen: () => {
      console.log(`Server listening on ${PORT}`);
    },
    onError: (e: unknown) => {
      console.error("Server error", e);
      return new Response(JSON.stringify({ message: "Server error" }), {
        status: 500,
      });
    },
  }
);
