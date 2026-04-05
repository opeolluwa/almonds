import { WireReader, WireWriter, BoltFFIModule, instantiateBoltFFISync, wireStringSize } from "@boltffi/runtime";
import type { Duration, WireCodec, BoltFFIImports, BoltFFIExports } from "@boltffi/runtime";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const EXPECTED_ABI_VERSION = 1;
const _callback_handle_js_namespace_start = 0x80000000;
const _callback_handle_key = (handle: number): number => handle >>> 0;
const _thisDir = dirname(fileURLToPath(import.meta.url));
const _wasmPath = join(_thisDir, "almond_kernel_bg.wasm");
const _callbackImports: Record<string, WebAssembly.ImportValue> = {};
export interface CreateBookmark {
  readonly title: string;
  readonly url: string;
  readonly tag: BookmarkTag;
}

const CreateBookmarkCodec: WireCodec<CreateBookmark> = {
  size: (v) => (4 + wireStringSize(v.title)) + (4 + wireStringSize(v.url)) + 4,
  encode: (writer, v) => {
    writer.writeString(v.title);
    writer.writeString(v.url);
    BookmarkTagCodec.encode(writer, v.tag);
  },
  decode: (reader) => {
    return {
      title: reader.readString(),
      url: reader.readString(),
      tag: BookmarkTagCodec.decode(reader),
    };
  },
};

export interface UpdateBookmark {
  readonly title: string | null;
  readonly url: string | null;
  readonly tag: BookmarkTag | null;
}

const UpdateBookmarkCodec: WireCodec<UpdateBookmark> = {
  size: (v) => (v.title !== null ? 1 + (4 + wireStringSize(v.title)) : 1) + (v.url !== null ? 1 + (4 + wireStringSize(v.url)) : 1) + (v.tag !== null ? 1 + 4 : 1),
  encode: (writer, v) => {
    writer.writeOptional(v.title, (v) => { writer.writeString(v) });
    writer.writeOptional(v.url, (v) => { writer.writeString(v) });
    writer.writeOptional(v.tag, (v) => { BookmarkTagCodec.encode(writer, v) });
  },
  decode: (reader) => {
    return {
      title: reader.readOptional(() => reader.readString()),
      url: reader.readOptional(() => reader.readString()),
      tag: reader.readOptional(() => BookmarkTagCodec.decode(reader)),
    };
  },
};

export interface RequestMeta {
  readonly workspaceIdentifier: string;
}

const RequestMetaCodec: WireCodec<RequestMeta> = {
  size: (v) => 16,
  encode: (writer, v) => {
    writer.writeUuid(v.workspaceIdentifier);
  },
  decode: (reader) => {
    return {
      workspaceIdentifier: reader.readUuid(),
    };
  },
};

export interface CreateNote {
  readonly title: string;
  readonly content: string;
  readonly categories: string[] | null;
  readonly workspaceIdentifier: string | null;
}

const CreateNoteCodec: WireCodec<CreateNote> = {
  size: (v) => (4 + wireStringSize(v.title)) + (4 + wireStringSize(v.content)) + (v.categories !== null ? 1 + (4 + v.categories.reduce((acc, item) => acc + (4 + wireStringSize(item)), 0)) : 1) + (v.workspaceIdentifier !== null ? 1 + 16 : 1),
  encode: (writer, v) => {
    writer.writeString(v.title);
    writer.writeString(v.content);
    writer.writeOptional(v.categories, (v) => { writer.writeArray(v, (item) => { writer.writeString(item) }) });
    writer.writeOptional(v.workspaceIdentifier, (v) => { writer.writeUuid(v) });
  },
  decode: (reader) => {
    return {
      title: reader.readString(),
      content: reader.readString(),
      categories: reader.readOptional(() => reader.readArray(() => reader.readString())),
      workspaceIdentifier: reader.readOptional(() => reader.readUuid()),
    };
  },
};

export interface UpdateNote {
  readonly title: string | null;
  readonly content: string | null;
  readonly categories: string[] | null;
}

