import type { EditorCustomHandlers, EditorMentionMenuItem, EditorSuggestionMenuItem, EditorToolbarItem } from "@nuxt/ui";


export const suggestionItems = [
  [
    {
      type: "label",
      label: "AI",
    },
    // {
    //   kind: "aiContinue",
    //   label: "Continue writing",
    //   icon: "i-lucide-sparkles",
    // },
  ],
  [
    {
      type: "label",
      label: "Style",
    },
    {
      kind: "paragraph",
      label: "Paragraph",
      icon: "i-lucide-type",
    },
    {
      kind: "heading",
      level: 1,
      label: "Heading 1",
      icon: "i-lucide-heading-1",
    },
    {
      kind: "heading",
      level: 2,
      label: "Heading 2",
      icon: "i-lucide-heading-2",
    },
    {
      kind: "heading",
      level: 3,
      label: "Heading 3",
      icon: "i-lucide-heading-3",
    },
    {
      kind: "bulletList",
      label: "Bullet List",
      icon: "i-lucide-list",
    },
    {
      kind: "orderedList",
      label: "Numbered List",
      icon: "i-lucide-list-ordered",
    },
    {
      kind: "blockquote",
      label: "Blockquote",
      icon: "i-lucide-text-quote",
    },
    {
      kind: "codeBlock",
      label: "Code Block",
      icon: "i-lucide-square-code",
    },
  ],
  [
    {
      type: "label",
      label: "Insert",
    },
    {
      kind: "mention",
      label: "Mention",
      icon: "i-lucide-at-sign",
    },
    {
      kind: "emoji",
      label: "Emoji",
      icon: "i-lucide-smile-plus",
    },
    {
      kind: "imageUpload",
      label: "Image",
      icon: "i-lucide-image",
    },
    {
      kind: "horizontalRule",
      label: "Horizontal Rule",
      icon: "i-lucide-separator-horizontal",
    },
  ],
] satisfies EditorSuggestionMenuItem<typeof customHandlers>[][];

export const mentionItems: EditorMentionMenuItem[] = [
  {
    label: "benjamincanac",
    avatar: { src: "https://avatars.githubusercontent.com/u/739984?v=4" },
  },
  {
    label: "HugoRCD",
    avatar: { src: "https://avatars.githubusercontent.com/u/71938701?v=4" },
  },
  {
    label: "romhml",
    avatar: { src: "https://avatars.githubusercontent.com/u/25613751?v=4" },
  },
  {
    label: "sandros94",
    avatar: { src: "https://avatars.githubusercontent.com/u/13056429?v=4" },
  },
  {
    label: "hywax",
    avatar: { src: "https://avatars.githubusercontent.com/u/149865959?v=4" },
  },
  {
    label: "J-Michalek",
    avatar: { src: "https://avatars.githubusercontent.com/u/71264422?v=4" },
  },
  {
    label: "genu",
    avatar: { src: "https://avatars.githubusercontent.com/u/928780?v=4" },
  },
];

export const customHandlers = {
  imageUpload: {
    canExecute: (editor) =>
      editor.can().insertContent({ type: "imageUpload" }),
    execute: (editor) =>
      editor.chain().focus().insertContent({ type: "imageUpload" }),
    isActive: (editor) => editor.isActive("imageUpload"),
    isDisabled: undefined,
  },
//   ...aiHandlers,
} satisfies EditorCustomHandlers;


