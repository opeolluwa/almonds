import 'package:flutter/material.dart';
import 'package:flutter_markdown/flutter_markdown.dart';
import 'package:heroicons/heroicons.dart';

// ── Module-level state ────────────────────────────────────────────────────────

class _Note {
  final String id;
  String title;
  String content;
  DateTime updatedAt;

  _Note({required this.id, required this.title, required this.content, required this.updatedAt});
}

final _notesNotifier = ValueNotifier<List<_Note>>([
  _Note(
    id: '1',
    title: 'Getting started',
    content:
        '# Getting Started\n\nWelcome to **Shurbs Notes** — your markdown-powered notebook.\n\n## Features\n\n- Write in **Markdown**\n- Live **preview** toggle\n- Quick formatting toolbar\n\n## Shortcuts\n\n| Action | Toolbar |\n|--------|--------|\n| Bold | **B** |\n| Italic | *I* |\n| Heading | H |\n| List | • |\n\n> Start writing to see the magic.',
    updatedAt: DateTime.now().subtract(const Duration(hours: 2)),
  ),
  _Note(
    id: '2',
    title: 'Meeting notes',
    content:
        '# Meeting Notes\n\n**Date:** 2026-04-04\n\n## Agenda\n\n1. Project status\n2. Blockers\n3. Next steps\n\n## Action items\n\n- [ ] Follow up with design team\n- [ ] Review PR #42\n- [x] Update documentation',
    updatedAt: DateTime.now().subtract(const Duration(days: 1)),
  ),
]);

int _nextNoteId = 3;

// ── Notes list page ───────────────────────────────────────────────────────────

class NotesPage extends StatefulWidget {
  const NotesPage({super.key});

  @override
  State<NotesPage> createState() => _NotesPageState();
}

class _NotesPageState extends State<NotesPage> {
  String _search = '';

  void _openNote(_Note note) {
    Navigator.push(
      context,
      MaterialPageRoute(builder: (_) => _NoteEditorPage(note: note)),
    ).then((_) => setState(() {}));
  }

  void _createNote() {
    final note = _Note(
      id: '${_nextNoteId++}',
      title: 'Untitled',
      content: '',
      updatedAt: DateTime.now(),
    );
    _notesNotifier.value = [..._notesNotifier.value, note];
    _openNote(note);
  }

  void _deleteNote(_Note note) {
    _notesNotifier.value = _notesNotifier.value.where((n) => n.id != note.id).toList();
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Scaffold(
      body: SafeArea(
        child: Column(
          children: [
            // ── Search bar ─────────────────────────────────────────────────
            Padding(
              padding: const EdgeInsets.fromLTRB(16, 12, 16, 8),
              child: TextField(
                decoration: InputDecoration(
                  hintText: 'Search notes…',
                  prefixIcon: const Padding(
                    padding: EdgeInsets.only(left: 12, right: 8),
                    child: HeroIcon(HeroIcons.magnifyingGlass, size: 18),
                  ),
                  prefixIconConstraints: const BoxConstraints(),
                  filled: true,
                  fillColor: colorScheme.surfaceContainerHighest.withValues(alpha: 0.5),
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(12),
                    borderSide: BorderSide.none,
                  ),
                  contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                ),
                onChanged: (v) => setState(() => _search = v),
              ),
            ),

            // ── List ───────────────────────────────────────────────────────
            Expanded(
              child: ValueListenableBuilder<List<_Note>>(
                valueListenable: _notesNotifier,
                builder: (_, notes, __) {
                  final filtered = _search.trim().isEmpty
                      ? notes
                      : notes
                            .where(
                              (n) =>
                                  n.title.toLowerCase().contains(_search.toLowerCase()) ||
                                  n.content.toLowerCase().contains(_search.toLowerCase()),
                            )
                            .toList();

                  if (filtered.isEmpty) {
                    return Center(
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          HeroIcon(HeroIcons.documentText, size: 48, color: colorScheme.onSurfaceVariant.withValues(alpha: 0.3)),
                          const SizedBox(height: 12),
                          Text(
                            _search.isEmpty ? 'No notes yet' : 'No results',
                            style: theme.textTheme.bodyLarge?.copyWith(color: colorScheme.onSurfaceVariant),
                          ),
                        ],
                      ),
                    );
                  }

                  return ListView.separated(
                    padding: const EdgeInsets.fromLTRB(16, 4, 16, 100),
                    itemCount: filtered.length,
                    separatorBuilder: (_, __) => const SizedBox(height: 8),
                    itemBuilder: (_, i) {
                      final note = filtered[i];
                      return _NoteCard(
                        note: note,
                        onTap: () => _openNote(note),
                        onDelete: () => _deleteNote(note),
                      );
                    },
                  );
                },
              ),
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _createNote,
        child: const HeroIcon(HeroIcons.plus, size: 22),
      ),
    );
  }
}