const UpdateNoteCodec: WireCodec<UpdateNote> = {
  size: (v) => (v.title !== null ? 1 + (4 + wireStringSize(v.title)) : 1) + (v.content !== null ? 1 + (4 + wireStringSize(v.content)) : 1) + (v.categories !== null ? 1 + (4 + v.categories.reduce((acc, item) => acc + (4 + wireStringSize(item)), 0)) : 1),
  encode: (writer, v) => {
    writer.writeOptional(v.title, (v) => { writer.writeString(v) });
    writer.writeOptional(v.content, (v) => { writer.writeString(v) });
    writer.writeOptional(v.categories, (v) => { writer.writeArray(v, (item) => { writer.writeString(item) }) });
  },
  decode: (reader) => {
    return {
      title: reader.readOptional(() => reader.readString()),
      content: reader.readOptional(() => reader.readString()),
      categories: reader.readOptional(() => reader.readArray(() => reader.readString())),
    };
  },
};

export interface CreateConversationHistory {
  readonly bookmarked: boolean;
}

const CreateConversationHistoryCodec: WireCodec<CreateConversationHistory> = {
  size: (_v) => 1,
  encode: (writer, v) => {
    writer.writeBool(v.bookmarked);
  },
  decode: (reader) => {
    return {
      bookmarked: reader.readBool(),
    };
  },
};

export interface UpdateConversationHistory {
  readonly bookmarked: boolean | null;
}

const UpdateConversationHistoryCodec: WireCodec<UpdateConversationHistory> = {
  size: (v) => (v.bookmarked !== null ? 1 + 1 : 1),
  encode: (writer, v) => {
    writer.writeOptional(v.bookmarked, (v) => { writer.writeBool(v) });
  },
  decode: (reader) => {
    return {
      bookmarked: reader.readOptional(() => reader.readBool()),
    };
  },
};

export interface CreateConversationPrompt {
  readonly historyId: string;
  readonly content: string;
}

const CreateConversationPromptCodec: WireCodec<CreateConversationPrompt> = {
  size: (v) => 16 + (4 + wireStringSize(v.content)),
  encode: (writer, v) => {
    writer.writeUuid(v.historyId);
    writer.writeString(v.content);
  },
  decode: (reader) => {
    return {
      historyId: reader.readUuid(),
      content: reader.readString(),
    };
  },
};

export interface CreateConversationResponse {
  readonly historyId: string;
  readonly content: string;
}

const CreateConversationResponseCodec: WireCodec<CreateConversationResponse> = {
  size: (v) => 16 + (4 + wireStringSize(v.content)),
  encode: (writer, v) => {
    writer.writeUuid(v.historyId);
    writer.writeString(v.content);
  },
  decode: (reader) => {
    return {
      historyId: reader.readUuid(),
      content: reader.readString(),
    };
  },
};

export interface CreateRecycleBinEntry {
  readonly itemId: string;
  readonly itemType: RecycleBinItemType;
  readonly payload: string;
  readonly workspaceIdentifier: string | null;
}

const CreateRecycleBinEntryCodec: WireCodec<CreateRecycleBinEntry> = {
  size: (v) => 16 + 4 + (4 + wireStringSize(v.payload)) + (v.workspaceIdentifier !== null ? 1 + 16 : 1),
  encode: (writer, v) => {
    writer.writeUuid(v.itemId);
    RecycleBinItemTypeCodec.encode(writer, v.itemType);
    writer.writeString(v.payload);
    writer.writeOptional(v.workspaceIdentifier, (v) => { writer.writeUuid(v) });
  },
  decode: (reader) => {
    return {
      itemId: reader.readUuid(),
      itemType: RecycleBinItemTypeCodec.decode(reader),
      payload: reader.readString(),
      workspaceIdentifier: reader.readOptional(() => reader.readUuid()),
    };
  },
};

export interface CreateReminder {
  readonly title: string;
  readonly description: string | null;
  readonly recurring: boolean;
  readonly recurrenceRule: string | null;
  readonly alarmSound: string | null;
  readonly workspaceIdentifier: string | null;
}

