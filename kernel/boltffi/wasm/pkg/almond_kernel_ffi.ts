import { WireReader, WireWriter, BoltFFIModule, instantiateBoltFFI, wireStringSize } from "@boltffi/runtime";
import type { Duration, WireCodec, BoltFFIImports, BoltFFIExports } from "@boltffi/runtime";

const EXPECTED_ABI_VERSION = 1;
const _callback_handle_js_namespace_start = 0x80000000;
const _callback_handle_key = (handle: number): number => handle >>> 0;

let _module: BoltFFIModule;
let _exports: BoltFFIExports;
const _callbackImports: Record<string, WebAssembly.ImportValue> = {};

export default async function init(source: BufferSource | Response): Promise<void> {
  _module = await instantiateBoltFFI(source, EXPECTED_ABI_VERSION, { env: _callbackImports });
  _exports = _module.exports;
}
export interface WasmExports {
  boltffi_wasm_abi_version(): number;
  boltffi_wasm_alloc(size: number): number;
  boltffi_wasm_free(ptr: number, size: number): void;
  boltffi_free_buf(ptr: number): void;
}
