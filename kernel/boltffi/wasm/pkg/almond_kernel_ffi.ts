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
export interface Model {
  readonly title: string;
  readonly url: string;
  readonly tag: Tag;
  readonly createdAt: DateTimeWithTimeZone;
  readonly updatedAt: DateTimeWithTimeZone;
}

const ModelCodec: WireCodec<Model> = {
  size: (v) => (4 + wireStringSize(v.title)) + (4 + wireStringSize(v.url)) + 4 + 8 + 8,
  encode: (writer, v) => {
    writer.writeString(v.title);
    writer.writeString(v.url);
    TagCodec.encode(writer, v.tag);
    writer.writeI64(v.createdAt);
    writer.writeI64(v.updatedAt);
  },
  decode: (reader) => {
    return {
      title: reader.readString(),
      url: reader.readString(),
      tag: TagCodec.decode(reader),
      createdAt: reader.readI64(),
      updatedAt: reader.readI64(),
    };
  },
};

export enum ItemType {
  Todo = 0,
  Note = 1,
  Reminder = 2,
  Snippet = 3,
  Bookmark = 4,
}

const ItemTypeCodec: WireCodec<ItemType> = {
  size: (_v) => 4,
  encode: (writer, v) => {
    switch (v) {
      case ItemType.Todo:
        writer.writeI32(0);
        break;
      case ItemType.Note:
        writer.writeI32(1);
        break;
      case ItemType.Reminder:
        writer.writeI32(2);
        break;
      case ItemType.Snippet:
        writer.writeI32(3);
        break;
      case ItemType.Bookmark:
        writer.writeI32(4);
        break;
    }
  },
  decode: (reader) => {
    const value = reader.readI32();
    switch (value) {
      case 0: return ItemType.Todo;
      case 1: return ItemType.Note;
      case 2: return ItemType.Reminder;
      case 3: return ItemType.Snippet;
      case 4: return ItemType.Bookmark;
      default: throw new Error(`Unknown ItemType wire tag: ${value}`);
    }
  },
};

export enum Priority {
  High = 0,
  Medium = 1,
  Low = 2,
}

const PriorityCodec: WireCodec<Priority> = {
  size: (_v) => 4,
  encode: (writer, v) => {
    switch (v) {
      case Priority.High:
        writer.writeI32(0);
        break;
      case Priority.Medium:
        writer.writeI32(1);
        break;
      case Priority.Low:
        writer.writeI32(2);
        break;
    }
  },
  decode: (reader) => {
    const value = reader.readI32();
    switch (value) {
      case 0: return Priority.High;
      case 1: return Priority.Medium;
      case 2: return Priority.Low;
      default: throw new Error(`Unknown Priority wire tag: ${value}`);
    }
  },
};

export enum Tag {
  Development = 0,
  Inspiration = 1,
  Design = 2,
  Research = 3,
}

const TagCodec: WireCodec<Tag> = {
  size: (_v) => 4,
  encode: (writer, v) => {
    switch (v) {
      case Tag.Development:
        writer.writeI32(0);
        break;
      case Tag.Inspiration:
        writer.writeI32(1);
        break;
      case Tag.Design:
        writer.writeI32(2);
        break;
      case Tag.Research:
        writer.writeI32(3);
        break;
    }
  },
  decode: (reader) => {
    const value = reader.readI32();
    switch (value) {
      case 0: return Tag.Development;
      case 1: return Tag.Inspiration;
      case 2: return Tag.Design;
      case 3: return Tag.Research;
      default: throw new Error(`Unknown Tag wire tag: ${value}`);
    }
  },
};

export interface WasmExports {
  boltffi_wasm_abi_version(): number;
  boltffi_wasm_alloc(size: number): number;
  boltffi_wasm_free(ptr: number, size: number): void;
  boltffi_free_buf(ptr: number): void;
}
