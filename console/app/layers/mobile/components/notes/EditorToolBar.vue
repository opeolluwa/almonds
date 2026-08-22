<script lang="ts" setup>
import { useCurrentEditor, useEditorState } from "@domternal/vue";
import { getMarkRange, type Command, type Editor } from "@domternal/core";
import { TextSelection } from "@domternal/pm/state";
import { emojis } from "@domternal/extension-emoji";

const { editor } = useCurrentEditor();

interface Snapshot {
  bold: boolean;
  italic: boolean;
  underline: boolean;
  strike: boolean;
  code: boolean;
  superscript: boolean;
  subscript: boolean;
  headingLevel: number | null;
  bulletList: boolean;
  orderedList: boolean;
  blockquote: boolean;
  codeBlock: boolean;
  alignLeft: boolean;
  alignCenter: boolean;
  alignRight: boolean;
  linkHref: string | null;
  color: string | null;
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
  superscript: false,
  subscript: false,
  headingLevel: null,
  bulletList: false,
  orderedList: false,
  blockquote: false,
  codeBlock: false,
  alignLeft: false,
  alignCenter: false,
  alignRight: false,
  linkHref: null,
  color: null,
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
  superscript: ed.isActive("superscript"),
  subscript: ed.isActive("subscript"),
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
  color: (ed.getAttributes("textStyle").color as string | undefined) ?? null,
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
const savedLinkSelection = ref<{ from: number; to: number } | null>(null);

watch(linkOpen, (open) => {
  if (open) linkDraft.value = s.value.linkHref ?? "";
});

function onLinkTriggerMousedown(event: MouseEvent) {
  const ed = editor.value;
  savedLinkSelection.value = ed
    ? { from: ed.state.selection.from, to: ed.state.selection.to }
    : null;
  event.preventDefault();
}

function normalizeUrl(raw: string) {
  const t = raw.trim();
  if (!t) return "";
  if (/^[a-zA-Z][a-zA-Z0-9+.-]*:/.test(t) || t.startsWith("/")) return t;
  return `https://${t}`;
}

// domternal's command registry lacks `setTextSelection`/`extendMarkRange`,
// which would silently abort any chain using them. This custom command
// restores the selection and widens it across an adjacent/existing link.
const selectAndExtendLink = (from: number, to: number): Command =>
  ({ tr, dispatch }) => {
    const size = tr.doc.content.size;
    let start = Math.max(0, Math.min(from, size));
    let end = Math.max(start, Math.min(to, size));
    const linkType = tr.doc.type.schema.marks.link;
    if (linkType) {
      for (const pos of [start, end]) {
        const range = getMarkRange(tr.doc.resolve(pos), linkType);
        if (range) {
          start = Math.min(start, range.from);
          end = Math.max(end, range.to);
        }
      }
    }
    if (dispatch) {
      dispatch(tr.setSelection(TextSelection.create(tr.doc, start, end)));
    }
    return true;
  };

function applyLink() {
  const href = normalizeUrl(linkDraft.value);
  exec((ed) => {
    const chain = ed.chain().focus();
    const sel = savedLinkSelection.value;
    if (sel && sel.to <= ed.state.doc.content.size) {
      chain.command(selectAndExtendLink(sel.from, sel.to));
    }
    if (!href) chain.unsetLink().run();
    else chain.setLink({ href }).run();
  });
  linkOpen.value = false;
}

function removeLink() {
  exec((ed) => {
    const chain = ed.chain().focus();
    const sel = savedLinkSelection.value;
    if (sel && sel.to <= ed.state.doc.content.size) {
      chain.command(selectAndExtendLink(sel.from, sel.to));
    }
    chain.unsetLink().run();
  });
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

const emojiOpen = ref(false);
const emojiQuery = ref("");

const emojiGroups = computed(() => {
  const q = emojiQuery.value.trim().toLowerCase();
  const source = q
    ? emojis.filter(
        (e) => e.name.includes(q) || e.emoji.includes(q) || e.group.toLowerCase().includes(q),
      )
    : emojis;
  const map = new Map<string, typeof emojis>();
  for (const item of source.slice(0, 120)) {
    const list = map.get(item.group) ?? [];
    list.push(item);
    map.set(item.group, list);
  }
  return [...map.entries()];
});

function insertEmoji(name: string) {
  exec((ed) => ed.chain().focus().insertEmoji(name).run());
  emojiOpen.value = false;
  emojiQuery.value = "";
}

const colorOpen = ref(false);

const colorPalette = computed<string[]>(() => {
  const ext = editor.value?.extensionManager.extensions.find(
    (e) => e.name === "textColor",
  );
  const colors = (
    ext?.options as { colors?: string[] } | undefined
  )?.colors;
  return colors ?? [];
});

function applyColor(color: string | null) {
  exec((ed) => {
    const chain = ed.chain().focus();
    if (color === null) chain.unsetTextColor().run();
    else chain.setTextColor(color).run();
  });
  colorOpen.value = false;
}

type Tool =
  | { kind: "sep" }
  | { kind: "headings" }
  | { kind: "link" }
  | { kind: "color" }
  | { kind: "emoji" }
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
  {
    kind: "btn",
    icon: "ri:superscript",
    label: "Superscript",
    active: s.value.superscript,
    action: () => exec((ed) => ed.chain().focus().toggleSuperscript().run()),
  },
  {
    kind: "btn",
    icon: "ri:subscript",
    label: "Subscript",
    active: s.value.subscript,
    action: () => exec((ed) => ed.chain().focus().toggleSubscript().run()),
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
  { kind: "color" },
  { kind: "emoji" },
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
    icon: "lucide:sigma",
    label: "Inline equation",
    action: () => exec((ed) => ed.chain().focus().insertMathInline().run()),
  },
  {
    kind: "btn",
    icon: "lucide:square-sigma",
    label: "Equation block",
    action: () => exec((ed) => ed.chain().focus().insertMathBlock().run()),
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
    class="fixed inset-x-0 z-50 select-none border-t border-gray-200 bg-white dark:border-gray-800 dark:bg-app-dark-900"
    style="
      bottom: var(--kb-inset, 0px);
      padding-bottom: max(
        0.625rem,
        calc(env(safe-area-inset-bottom) - var(--kb-inset, 0px))
      );
    "
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
            @mousedown="onLinkTriggerMousedown"
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

        <UPopover
          v-else-if="tool.kind === 'color'"
          v-model:open="colorOpen"
          :content="{ side: 'top', align: 'start' }"
          :ui="{ content: 'p-2' }"
        >
          <button
            type="button"
            title="Text color"
            aria-label="Text color"
            :class="btnClass(s.color !== null)"
            @mousedown.prevent
          >
            <span class="flex flex-col items-center leading-none">
              <UIcon name="ri:font-color" class="size-5" />
              <span
                class="mt-0.5 block h-0.5 w-4 rounded-full bg-current"
                :style="s.color ? { backgroundColor: s.color } : undefined"
              />
            </span>
          </button>
          <template #content>
            <div class="grid w-44 grid-cols-5 gap-1">
              <button
                v-for="c in colorPalette"
                :key="c"
                type="button"
                :title="c"
                :aria-label="`Color ${c}`"
                class="flex size-7 items-center justify-center rounded-md transition hover:bg-gray-100 dark:hover:bg-gray-800"
                @click="applyColor(c)"
              >
                <span
                  class="size-5 rounded-full border border-gray-300 dark:border-gray-600"
                  :style="{ backgroundColor: c }"
                />
              </button>
            </div>
            <div class="mt-2 border-t border-gray-100 pt-1.5 dark:border-gray-800">
              <button
                type="button"
                class="flex w-full items-center gap-1.5 rounded-md px-1 py-1 text-xs text-gray-500 transition-colors hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800"
                @click="applyColor(null)"
              >
                <UIcon name="ri:close-circle-line" class="size-3.5" />
                Default
              </button>
            </div>
          </template>
        </UPopover>

        <UPopover
          v-else-if="tool.kind === 'emoji'"
          v-model:open="emojiOpen"
          :content="{ side: 'top', align: 'start' }"
          :ui="{ content: 'p-2' }"
        >
          <button
            type="button"
            title="Emoji"
            aria-label="Emoji"
            :class="btnClass(false)"
            @mousedown.prevent
          >
            <UIcon name="ri:emotion-happy-line" class="size-5" />
          </button>
          <template #content>
            <div class="w-64 max-w-[calc(100vw-2rem)]">
              <UInput
                v-model="emojiQuery"
                icon="ri:search-line"
                placeholder="Search emoji…"
                size="sm"
                class="w-full"
              />
              <div
                class="mt-2 max-h-52 overflow-y-auto overscroll-contain pr-0.5"
              >
                <template
                  v-for="[group, items] in emojiGroups"
                  :key="group"
                >
                  <p
                    class="px-1 pb-1 pt-2 text-[10px] font-semibold uppercase tracking-wide text-gray-400 dark:text-gray-500"
                  >
                    {{ group }}
                  </p>
                  <div class="grid grid-cols-8 gap-0.5">
                    <button
                      v-for="item in items"
                      :key="item.name"
                      type="button"
                      :title="item.name.replaceAll('_', ' ')"
                      :aria-label="item.name.replaceAll('_', ' ')"
                      class="flex size-7 items-center justify-center rounded-md text-lg leading-none transition hover:bg-gray-100 dark:hover:bg-gray-800"
                      @click="insertEmoji(item.name)"
                    >
                      {{ item.emoji }}
                    </button>
                  </div>
                </template>
                <p
                  v-if="emojiGroups.length === 0"
                  class="py-4 text-center text-xs text-gray-400"
                >
                  No emoji found
                </p>
              </div>
            </div>
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
