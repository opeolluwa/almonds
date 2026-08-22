<script lang="ts" setup>
import { useCurrentEditor, useEditorState } from "@domternal/vue";
import type { Editor } from "@domternal/core";

const { editor } = useCurrentEditor();

interface Snapshot {
  bold: boolean;
  italic: boolean;
  underline: boolean;
  strike: boolean;
  code: boolean;
  headingLevel: number | null;
  bulletList: boolean;
  orderedList: boolean;
  blockquote: boolean;
  codeBlock: boolean;
  alignLeft: boolean;
  alignCenter: boolean;
  alignRight: boolean;
  linkHref: string | null;
  canUndo: boolean;
  canRedo: boolean;
  inTable: boolean;
}

const IDLE: Snapshot = {
  bold: false,
  italic: false,
  underline: false,
  strike: false,
  code: false,
  headingLevel: null,
  bulletList: false,
  orderedList: false,
  blockquote: false,
  codeBlock: false,
  alignLeft: false,
  alignCenter: false,
  alignRight: false,
  linkHref: null,
  canUndo: false,
  canRedo: false,
  inTable: false,
};

const snapshot = useEditorState(editor, (ed): Snapshot => ({
  bold: ed.isActive("bold"),
  italic: ed.isActive("italic"),
  underline: ed.isActive("underline"),
  strike: ed.isActive("strike"),
  code: ed.isActive("code"),
  headingLevel:
    ([1, 2, 3] as const).find((l) => ed.isActive("heading", { level: l })) ??
    null,
  bulletList: ed.isActive("bulletList"),
  orderedList: ed.isActive("orderedList"),
  blockquote: ed.isActive("blockquote"),
  codeBlock: ed.isActive("codeBlock"),
  alignLeft: ed.isActive({ textAlign: "left" }),
  alignCenter: ed.isActive({ textAlign: "center" }),
  alignRight: ed.isActive({ textAlign: "right" }),
  linkHref: ed.isActive("link")
    ? String(ed.getAttributes("link").href ?? "")
    : null,
  canUndo: ed.can().undo(),
  canRedo: ed.can().redo(),
  inTable: ed.isActive("table"),
}));

const s = computed<Snapshot>(() => snapshot.value ?? IDLE);

function exec(fn: (ed: Editor) => void) {
  if (editor.value) fn(editor.value);
}

function btnClass(active?: boolean) {
  return [
    "flex size-9 shrink-0 items-center justify-center rounded-lg transition active:scale-95 disabled:pointer-events-none disabled:opacity-30",
    active
      ? "bg-primary-500/10 text-primary-600 dark:bg-primary-500/15 dark:text-primary-400"
      : "text-gray-500 hover:bg-gray-100 hover:text-gray-700 dark:text-gray-400 dark:hover:bg-gray-800 dark:hover:text-gray-300",
  ].join(" ");
}

type HeadingLevel = 1 | 2 | 3;

const headingsOpen = ref(false);
const headingOptions: {
  key: string;
  label: string;
  level: HeadingLevel | null;
  preview: string;
}[] = [
  { key: "p", label: "Paragraph", level: null, preview: "text-sm" },
  {
    key: "h1",
    label: "Heading 1",
    level: 1,
    preview: "text-base font-bold",
  },
  {
    key: "h2",
    label: "Heading 2",
    level: 2,
    preview: "text-sm font-bold",
  },
  {
    key: "h3",
    label: "Heading 3",
    level: 3,
    preview: "text-xs font-bold",
  },
];

function applyHeading(level: HeadingLevel | null) {
  exec((ed) => {
    const chain = ed.chain().focus();
    if (level === null) chain.setParagraph().run();
    else chain.toggleHeading({ level }).run();
  });
  headingsOpen.value = false;
}

const linkOpen = ref(false);
const linkDraft = ref("");

watch(linkOpen, (open) => {
  if (open) linkDraft.value = s.value.linkHref ?? "";
});

function normalizeUrl(raw: string) {
  const t = raw.trim();
  if (!t) return "";
  if (/^[a-zA-Z][a-zA-Z0-9+.-]*:/.test(t) || t.startsWith("/")) return t;
  return `https://${t}`;
}

function applyLink() {
  const href = normalizeUrl(linkDraft.value);
  if (!href) {
    removeLink();
    return;
  }
  exec((ed) => ed.chain().focus().extendMarkRange("link").setLink({ href }).run());
  linkOpen.value = false;
}

function removeLink() {
  exec((ed) => ed.chain().focus().extendMarkRange("link").unsetLink().run());
  linkOpen.value = false;
}

const imageInput = ref<HTMLInputElement | null>(null);

async function handleImagePick(event: Event) {
  const inputEl = event.target as HTMLInputElement;
  const file = inputEl.files?.[0];
  inputEl.value = "";
  if (!file) return;
  try {
    const form = new FormData();
    form.append("file", file);
    const res = await fetch("/api/upload", { method: "POST", body: form });
    if (!res.ok) throw new Error(`Upload failed (${res.status})`);
    const { url } = await res.json();
    exec((ed) => ed.chain().focus().setImage({ src: url }).run());
  } catch (err) {
    console.error("Image upload failed:", err);
  }
}

