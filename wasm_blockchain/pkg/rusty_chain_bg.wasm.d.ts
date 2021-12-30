/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function __wbg_block_free(a: number): void;
export function block_get_index(a: number): number;
export function block_get_nonce(a: number): number;
export function __wbg_blockchain_free(a: number): void;
export function blockchain_new(): number;
export function blockchain_get_chain(a: number): number;
export function blockchain_new_transaction(a: number, b: number, c: number, d: number, e: number, f: number): number;
export function blockchain_new_block(a: number, b: number): void;
export function blockchain_last_block(a: number): number;
export function blockchain_proof_of_work(a: number, b: number): number;
export function init_panic_hook(): void;
export function __wbindgen_malloc(a: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number): number;
export function __wbindgen_free(a: number, b: number): void;