export const fixedToolbarItems = [
  [
    {
      kind: "undo",
      icon: "i-lucide-undo",
      tooltip: { text: "Undo" },
    },
    {
      kind: "redo",
      icon: "i-lucide-redo",
      tooltip: { text: "Redo" },
    },
  ],
  [
    {
      icon: "i-lucide-heading",
      tooltip: { text: "Headings" },
      content: {
        align: "start",
      },
      items: [
        {
          kind: "heading",
          level: 1,
          icon: "i-lucide-heading-1",
          label: "Heading 1",
        },
        {
          kind: "heading",
          level: 2,
          icon: "i-lucide-heading-2",
          label: "Heading 2",
        },
        {
          kind: "heading",
          level: 3,
          icon: "i-lucide-heading-3",
          label: "Heading 3",
        },
        {
          kind: "heading",
          level: 4,
          icon: "i-lucide-heading-4",
          label: "Heading 4",
        },
      ],
    },
    {
      icon: "i-lucide-list",
      tooltip: { text: "Lists" },
      content: {
        align: "start",
      },
      items: [
        {
          kind: "bulletList",
          icon: "i-lucide-list",
          label: "Bullet List",
        },
        {
          kind: "orderedList",
          icon: "i-lucide-list-ordered",
          label: "Ordered List",
        },
      ],
    },
    {
      kind: "blockquote",
      icon: "i-lucide-text-quote",
      tooltip: { text: "Blockquote" },
    },
    {
      kind: "codeBlock",
      icon: "i-lucide-square-code",
      tooltip: { text: "Code Block" },
    },
  ],
  [
    {
      kind: "mark",
      mark: "bold",
      icon: "i-lucide-bold",
      tooltip: { text: "Bold" },
    },
    {
      kind: "mark",
      mark: "italic",
      icon: "i-lucide-italic",
      tooltip: { text: "Italic" },
    },
    {
      kind: "mark",
      mark: "underline",
      icon: "i-lucide-underline",
      tooltip: { text: "Underline" },
    },
    {
      kind: "mark",
      mark: "strike",
      icon: "i-lucide-strikethrough",
      tooltip: { text: "Strikethrough" },
    },
    {
      kind: "mark",
      mark: "code",
      icon: "i-lucide-code",
      tooltip: { text: "Code" },
    },
  ],
  [
    {
      slot: "link" as const,
      icon: "i-lucide-link",
    },
    {
      kind: "imageUpload",
      icon: "i-lucide-image",
      tooltip: { text: "Image" },
    },
  ],
  [
    {
      icon: "i-lucide-align-justify",
      tooltip: { text: "Text Align" },
      content: {
        align: "end",
      },
      items: [
        {
          kind: "textAlign",
          align: "left",
          icon: "i-lucide-align-left",
          label: "Align Left",
        },
        {
          kind: "textAlign",
          align: "center",
          icon: "i-lucide-align-center",
          label: "Align Center",
        },
        {
          kind: "textAlign",
          align: "right",
          icon: "i-lucide-align-right",
          label: "Align Right",
        },
        {
          kind: "textAlign",
          align: "justify",
          icon: "i-lucide-align-justify",
          label: "Align Justify",
        },
      ],
    },
  ],
] satisfies EditorToolbarItem<typeof customHandlers>[][];