const CreateReminderCodec: WireCodec<CreateReminder> = {
  size: (v) => (4 + wireStringSize(v.title)) + (v.description !== null ? 1 + (4 + wireStringSize(v.description)) : 1) + 1 + (v.recurrenceRule !== null ? 1 + (4 + wireStringSize(v.recurrenceRule)) : 1) + (v.alarmSound !== null ? 1 + (4 + wireStringSize(v.alarmSound)) : 1) + (v.workspaceIdentifier !== null ? 1 + 16 : 1),
  encode: (writer, v) => {
    writer.writeString(v.title);
    writer.writeOptional(v.description, (v) => { writer.writeString(v) });
    writer.writeBool(v.recurring);
    writer.writeOptional(v.recurrenceRule, (v) => { writer.writeString(v) });
    writer.writeOptional(v.alarmSound, (v) => { writer.writeString(v) });
    writer.writeOptional(v.workspaceIdentifier, (v) => { writer.writeUuid(v) });
  },
  decode: (reader) => {
    return {
      title: reader.readString(),
      description: reader.readOptional(() => reader.readString()),
      recurring: reader.readBool(),
      recurrenceRule: reader.readOptional(() => reader.readString()),
      alarmSound: reader.readOptional(() => reader.readString()),
      workspaceIdentifier: reader.readOptional(() => reader.readUuid()),
    };
  },
};

export interface UpdateReminder {
  readonly title: string | null;
  readonly description: string | null;
  readonly recurring: boolean | null;
  readonly recurrenceRule: string | null;
  readonly alarmSound: string | null;
}

const UpdateReminderCodec: WireCodec<UpdateReminder> = {
  size: (v) => (v.title !== null ? 1 + (4 + wireStringSize(v.title)) : 1) + (v.description !== null ? 1 + (4 + wireStringSize(v.description)) : 1) + (v.recurring !== null ? 1 + 1 : 1) + (v.recurrenceRule !== null ? 1 + (4 + wireStringSize(v.recurrenceRule)) : 1) + (v.alarmSound !== null ? 1 + (4 + wireStringSize(v.alarmSound)) : 1),
  encode: (writer, v) => {
    writer.writeOptional(v.title, (v) => { writer.writeString(v) });
    writer.writeOptional(v.description, (v) => { writer.writeString(v) });
    writer.writeOptional(v.recurring, (v) => { writer.writeBool(v) });
    writer.writeOptional(v.recurrenceRule, (v) => { writer.writeString(v) });
    writer.writeOptional(v.alarmSound, (v) => { writer.writeString(v) });
  },
  decode: (reader) => {
    return {
      title: reader.readOptional(() => reader.readString()),
      description: reader.readOptional(() => reader.readString()),
      recurring: reader.readOptional(() => reader.readBool()),
      recurrenceRule: reader.readOptional(() => reader.readString()),
      alarmSound: reader.readOptional(() => reader.readString()),
    };
  },
};

export interface Snippet {
  readonly title: string | null;
  readonly language: string | null;
  readonly code: string;
  readonly description: string | null;
  readonly isPinned: boolean;
}

const SnippetCodec: WireCodec<Snippet> = {
  size: (v) => (v.title !== null ? 1 + (4 + wireStringSize(v.title)) : 1) + (v.language !== null ? 1 + (4 + wireStringSize(v.language)) : 1) + (4 + wireStringSize(v.code)) + (v.description !== null ? 1 + (4 + wireStringSize(v.description)) : 1) + 1,
  encode: (writer, v) => {
    writer.writeOptional(v.title, (v) => { writer.writeString(v) });
    writer.writeOptional(v.language, (v) => { writer.writeString(v) });
    writer.writeString(v.code);
    writer.writeOptional(v.description, (v) => { writer.writeString(v) });
    writer.writeBool(v.isPinned);
  },
  decode: (reader) => {
    return {
      title: reader.readOptional(() => reader.readString()),
      language: reader.readOptional(() => reader.readString()),
      code: reader.readString(),
      description: reader.readOptional(() => reader.readString()),
      isPinned: reader.readBool(),
    };
  },
};

export interface CreateSnippet {
  readonly title: string | null;
  readonly language: string | null;
  readonly code: string;
  readonly description: string | null;
  readonly isPinned: boolean;
  readonly workspaceIdentifier: string | null;
}