// ── Note card ─────────────────────────────────────────────────────────────────

class _NoteCard extends StatelessWidget {
  final _Note note;
  final VoidCallback onTap;
  final VoidCallback onDelete;

  const _NoteCard({required this.note, required this.onTap, required this.onDelete});

  String _preview(String content) {
    final stripped = content.replaceAll(RegExp(r'#+\s*'), '').replaceAll(RegExp(r'[*_`>\-]'), '').trim();
    if (stripped.isEmpty) return 'No content';
    return stripped.length > 90 ? '${stripped.substring(0, 90)}…' : stripped;
  }

  String _relativeTime(DateTime dt) {
    final diff = DateTime.now().difference(dt);
    if (diff.inMinutes < 1) return 'just now';
    if (diff.inMinutes < 60) return '${diff.inMinutes}m ago';
    if (diff.inHours < 24) return '${diff.inHours}h ago';
    if (diff.inDays < 7) return '${diff.inDays}d ago';
    return '${dt.year}-${dt.month.toString().padLeft(2, '0')}-${dt.day.toString().padLeft(2, '0')}';
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Card(
      margin: EdgeInsets.zero,
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(12),
        child: Padding(
          padding: const EdgeInsets.fromLTRB(16, 14, 12, 14),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      note.title.isEmpty ? 'Untitled' : note.title,
                      style: theme.textTheme.bodyLarge?.copyWith(fontWeight: FontWeight.w600),
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                    ),
                    const SizedBox(height: 4),
                    Text(
                      _preview(note.content),
                      style: theme.textTheme.bodySmall?.copyWith(
                        color: colorScheme.onSurfaceVariant,
                        height: 1.4,
                      ),
                      maxLines: 2,
                      overflow: TextOverflow.ellipsis,
                    ),
                    const SizedBox(height: 8),
                    Text(
                      _relativeTime(note.updatedAt),
                      style: theme.textTheme.labelSmall?.copyWith(color: colorScheme.onSurfaceVariant.withValues(alpha: 0.6)),
                    ),
                  ],
                ),
              ),
              PopupMenuButton<String>(
                icon: HeroIcon(HeroIcons.ellipsisVertical, size: 18, color: colorScheme.onSurfaceVariant),
                onSelected: (v) {
                  if (v == 'delete') onDelete();
                },
                itemBuilder: (_) => [
                  const PopupMenuItem(value: 'delete', child: Text('Delete')),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}

// ── Note editor page ──────────────────────────────────────────────────────────

class _NoteEditorPage extends StatefulWidget {
  final _Note note;
  const _NoteEditorPage({super.key, required this.note});

  @override
  State<_NoteEditorPage> createState() => _NoteEditorPageState();
}

class _NoteEditorPageState extends State<_NoteEditorPage> {
  late final TextEditingController _titleController;
  late final TextEditingController _contentController;
  bool _previewing = false;

  @override
  void initState() {
    super.initState();
    _titleController = TextEditingController(text: widget.note.title);
    _contentController = TextEditingController(text: widget.note.content);

    _titleController.addListener(_persist);
    _contentController.addListener(_persist);
  }

  @override
  void dispose() {
    _titleController.removeListener(_persist);
    _contentController.removeListener(_persist);
    _titleController.dispose();
    _contentController.dispose();
    super.dispose();
  }

  void _persist() {
    widget.note.title = _titleController.text;
    widget.note.content = _contentController.text;
    widget.note.updatedAt = DateTime.now();
    // Notify listeners so the list updates
    _notesNotifier.value = List.from(_notesNotifier.value);
  }

  // ── Toolbar helpers ──────────────────────────────────────────────────────

  void _wrapSelection(String before, String after) {
    final ctrl = _contentController;
    final sel = ctrl.selection;
    if (!sel.isValid) return;
    final text = ctrl.text;
    final selected = sel.textInside(text);
    final replacement = '$before$selected$after';
    ctrl.value = ctrl.value.replaced(sel, replacement);
    final newOffset = sel.start + before.length + selected.length;
    ctrl.selection = TextSelection.collapsed(offset: newOffset);
  }

  void _insertAtLineStart(String prefix) {
    final ctrl = _contentController;
    final sel = ctrl.selection;
    if (!sel.isValid) return;
    final text = ctrl.text;
    final lineStart = text.lastIndexOf('\n', sel.start - 1) + 1;
    final before = text.substring(0, lineStart);
    final after = text.substring(lineStart);
    final newText = '$before$prefix$after';
    ctrl.value = TextEditingValue(
      text: newText,
      selection: TextSelection.collapsed(offset: sel.start + prefix.length),
    );
  }

  void _insertSnippet(String snippet) {
    final ctrl = _contentController;
    final sel = ctrl.selection;
    if (!sel.isValid) return;
    final text = ctrl.text;
    final newText = text.replaceRange(sel.start, sel.end, snippet);
    ctrl.value = TextEditingValue(
      text: newText,
      selection: TextSelection.collapsed(offset: sel.start + snippet.length),
    );
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Scaffold(
      appBar: AppBar(
        title: TextField(
          controller: _titleController,
          style: theme.textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w600),
          decoration: const InputDecoration(
            hintText: 'Title',
            border: InputBorder.none,
            isDense: true,
          ),
          textInputAction: TextInputAction.next,
        ),
        actions: [
          // Word count
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 4),
            child: ValueListenableBuilder(
              valueListenable: _contentController,
              builder: (_, value, __) {
                final words = value.text.trim().isEmpty
                    ? 0
                    : value.text.trim().split(RegExp(r'\s+')).length;
                return Center(
                  child: Text(
                    '$words w',
                    style: theme.textTheme.labelSmall?.copyWith(color: colorScheme.onSurfaceVariant),
                  ),
                );
              },
            ),
          ),
          // Edit / Preview toggle
          IconButton(
            tooltip: _previewing ? 'Edit' : 'Preview',
            icon: HeroIcon(_previewing ? HeroIcons.pencil : HeroIcons.eye, size: 20),
            onPressed: () => setState(() => _previewing = !_previewing),
          ),
        ],
      ),
      body: Column(
        children: [
          // ── Markdown toolbar (editor only) ─────────────────────────────
          if (!_previewing)
            _MarkdownToolbar(
              onBold: () => _wrapSelection('**', '**'),
              onItalic: () => _wrapSelection('*', '*'),
              onCode: () => _wrapSelection('`', '`'),
              onH1: () => _insertAtLineStart('# '),
              onH2: () => _insertAtLineStart('## '),
              onBullet: () => _insertAtLineStart('- '),
              onQuote: () => _insertAtLineStart('> '),
              onHr: () => _insertSnippet('\n---\n'),
            ),

          // ── Editor / Preview ───────────────────────────────────────────
          Expanded(
            child: _previewing
                ? Markdown(
                    data: _contentController.text.isEmpty ? '*Nothing to preview yet.*' : _contentController.text,
                    padding: const EdgeInsets.fromLTRB(20, 16, 20, 32),
                    styleSheet: MarkdownStyleSheet.fromTheme(theme).copyWith(
                      p: theme.textTheme.bodyMedium?.copyWith(height: 1.6),
                      h1: theme.textTheme.headlineSmall?.copyWith(fontWeight: FontWeight.bold),
                      h2: theme.textTheme.titleLarge?.copyWith(fontWeight: FontWeight.bold),
                      h3: theme.textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w600),
                      code: theme.textTheme.bodyMedium?.copyWith(
                        fontFamily: 'monospace',
                        backgroundColor: colorScheme.surfaceContainerHighest,
                      ),
                      blockquoteDecoration: BoxDecoration(
                        border: Border(left: BorderSide(color: colorScheme.primary, width: 3)),
                        color: colorScheme.primary.withValues(alpha: 0.06),
                      ),
                    ),
                  )
                : TextField(
                    controller: _contentController,
                    expands: true,
                    maxLines: null,
                    minLines: null,
                    keyboardType: TextInputType.multiline,
                    textAlignVertical: TextAlignVertical.top,
                    style: theme.textTheme.bodyMedium?.copyWith(
                      fontFamily: 'monospace',
                      height: 1.7,
                    ),
                    decoration: InputDecoration(
                      hintText: 'Start writing in Markdown…',
                      border: InputBorder.none,
                      contentPadding: const EdgeInsets.fromLTRB(20, 16, 20, 32),
                    ),
                  ),
          ),
        ],
      ),
    );
  }
}

