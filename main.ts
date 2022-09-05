import { serve } from "https://deno.land/std@0.153.0/http/server.ts";
import init, { Game } from "./pkg/nextjs_rust_playground_server.js";

await init();
const game = new Game(); //glue for src/lib.rs
const PORT = 8080;
const sockets: Map<number, WebSocket> = new Map();
const TICKS_PER_SECOND = 1; //keep this
const TICK_DURATION_MS = 1000 / TICKS_PER_SECOND;

let intervalId: number | null = null;
let id_counter = 1;

/** update state and send to all connected sockets */
function updateGameState() {
  game.tick();
  const renderable = game.renderable;
  const data = JSON.stringify(renderable);
  for (const [_, socket] of sockets) {
    socket.send(data);
  }
  console.log("game.players:", game.players);
  console.log("game.renderable:", game.renderable);
}

/** start game if its not running */
function maybeStartGameLoop() {
  if (!intervalId) {
    intervalId = setInterval(updateGameState, TICK_DURATION_MS);
    console.log("started game loop");
  }
}

/** stop game if there no players */
function maybeStopGameLoop() {
  if (intervalId && sockets.size < 1) {
    clearInterval(intervalId);
    intervalId = null;
    console.log("stopped game loop (no connected sockets)");
  }
}

function onSocketOpen(socket_id: number, socket: WebSocket) {
  sockets.set(socket_id, socket);
  game.add_player(socket_id);
  console.log("onopen, game.players:", game.players);
  maybeStartGameLoop();
}

function onSocketClose(socket_id: number) {
  sockets.delete(socket_id);
  game.remove_player(socket_id);
  console.log("onclose, game.players:", game.players);
  maybeStopGameLoop();
}

function onSocketMessage(socket_id: number, data: string | ArrayBuffer | Blob) {
  //data can be string | ArrayBuffer | Blob
  //console.log('typeof data == "string"', typeof data == "string");
  //console.log("data instanceof ArrayBuffer", data instanceof ArrayBuffer);
  //console.log("data instanceof Blob", data instanceof Blob);

  if (data instanceof ArrayBuffer) {
    const playerInput = new Uint8Array(data);

    const [step_forward, step_backward, step_left, step_right] = playerInput;
    game.set_player_input(
      socket_id,
      step_forward,
      step_backward,
      step_left,
      step_right
    );
  }
}

await serve(
  (req: Request) => {
    if (req.headers.get("upgrade") != "websocket") {
      return new Response(null, { status: 501 });
    }

    const { socket, response } = Deno.upgradeWebSocket(req);

    const socket_id = id_counter;
    id_counter += 1;
    socket.onopen = () => onSocketOpen(socket_id, socket);
    socket.onclose = () => onSocketClose(socket_id);
    socket.onerror = () => onSocketClose(socket_id);
    socket.onmessage = (e) => onSocketMessage(socket_id, e.data);
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