const CreateSnippetCodec: WireCodec<CreateSnippet> = {
  size: (v) => (v.title !== null ? 1 + (4 + wireStringSize(v.title)) : 1) + (v.language !== null ? 1 + (4 + wireStringSize(v.language)) : 1) + (4 + wireStringSize(v.code)) + (v.description !== null ? 1 + (4 + wireStringSize(v.description)) : 1) + 1 + (v.workspaceIdentifier !== null ? 1 + 16 : 1),
  encode: (writer, v) => {
    writer.writeOptional(v.title, (v) => { writer.writeString(v) });
    writer.writeOptional(v.language, (v) => { writer.writeString(v) });
    writer.writeString(v.code);
    writer.writeOptional(v.description, (v) => { writer.writeString(v) });
    writer.writeBool(v.isPinned);
    writer.writeOptional(v.workspaceIdentifier, (v) => { writer.writeUuid(v) });
  },
  decode: (reader) => {
    return {
      title: reader.readOptional(() => reader.readString()),
      language: reader.readOptional(() => reader.readString()),
      code: reader.readString(),
      description: reader.readOptional(() => reader.readString()),
      isPinned: reader.readBool(),
      workspaceIdentifier: reader.readOptional(() => reader.readUuid()),
    };
  },
};

export interface UpdateSnippet {
  readonly title: string | null;
  readonly language: string | null;
  readonly code: string | null;
  readonly description: string | null;
  readonly isPinned: boolean | null;
}

const UpdateSnippetCodec: WireCodec<UpdateSnippet> = {
  size: (v) => (v.title !== null ? 1 + (4 + wireStringSize(v.title)) : 1) + (v.language !== null ? 1 + (4 + wireStringSize(v.language)) : 1) + (v.code !== null ? 1 + (4 + wireStringSize(v.code)) : 1) + (v.description !== null ? 1 + (4 + wireStringSize(v.description)) : 1) + (v.isPinned !== null ? 1 + 1 : 1),
  encode: (writer, v) => {
    writer.writeOptional(v.title, (v) => { writer.writeString(v) });
    writer.writeOptional(v.language, (v) => { writer.writeString(v) });
    writer.writeOptional(v.code, (v) => { writer.writeString(v) });
    writer.writeOptional(v.description, (v) => { writer.writeString(v) });
    writer.writeOptional(v.isPinned, (v) => { writer.writeBool(v) });
  },
  decode: (reader) => {
    return {
      title: reader.readOptional(() => reader.readString()),
      language: reader.readOptional(() => reader.readString()),
      code: reader.readOptional(() => reader.readString()),
      description: reader.readOptional(() => reader.readString()),
      isPinned: reader.readOptional(() => reader.readBool()),
    };
  },
};

export interface SyncQueueEntry {
  readonly tableName: string;
  readonly recordIdentifier: string;
  readonly operation: string;
  readonly createdAt: string;
}

const SyncQueueEntryCodec: WireCodec<SyncQueueEntry> = {
  size: (v) => (4 + wireStringSize(v.tableName)) + (4 + wireStringSize(v.recordIdentifier)) + (4 + wireStringSize(v.operation)) + (4 + wireStringSize(v.createdAt)),
  encode: (writer, v) => {
    writer.writeString(v.tableName);
    writer.writeString(v.recordIdentifier);
    writer.writeString(v.operation);
    writer.writeString(v.createdAt);
  },
  decode: (reader) => {
    return {
      tableName: reader.readString(),
      recordIdentifier: reader.readString(),
      operation: reader.readString(),
      createdAt: reader.readString(),
    };
  },
};

export interface CreateTodo {
  readonly title: string;
  readonly description: string | null;
  readonly priority: TodoPriority;
}

const CreateTodoCodec: WireCodec<CreateTodo> = {
  size: (v) => (4 + wireStringSize(v.title)) + (v.description !== null ? 1 + (4 + wireStringSize(v.description)) : 1) + 4,
  encode: (writer, v) => {
    writer.writeString(v.title);
    writer.writeOptional(v.description, (v) => { writer.writeString(v) });
    TodoPriorityCodec.encode(writer, v.priority);
  },
  decode: (reader) => {
    return {
      title: reader.readString(),
      description: reader.readOptional(() => reader.readString()),
      priority: TodoPriorityCodec.decode(reader),
    };
  },
};

export interface UpdateTodo {
  readonly title: string | null;
  readonly description: string | null;
}

