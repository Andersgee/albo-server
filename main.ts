import { serve } from "https://deno.land/std@0.153.0/http/server.ts";
import init, { Game } from "./pkg/nextjs_rust_playground_server.js";

await init();
const game = new Game(); //glue for src/lib.rs

const PORT = 8080;

const sockets: Map<number, WebSocket> = new Map();

const TICKS_PER_SECOND = 1; //keep this
const TICK_DURATION_MS = 1000 / TICKS_PER_SECOND;

/** update state and send to all connected sockets */
function updategamestate() {
  game.tick();
  const state = game.state;
  //entities.hej += 1;
  const data = JSON.stringify(state);
  for (const [_, socket] of sockets) {
    socket.send(data);
  }
}

let intervalId: number | null = null;

/** start game if its not running */
function maybeStartGameLoop() {
  if (!intervalId) {
    intervalId = setInterval(updategamestate, TICK_DURATION_MS);
    console.log("started game loop");
  }
}

/** stop game if there no players */
function maybeStopGameLoop() {
  if ([...sockets.keys()].length < 1 && intervalId) {
    clearInterval(intervalId);
    intervalId = null;
    console.log("stopped game loop (no connected sockets)");
  }
}

let last_socket_id = 1;

await serve(
  (req: Request) => {
    if (req.headers.get("upgrade") != "websocket") {
      return new Response(null, { status: 501 });
    }

    const { socket, response } = Deno.upgradeWebSocket(req);

    const socket_id = last_socket_id;
    last_socket_id += 1;
    console.log(`Client connected, it got id: ${socket_id}`);

    socket.onopen = () => {
      sockets.set(socket_id, socket);
      game.add_player(socket_id);
      console.log("onopen, game.players:", game.players);
      maybeStartGameLoop();
    };
    socket.onmessage = ({ data }) => {
      //data can be string | ArrayBuffer | Blob
      //console.log('typeof data == "string"', typeof data == "string");
      //console.log("data instanceof ArrayBuffer", data instanceof ArrayBuffer);
      //console.log("data instanceof Blob", data instanceof Blob);

      if (data instanceof ArrayBuffer) {
        const playerInput = new Uint8Array(data);
        //playerInputs.set(id, playerInput);

        const [step_forward, step_backward, step_left, step_right] =
          playerInput;
        game.set_player_input(
          socket_id,
          step_forward,
          step_backward,
          step_left,
          step_right
        );

        //console.log("stepForward:", stepForward);
        console.log("playerInput:", playerInput);
      }
    };
    socket.onerror = (e) => {
      sockets.delete(socket_id);
      game.remove_player(socket_id);
      console.log("onerror, game.players:", game.players);
      maybeStopGameLoop();
    };
    socket.onclose = () => {
      sockets.delete(socket_id);
      game.remove_player(socket_id);
      console.log("onclose, game.players:", game.players);
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