type Tool =
  | { kind: "sep" }
  | { kind: "headings" }
  | { kind: "link" }
  | {
      kind: "btn";
      icon: string;
      label: string;
      active?: boolean;
      disabled?: boolean;
      action: () => void;
    };

const tools = computed<Tool[]>(() => [
  {
    kind: "btn",
    icon: "ri:arrow-go-back-line",
    label: "Undo",
    disabled: !s.value.canUndo,
    action: () => exec((ed) => ed.chain().focus().undo().run()),
  },
  {
    kind: "btn",
    icon: "ri:arrow-go-forward-line",
    label: "Redo",
    disabled: !s.value.canRedo,
    action: () => exec((ed) => ed.chain().focus().redo().run()),
  },
  { kind: "sep" },
  { kind: "headings" },
  { kind: "sep" },
  {
    kind: "btn",
    icon: "ri:bold",
    label: "Bold",
    active: s.value.bold,
    action: () => exec((ed) => ed.chain().focus().toggleBold().run()),
  },
  {
    kind: "btn",
    icon: "ri:italic",
    label: "Italic",
    active: s.value.italic,
    action: () => exec((ed) => ed.chain().focus().toggleItalic().run()),
  },
  {
    kind: "btn",
    icon: "ri:underline",
    label: "Underline",
    active: s.value.underline,
    action: () => exec((ed) => ed.chain().focus().toggleUnderline().run()),
  },
  {
    kind: "btn",
    icon: "ri:strikethrough",
    label: "Strikethrough",
    active: s.value.strike,
    action: () => exec((ed) => ed.chain().focus().toggleStrike().run()),
  },
  {
    kind: "btn",
    icon: "ri:code-line",
    label: "Inline code",
    active: s.value.code,
    action: () => exec((ed) => ed.chain().focus().toggleCode().run()),
  },
  { kind: "sep" },
  {
    kind: "btn",
    icon: "ri:list-unordered",
    label: "Bullet list",
    active: s.value.bulletList,
    action: () => exec((ed) => ed.chain().focus().toggleBulletList().run()),
  },
  {
    kind: "btn",
    icon: "ri:list-ordered",
    label: "Numbered list",
    active: s.value.orderedList,
    action: () => exec((ed) => ed.chain().focus().toggleOrderedList().run()),
  },
  {
    kind: "btn",
    icon: "ri:double-quotes-l",
    label: "Quote",
    active: s.value.blockquote,
    action: () => exec((ed) => ed.chain().focus().toggleBlockquote().run()),
  },
  {
    kind: "btn",
    icon: "ri:code-box-line",
    label: "Code block",
    active: s.value.codeBlock,
    action: () => exec((ed) => ed.chain().focus().toggleCodeBlock().run()),
  },
  { kind: "sep" },
  {
    kind: "btn",
    icon: "ri:align-left",
    label: "Align left",
    active: s.value.alignLeft,
    action: () => exec((ed) => ed.chain().focus().setTextAlign("left").run()),
  },
  {
    kind: "btn",
    icon: "ri:align-center",
    label: "Align center",
    active: s.value.alignCenter,
    action: () =>
      exec((ed) => ed.chain().focus().setTextAlign("center").run()),
  },
  {
    kind: "btn",
    icon: "ri:align-right",
    label: "Align right",
    active: s.value.alignRight,
    action: () => exec((ed) => ed.chain().focus().setTextAlign("right").run()),
  },
  { kind: "sep" },
  { kind: "link" },
  {
    kind: "btn",
    icon: "ri:image-add-line",
    label: "Insert image",
    action: () => imageInput.value?.click(),
  },
  {
    kind: "btn",
    icon: "ri:table-line",
    label: "Insert table",
    action: () =>
      exec((ed) =>
        ed
          .chain()
          .focus()
          .insertTable({ rows: 3, cols: 3, withHeaderRow: true })
          .run(),
      ),
  },
  {
    kind: "btn",
    icon: "ri:separator",
    label: "Divider",
    action: () => exec((ed) => ed.chain().focus().setHorizontalRule().run()),
  },
  {
    kind: "btn",
    icon: "lucide:chevrons-up-down",
    label: "Collapsible section",
    action: () => exec((ed) => ed.chain().focus().toggleDetails().run()),
  },
]);