const UpdateTodoCodec: WireCodec<UpdateTodo> = {
  size: (v) => (v.title !== null ? 1 + (4 + wireStringSize(v.title)) : 1) + (v.description !== null ? 1 + (4 + wireStringSize(v.description)) : 1),
  encode: (writer, v) => {
    writer.writeOptional(v.title, (v) => { writer.writeString(v) });
    writer.writeOptional(v.description, (v) => { writer.writeString(v) });
  },
  decode: (reader) => {
    return {
      title: reader.readOptional(() => reader.readString()),
      description: reader.readOptional(() => reader.readString()),
    };
  },
};

export interface CreateUserPreference {
  readonly firstName: string;
  readonly lastName: string;
  readonly email: string;
}

const CreateUserPreferenceCodec: WireCodec<CreateUserPreference> = {
  size: (v) => (4 + wireStringSize(v.firstName)) + (4 + wireStringSize(v.lastName)) + (4 + wireStringSize(v.email)),
  encode: (writer, v) => {
    writer.writeString(v.firstName);
    writer.writeString(v.lastName);
    writer.writeString(v.email);
  },
  decode: (reader) => {
    return {
      firstName: reader.readString(),
      lastName: reader.readString(),
      email: reader.readString(),
    };
  },
};

export interface UpdateUserPreference {
  readonly firstName: string | null;
  readonly lastName: string | null;
  readonly email: string | null;
}

const UpdateUserPreferenceCodec: WireCodec<UpdateUserPreference> = {
  size: (v) => (v.firstName !== null ? 1 + (4 + wireStringSize(v.firstName)) : 1) + (v.lastName !== null ? 1 + (4 + wireStringSize(v.lastName)) : 1) + (v.email !== null ? 1 + (4 + wireStringSize(v.email)) : 1),
  encode: (writer, v) => {
    writer.writeOptional(v.firstName, (v) => { writer.writeString(v) });
    writer.writeOptional(v.lastName, (v) => { writer.writeString(v) });
    writer.writeOptional(v.email, (v) => { writer.writeString(v) });
  },
  decode: (reader) => {
    return {
      firstName: reader.readOptional(() => reader.readString()),
      lastName: reader.readOptional(() => reader.readString()),
      email: reader.readOptional(() => reader.readString()),
    };
  },
};

export interface CreateWorkspace {
  readonly name: string;
  readonly description: string;
}

const CreateWorkspaceCodec: WireCodec<CreateWorkspace> = {
  size: (v) => (4 + wireStringSize(v.name)) + (4 + wireStringSize(v.description)),
  encode: (writer, v) => {
    writer.writeString(v.name);
    writer.writeString(v.description);
  },
  decode: (reader) => {
    return {
      name: reader.readString(),
      description: reader.readString(),
    };
  },
};

export interface UpdateWorkspace {
  readonly name: string | null;
  readonly description: string | null;
  readonly isDefault: boolean | null;
  readonly isHidden: boolean | null;
  readonly isSecured: boolean | null;
  /**
   * Plain-text password to be hashed; set to Some("") to remove the password.
   */
  readonly password: string | null;
}

const UpdateWorkspaceCodec: WireCodec<UpdateWorkspace> = {
  size: (v) => (v.name !== null ? 1 + (4 + wireStringSize(v.name)) : 1) + (v.description !== null ? 1 + (4 + wireStringSize(v.description)) : 1) + (v.isDefault !== null ? 1 + 1 : 1) + (v.isHidden !== null ? 1 + 1 : 1) + (v.isSecured !== null ? 1 + 1 : 1) + (v.password !== null ? 1 + (4 + wireStringSize(v.password)) : 1),
  encode: (writer, v) => {
    writer.writeOptional(v.name, (v) => { writer.writeString(v) });
    writer.writeOptional(v.description, (v) => { writer.writeString(v) });
    writer.writeOptional(v.isDefault, (v) => { writer.writeBool(v) });
    writer.writeOptional(v.isHidden, (v) => { writer.writeBool(v) });
    writer.writeOptional(v.isSecured, (v) => { writer.writeBool(v) });
    writer.writeOptional(v.password, (v) => { writer.writeString(v) });
  },
  decode: (reader) => {
    return {
      name: reader.readOptional(() => reader.readString()),
      description: reader.readOptional(() => reader.readString()),
      isDefault: reader.readOptional(() => reader.readBool()),
      isHidden: reader.readOptional(() => reader.readBool()),
      isSecured: reader.readOptional(() => reader.readBool()),
      password: reader.readOptional(() => reader.readString()),
    };
  },
};

