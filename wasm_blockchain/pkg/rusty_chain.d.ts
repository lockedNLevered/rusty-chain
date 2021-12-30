/* tslint:disable */
/* eslint-disable */
/**
*/
export function init_panic_hook(): void;
/**
*/
export class Block {
  free(): void;
/**
* @returns {number}
*/
  get_index(): number;
/**
* @returns {number}
*/
  get_nonce(): number;
}
/**
*/
export class BlockChain {
  free(): void;
/**
*/
  constructor();
/**
* @returns {any}
*/
  get_chain(): any;
/**
* @param {string} sender
* @param {string} recipient
* @param {number} amount
* @returns {number}
*/
  new_transaction(sender: string, recipient: string, amount: number): number;
/**
* @param {number} nonce
*/
  new_block(nonce: number): void;
/**
* @returns {Block}
*/
  last_block(): Block;
/**
* @param {number} last_nonce
* @returns {number}
*/
  proof_of_work(last_nonce: number): number;
}