export const bubbleToolbarItems = computed(
  () =>
    [
    //   [
    //     {
    //       icon: "i-lucide-sparkles",
    //       label: "Improve",
    //       activeColor: "neutral",
    //       activeVariant: "ghost",
    //       loading: aiLoading.value,
    //       content: {
    //         align: "start",
    //       },
    //       items: [
    //         {
    //           kind: "aiFix",
    //           icon: "i-lucide-spell-check",
    //           label: "Fix spelling & grammar",
    //         },
    //         {
    //           kind: "aiExtend",
    //           icon: "i-lucide-unfold-vertical",
    //           label: "Extend text",
    //         },
    //         {
    //           kind: "aiReduce",
    //           icon: "i-lucide-fold-vertical",
    //           label: "Reduce text",
    //         },
    //         {
    //           kind: "aiSimplify",
    //           icon: "i-lucide-lightbulb",
    //           label: "Simplify text",
    //         },
    //         {
    //           kind: "aiContinue",
    //           icon: "i-lucide-text",
    //           label: "Continue sentence",
    //         },
    //         {
    //           kind: "aiSummarize",
    //           icon: "i-lucide-list",
    //           label: "Summarize",
    //         },
    //         {
    //           icon: "i-lucide-languages",
    //           label: "Translate",
    //           children: [
    //             {
    //               kind: "aiTranslate",
    //               language: "English",
    //               label: "English",
    //             },
    //             {
    //               kind: "aiTranslate",
    //               language: "French",
    //               label: "French",
    //             },
    //             {
    //               kind: "aiTranslate",
    //               language: "Spanish",
    //               label: "Spanish",
    //             },
    //             {
    //               kind: "aiTranslate",
    //               language: "German",
    //               label: "German",
    //             },
    //           ],
    //         },
    //       ],
    //     },
    //   ],
      [
        {
          label: "Turn into",
          trailingIcon: "i-lucide-chevron-down",
          activeColor: "neutral",
          activeVariant: "ghost",
          tooltip: { text: "Turn into" },
          content: {
            align: "start",
          },
          ui: {
            label: "text-xs",
          },
          items: [
            {
              type: "label",
              label: "Turn into",
            },
            {
              kind: "paragraph",
              label: "Paragraph",
              icon: "i-lucide-type",
            },
            {
              kind: "heading",
              level: 1,
              icon: "i-lucide-heading-1",
              label: "Heading 1",
            },
            {
              kind: "heading",
              level: 2,
              icon: "i-lucide-heading-2",
              label: "Heading 2",
            },
            {
              kind: "heading",
              level: 3,
              icon: "i-lucide-heading-3",
              label: "Heading 3",
            },
            {
              kind: "heading",
              level: 4,
              icon: "i-lucide-heading-4",
              label: "Heading 4",
            },
            {
              kind: "bulletList",
              icon: "i-lucide-list",
              label: "Bullet List",
            },
            {
              kind: "orderedList",
              icon: "i-lucide-list-ordered",
              label: "Ordered List",
            },
            {
              kind: "blockquote",
              icon: "i-lucide-text-quote",
              label: "Blockquote",
            },
            {
              kind: "codeBlock",
              icon: "i-lucide-square-code",
              label: "Code Block",
            },
          ],
        },
      ],
      [
        {
          kind: "mark",
          mark: "bold",
          icon: "i-lucide-bold",
          tooltip: { text: "Bold" },
        },
        {
          kind: "mark",
          mark: "italic",
          icon: "i-lucide-italic",
          tooltip: { text: "Italic" },
        },
        {
          kind: "mark",
          mark: "underline",
          icon: "i-lucide-underline",
          tooltip: { text: "Underline" },
        },
        {
          kind: "mark",
          mark: "strike",
          icon: "i-lucide-strikethrough",
          tooltip: { text: "Strikethrough" },
        },
        {
          kind: "mark",
          mark: "code",
          icon: "i-lucide-code",
          tooltip: { text: "Code" },
        },
      ],
      [
        {
          slot: "link" as const,
          icon: "i-lucide-link",
        },
        {
          kind: "imageUpload",
          icon: "i-lucide-image",
          tooltip: { text: "Image" },
        },
      ],
      [
        {
          icon: "i-lucide-align-justify",
          tooltip: { text: "Text Align" },
          content: {
            align: "end",
          },
          items: [
            {
              kind: "textAlign",
              align: "left",
              icon: "i-lucide-align-left",
              label: "Align Left",
            },
            {
              kind: "textAlign",
              align: "center",
              icon: "i-lucide-align-center",
              label: "Align Center",
            },
            {
              kind: "textAlign",
              align: "right",
              icon: "i-lucide-align-right",
              label: "Align Right",
            },
            {
              kind: "textAlign",
              align: "justify",
              icon: "i-lucide-align-justify",
              label: "Align Justify",
            },
          ],
        },
      ],
    ] satisfies EditorToolbarItem<typeof customHandlers>[][],
);