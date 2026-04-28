import { WireReader, WireWriter, BoltFFIModule, instantiateBoltFFISync, wireStringSize } from "@boltffi/runtime";
import type { Duration, WireCodec, BoltFFIImports, BoltFFIExports } from "@boltffi/runtime";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const EXPECTED_ABI_VERSION = 1;
const _callback_handle_js_namespace_start = 0x80000000;
const _callback_handle_key = (handle: number): number => handle >>> 0;
const _thisDir = dirname(fileURLToPath(import.meta.url));
const _wasmPath = join(_thisDir, "almond_kernel_ffi_bg.wasm");
const _callbackImports: Record<string, WebAssembly.ImportValue> = {};
export interface WasmExports {
  boltffi_wasm_abi_version(): number;
  boltffi_wasm_alloc(size: number): number;
  boltffi_wasm_free(ptr: number, size: number): void;
  boltffi_free_buf(ptr: number): void;
}
const _wasmBytes = readFileSync(_wasmPath);
const _module: BoltFFIModule = instantiateBoltFFISync(_wasmBytes, EXPECTED_ABI_VERSION, { env: _callbackImports });
const _exports: BoltFFIExports = _module.exports;

export const initialized = Promise.resolve();
export default function init(): Promise<void> { return Promise.resolve(); }

