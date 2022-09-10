/* tslint:disable */
/* eslint-disable */
/**
*/
export class Game {
  free(): void;
/**
*/
  constructor();
/**
*/
  tick(): void;
/**
* @param {number} socket_id
*/
  add_player(socket_id: number): void;
/**
* @param {number} socket_id
*/
  remove_player(socket_id: number): void;
/**
* @param {number} socket_id
* @param {number} step_forward
* @param {number} step_backward
* @param {number} step_left
* @param {number} step_right
*/
  set_player_input(socket_id: number, step_forward: number, step_backward: number, step_left: number, step_right: number): void;
/**
*/
  readonly client_entities: Array<any>;
/**
*/
  readonly players: Array<any>;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_game_free: (a: number) => void;
  readonly game_new: () => number;
  readonly game_tick: (a: number) => void;
  readonly game_client_entities: (a: number) => number;
  readonly game_players: (a: number) => number;
  readonly game_add_player: (a: number, b: number) => void;
  readonly game_remove_player: (a: number, b: number) => void;
  readonly game_set_player_input: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
