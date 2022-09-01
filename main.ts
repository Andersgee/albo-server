import { serve } from "https://deno.land/std@0.153.0/http/server.ts";

const port = 8080;

const sockets: Map<number, WebSocket> = new Map();

const entities = { hej: 99 };

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
    console.log("stopped game loop");
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
      console.log("socket opened");
      maybeStartGameLoop();
    };
    socket.onmessage = (e) => {
      //handle player input here
      console.log("socket message:", e.data);
      //socket.send("recieved input");
    };
    socket.onerror = (e) => {
      sockets.delete(id);
      console.log("socket errored:", e);
      maybeStopGameLoop();
    };
    socket.onclose = () => {
      sockets.delete(id);
      console.log("socket closed");
      maybeStopGameLoop();
    };

    return response;
  },
  {
    port,
    onListen: () => {
      console.log(`Server listening on ${port}`);
    },
    onError: (e: unknown) => {
      console.error("Server error", e);
      return new Response(JSON.stringify({ message: "Server error" }), {
        status: 500,
      });
    },
  }
);
/*
async function handleConn(conn: Deno.Conn) {
  const httpConn = Deno.serveHttp(conn);
  for await (const e of httpConn) {
    e.respondWith(handle(e.request));
  }
}

function handle(req: Request) {
  if (req.headers.get("upgrade") != "websocket") {
    //return new Response("not trying to upgrade as websocket.");
    return new Response(null, { status: 501 });
  }
  const { socket, response } = Deno.upgradeWebSocket(req);
  socket.onopen = () => console.log("socket opened");
  socket.onmessage = (e) => {
    //handle player input here
    console.log("socket message:", e.data);
    socket.send(new Date().toString());
  };
  socket.onerror = (e) => console.log("socket errored:", e);
  socket.onclose = () => console.log("socket closed");

  return response;
}

const listener = Deno.listen({ port: 8080 });
console.log("listening on http://localhost:8080");
for await (const conn of listener) {
  handleConn(conn);
}
*/