export interface Model {
  readonly name: string;
  readonly description: string;
  readonly isDefault: boolean;
  readonly isHidden: boolean;
  readonly isSecured: boolean;
  readonly passwordHash: string | null;
}

const ModelCodec: WireCodec<Model> = {
  size: (v) => (4 + wireStringSize(v.name)) + (4 + wireStringSize(v.description)) + 1 + 1 + 1 + (v.passwordHash !== null ? 1 + (4 + wireStringSize(v.passwordHash)) : 1),
  encode: (writer, v) => {
    writer.writeString(v.name);
    writer.writeString(v.description);
    writer.writeBool(v.isDefault);
    writer.writeBool(v.isHidden);
    writer.writeBool(v.isSecured);
    writer.writeOptional(v.passwordHash, (v) => { writer.writeString(v) });
  },
  decode: (reader) => {
    return {
      name: reader.readString(),
      description: reader.readString(),
      isDefault: reader.readBool(),
      isHidden: reader.readBool(),
      isSecured: reader.readBool(),
      passwordHash: reader.readOptional(() => reader.readString()),
    };
  },
};

export enum BookmarkTag {
  Development = 0,
  Inspiration = 1,
  Design = 2,
  Research = 3,
}

const BookmarkTagCodec: WireCodec<BookmarkTag> = {
  size: (_v) => 4,
  encode: (writer, v) => {
    switch (v) {
      case BookmarkTag.Development:
        writer.writeI32(0);
        break;
      case BookmarkTag.Inspiration:
        writer.writeI32(1);
        break;
      case BookmarkTag.Design:
        writer.writeI32(2);
        break;
      case BookmarkTag.Research:
        writer.writeI32(3);
        break;
    }
  },
  decode: (reader) => {
    const value = reader.readI32();
    switch (value) {
      case 0: return BookmarkTag.Development;
      case 1: return BookmarkTag.Inspiration;
      case 2: return BookmarkTag.Design;
      case 3: return BookmarkTag.Research;
      default: throw new Error(`Unknown BookmarkTag wire tag: ${value}`);
    }
  },
};

export enum RecycleBinItemType {
  Todo = 0,
  Note = 1,
  Reminder = 2,
  Snippet = 3,
  Bookmark = 4,
  Workspace = 5,
}

const RecycleBinItemTypeCodec: WireCodec<RecycleBinItemType> = {
  size: (_v) => 4,
  encode: (writer, v) => {
    switch (v) {
      case RecycleBinItemType.Todo:
        writer.writeI32(0);
        break;
      case RecycleBinItemType.Note:
        writer.writeI32(1);
        break;
      case RecycleBinItemType.Reminder:
        writer.writeI32(2);
        break;
      case RecycleBinItemType.Snippet:
        writer.writeI32(3);
        break;
      case RecycleBinItemType.Bookmark:
        writer.writeI32(4);
        break;
      case RecycleBinItemType.Workspace:
        writer.writeI32(5);
        break;
    }
  },
  decode: (reader) => {
    const value = reader.readI32();
    switch (value) {
      case 0: return RecycleBinItemType.Todo;
      case 1: return RecycleBinItemType.Note;
      case 2: return RecycleBinItemType.Reminder;
      case 3: return RecycleBinItemType.Snippet;
      case 4: return RecycleBinItemType.Bookmark;
      case 5: return RecycleBinItemType.Workspace;
      default: throw new Error(`Unknown RecycleBinItemType wire tag: ${value}`);
    }
  },
};

export enum TodoPriority {
  High = 0,
  Medium = 1,
  Low = 2,
}

const TodoPriorityCodec: WireCodec<TodoPriority> = {
  size: (_v) => 4,
  encode: (writer, v) => {
    switch (v) {
      case TodoPriority.High:
        writer.writeI32(0);
        break;
      case TodoPriority.Medium:
        writer.writeI32(1);
        break;
      case TodoPriority.Low:
        writer.writeI32(2);
        break;
    }
  },
  decode: (reader) => {
    const value = reader.readI32();
    switch (value) {
      case 0: return TodoPriority.High;
      case 1: return TodoPriority.Medium;
      case 2: return TodoPriority.Low;
      default: throw new Error(`Unknown TodoPriority wire tag: ${value}`);
    }
  },
};

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