// ── Markdown toolbar ──────────────────────────────────────────────────────────

class _MarkdownToolbar extends StatelessWidget {
  final VoidCallback onBold;
  final VoidCallback onItalic;
  final VoidCallback onCode;
  final VoidCallback onH1;
  final VoidCallback onH2;
  final VoidCallback onBullet;
  final VoidCallback onQuote;
  final VoidCallback onHr;

  const _MarkdownToolbar({
    required this.onBold,
    required this.onItalic,
    required this.onCode,
    required this.onH1,
    required this.onH2,
    required this.onBullet,
    required this.onQuote,
    required this.onHr,
  });

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;

    return Container(
      height: 44,
      decoration: BoxDecoration(
        color: colorScheme.surfaceContainerHighest.withValues(alpha: 0.4),
        border: Border(bottom: BorderSide(color: colorScheme.outlineVariant.withValues(alpha: 0.4))),
      ),
      child: ListView(
        scrollDirection: Axis.horizontal,
        padding: const EdgeInsets.symmetric(horizontal: 8),
        children: [
          _ToolbarBtn(label: 'B', bold: true, onTap: onBold),
          _ToolbarBtn(label: 'I', italic: true, onTap: onItalic),
          _ToolbarBtn(label: '`', mono: true, onTap: onCode),
          const _ToolbarDivider(),
          _ToolbarBtn(label: 'H1', onTap: onH1),
          _ToolbarBtn(label: 'H2', onTap: onH2),
          const _ToolbarDivider(),
          _ToolbarBtn(label: '•', onTap: onBullet),
          _ToolbarBtn(label: '❝', onTap: onQuote),
          _ToolbarBtn(label: '—', onTap: onHr),
        ],
      ),
    );
  }
}

class _ToolbarBtn extends StatelessWidget {
  final String label;
  final VoidCallback onTap;
  final bool bold;
  final bool italic;
  final bool mono;

  const _ToolbarBtn({
    required this.label,
    required this.onTap,
    this.bold = false,
    this.italic = false,
    this.mono = false,
  });

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(6),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 12),
        alignment: Alignment.center,
        child: Text(
          label,
          style: TextStyle(
            fontWeight: bold ? FontWeight.bold : FontWeight.w500,
            fontStyle: italic ? FontStyle.italic : FontStyle.normal,
            fontFamily: mono ? 'monospace' : null,
            fontSize: 14,
            color: colorScheme.onSurface,
          ),
        ),
      ),
    );
  }
}

class _ToolbarDivider extends StatelessWidget {
  const _ToolbarDivider();

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 10, horizontal: 2),
      child: VerticalDivider(width: 1, color: Theme.of(context).colorScheme.outlineVariant),
    );
  }
}