const tableOps = computed(() => [
  {
    icon: "ri:insert-row-top",
    label: "Insert row above",
    action: () => exec((ed) => ed.chain().focus().addRowBefore().run()),
  },
  {
    icon: "ri:insert-row-bottom",
    label: "Insert row below",
    action: () => exec((ed) => ed.chain().focus().addRowAfter().run()),
  },
  {
    icon: "ri:delete-row",
    label: "Delete row",
    action: () => exec((ed) => ed.chain().focus().deleteRow().run()),
  },
  {
    icon: "ri:insert-column-left",
    label: "Insert column left",
    action: () => exec((ed) => ed.chain().focus().addColumnBefore().run()),
  },
  {
    icon: "ri:insert-column-right",
    label: "Insert column right",
    action: () => exec((ed) => ed.chain().focus().addColumnAfter().run()),
  },
  {
    icon: "ri:delete-column",
    label: "Delete column",
    action: () => exec((ed) => ed.chain().focus().deleteColumn().run()),
  },
  {
    icon: "ri:heading",
    label: "Toggle header row",
    action: () => exec((ed) => ed.chain().focus().toggleHeaderRow().run()),
  },
  {
    icon: "ri:delete-bin-line",
    label: "Delete table",
    action: () => exec((ed) => ed.chain().focus().deleteTable().run()),
  },
]);
</script>

<template>
  <nav
    class="fixed inset-x-0 bottom-0 z-50 select-none border-t border-gray-200 bg-white dark:border-gray-800 dark:bg-app-dark-900"
    style="padding-bottom: max(0.625rem, env(safe-area-inset-bottom))"
  >
    <div
      v-if="s.inTable"
      class="no-scrollbar flex items-center gap-0.5 overflow-x-auto border-b border-gray-100 px-2 pb-1.5 pt-2 dark:border-gray-800"
    >
      <button
        v-for="op in tableOps"
        :key="op.label"
        type="button"
        :title="op.label"
        :aria-label="op.label"
        class="flex size-8 shrink-0 items-center justify-center rounded-md text-gray-500 transition hover:bg-gray-100 hover:text-gray-700 active:scale-95 dark:text-gray-400 dark:hover:bg-gray-800 dark:hover:text-gray-300"
        @mousedown.prevent
        @click="op.action()"
      >
        <UIcon :name="op.icon" class="size-4.5" />
      </button>
    </div>

    <div class="no-scrollbar flex items-center gap-0.5 overflow-x-auto px-2 pt-2">
      <template v-for="(tool, i) in tools" :key="i">
        <span
          v-if="tool.kind === 'sep'"
          class="mx-1 h-6 w-px shrink-0 bg-gray-200 dark:bg-gray-700"
        />

        <UPopover
          v-else-if="tool.kind === 'headings'"
          v-model:open="headingsOpen"
          :content="{ side: 'top', align: 'start' }"
          :ui="{ content: 'w-44 p-1' }"
        >
          <button
            type="button"
            title="Text style"
            aria-label="Text style"
            :class="btnClass(s.headingLevel !== null)"
            class="min-w-9 px-1"
            @mousedown.prevent
          >
            <span class="text-sm font-semibold">
              {{ s.headingLevel ? `H${s.headingLevel}` : "Aa" }}
            </span>
          </button>
          <template #content>
            <button
              v-for="opt in headingOptions"
              :key="opt.key"
              type="button"
              class="flex w-full items-center justify-between rounded-lg px-2.5 py-2 text-left text-gray-700 transition-colors hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-800"
              @click="applyHeading(opt.level)"
            >
              <span :class="opt.preview">{{ opt.label }}</span>
              <UIcon
                v-if="(s.headingLevel ?? null) === opt.level"
                name="ri:check-line"
                class="size-4 text-primary-500"
              />
            </button>
          </template>
        </UPopover>

        <UPopover
          v-else-if="tool.kind === 'link'"
          v-model:open="linkOpen"
          :content="{ side: 'top', align: 'start' }"
          :ui="{ content: 'p-1.5' }"
        >
          <button
            type="button"
            title="Link"
            aria-label="Link"
            :class="btnClass(s.linkHref !== null)"
            @mousedown.prevent
          >
            <UIcon name="ri:link" class="size-5" />
          </button>
          <template #content>
            <form
              class="flex w-72 max-w-[calc(100vw-2rem)] items-center gap-1"
              @submit.prevent="applyLink"
            >
              <UInput
                v-model="linkDraft"
                placeholder="Paste or type a URL…"
                size="sm"
                class="flex-1"
              />
              <UButton
                type="submit"
                icon="ri:check-line"
                size="sm"
                variant="soft"
                aria-label="Apply link"
              />
              <UButton
                v-if="s.linkHref"
                icon="ri:link-unlink"
                color="error"
                size="sm"
                variant="ghost"
                aria-label="Remove link"
                @click="removeLink"
              />
            </form>
          </template>
        </UPopover>

        <button
          v-else
          type="button"
          :title="tool.label"
          :aria-label="tool.label"
          :aria-pressed="tool.active || undefined"
          :disabled="tool.disabled"
          :class="btnClass(tool.active)"
          @mousedown.prevent
          @click="tool.action()"
        >
          <UIcon :name="tool.icon" class="size-5" />
        </button>
      </template>

      <input
        ref="imageInput"
        type="file"
        accept="image/*"
        class="hidden"
        @change="handleImagePick"
      />
    </div>
  </nav>
</template>

<style scoped>
.no-scrollbar {
  scrollbar-width: none;
}
.no-scrollbar::-webkit-scrollbar {
  display: none;
}
</style>
