import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:heroicons/heroicons.dart';

import '../rust/api/bookmarks.dart';

class _Bookmark {
  final String id;
  final String title;
  final String url;
  final String? tag;

  _Bookmark({required this.id, required this.title, required this.url, this.tag});

  factory _Bookmark.fromJson(Map<String, dynamic> j) => _Bookmark(
        id: j['identifier'] as String,
        title: j['title'] as String,
        url: j['url'] as String,
        tag: j['tag'] as String?,
      );
}

class BookmarksPage extends StatefulWidget {
  const BookmarksPage({super.key});

  @override
  State<BookmarksPage> createState() => _BookmarksPageState();
}

class _BookmarksPageState extends State<BookmarksPage> {
  List<_Bookmark> _bookmarks = [];
  bool _loading = true;
  String _search = '';
  String? _activeTag;

  static const _validTags = ['development', 'inspiration', 'design', 'research'];

  @override
  void initState() {
    super.initState();
    _loadBookmarks();
  }

  Future<void> _loadBookmarks() async {
    try {
      final raw = await getAllBookmarks();
      final list = (jsonDecode(raw) as List).cast<Map<String, dynamic>>();
      setState(() {
        _bookmarks = list.map(_Bookmark.fromJson).toList();
        _loading = false;
      });
    } catch (_) {
      setState(() => _loading = false);
    }
  }

  List<String> get _tags {
    final tags = _bookmarks.map((b) => b.tag).whereType<String>().toSet().toList();
    tags.sort();
    return tags;
  }

  List<_Bookmark> get _filtered {
    return _bookmarks.where((b) {
      final matchesSearch = _search.isEmpty ||
          b.title.toLowerCase().contains(_search.toLowerCase()) ||
          b.url.toLowerCase().contains(_search.toLowerCase());
      final matchesTag = _activeTag == null || b.tag == _activeTag;
      return matchesSearch && matchesTag;
    }).toList();
  }

  Future<void> _deleteBookmark(_Bookmark b) async {
    try {
      await deleteBookmark(identifier: b.id);
      setState(() => _bookmarks.removeWhere((x) => x.id == b.id));
    } catch (_) {}
  }

  void _addBookmark() {
    final titleController = TextEditingController();
    final urlController = TextEditingController();
    String selectedTag = 'development';

    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (ctx) => Padding(
        padding: EdgeInsets.only(
          left: 24,
          right: 24,
          top: 24,
          bottom: MediaQuery.of(ctx).viewInsets.bottom + 24,
        ),
        child: StatefulBuilder(
          builder: (ctx, setModalState) => Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text('New Bookmark', style: Theme.of(ctx).textTheme.titleLarge),
              const SizedBox(height: 16),
              TextField(
                controller: titleController,
                autofocus: true,
                decoration: const InputDecoration(hintText: 'Title', border: OutlineInputBorder()),
              ),
              const SizedBox(height: 10),
              TextField(
                controller: urlController,
                keyboardType: TextInputType.url,
                decoration: const InputDecoration(hintText: 'URL', border: OutlineInputBorder()),
              ),
              const SizedBox(height: 10),
              DropdownButtonFormField<String>(
                value: selectedTag,
                decoration: const InputDecoration(labelText: 'Tag', border: OutlineInputBorder()),
                items: _validTags
                    .map((t) => DropdownMenuItem(value: t, child: Text(t)))
                    .toList(),
                onChanged: (v) => setModalState(() => selectedTag = v ?? 'development'),
              ),
              const SizedBox(height: 16),
              SizedBox(
                width: double.infinity,
                child: FilledButton(
                  onPressed: () async {
                    if (titleController.text.trim().isNotEmpty && urlController.text.trim().isNotEmpty) {
                      Navigator.pop(ctx);
                      try {
                        final raw = await createBookmark(
                          title: titleController.text.trim(),
                          url: urlController.text.trim(),
                          tag: selectedTag,
                        );
                        final json = jsonDecode(raw) as Map<String, dynamic>;
                        setState(() => _bookmarks.insert(0, _Bookmark.fromJson(json)));
                      } catch (_) {}
                    }
                  },
                  child: const Text('Save Bookmark'),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;

    if (_loading) {
      return const Scaffold(body: Center(child: CircularProgressIndicator()));
    }

    final filtered = _filtered;

    return Scaffold(
      body: SafeArea(
        child: CustomScrollView(
          slivers: [
            SliverPadding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              sliver: SliverToBoxAdapter(
                child: Column(
                  children: [
                    SingleChildScrollView(
                      scrollDirection: Axis.horizontal,
                      child: Row(
                        children: [
                          FilterChip(
                            label: const Text('All'),
                            selected: _activeTag == null,
                            onSelected: (_) => setState(() => _activeTag = null),
                          ),
                          const SizedBox(width: 8),
                          ..._tags.map((tag) => Padding(
                                padding: const EdgeInsets.only(right: 8),
                                child: FilterChip(
                                  label: Text(tag),
                                  selected: _activeTag == tag,
                                  onSelected: (_) =>
                                      setState(() => _activeTag = _activeTag == tag ? null : tag),
                                ),
                              )),
                        ],
                      ),
                    ),
                    const SizedBox(height: 8),
                    TextField(
                      decoration: InputDecoration(
                        hintText: 'Search bookmarks…',
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
                    const SizedBox(height: 8),
                  ],
                ),
              ),
            ),
            filtered.isEmpty
                ? SliverFillRemaining(
                    child: Center(
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          HeroIcon(HeroIcons.bookmark, size: 64, color: colorScheme.outlineVariant),
                          const SizedBox(height: 12),
                          const Text('No bookmarks found'),
                        ],
                      ),
                    ),
                  )
                : SliverPadding(
                    padding: const EdgeInsets.symmetric(horizontal: 16),
                    sliver: SliverList(
                      delegate: SliverChildBuilderDelegate(
                        (ctx, i) => _BookmarkTile(
                          bookmark: filtered[i],
                          onDelete: () => _deleteBookmark(filtered[i]),
                        ),
                        childCount: filtered.length,
                      ),
                    ),
                  ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _addBookmark,
        child: const HeroIcon(HeroIcons.plus),
      ),
    );
  }
}

class _BookmarkTile extends StatelessWidget {
  final _Bookmark bookmark;
  final VoidCallback onDelete;

  const _BookmarkTile({required this.bookmark, required this.onDelete});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Card(
      margin: const EdgeInsets.only(bottom: 8),
      child: ListTile(
        leading: CircleAvatar(
          backgroundColor: theme.colorScheme.secondaryContainer,
          child: Text(
            bookmark.title[0].toUpperCase(),
            style: TextStyle(color: theme.colorScheme.onSecondaryContainer, fontWeight: FontWeight.bold),
          ),
        ),
        title: Text(bookmark.title, maxLines: 1, overflow: TextOverflow.ellipsis),
        subtitle: Text(bookmark.url, maxLines: 1, overflow: TextOverflow.ellipsis, style: theme.textTheme.bodySmall),
        trailing: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            if (bookmark.tag != null)
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
                decoration: BoxDecoration(
                  color: theme.colorScheme.primaryContainer,
                  borderRadius: BorderRadius.circular(12),
                ),
                child: Text(bookmark.tag!, style: TextStyle(fontSize: 11, color: theme.colorScheme.onPrimaryContainer)),
              ),
            IconButton(
              icon: const HeroIcon(HeroIcons.documentDuplicate, size: 20),
              onPressed: () {
                Clipboard.setData(ClipboardData(text: bookmark.url));
                ScaffoldMessenger.of(context).showSnackBar(
                  const SnackBar(content: Text('URL copied'), duration: Duration(seconds: 2)),
                );
              },
            ),
            IconButton(
              icon: const HeroIcon(HeroIcons.trash, size: 20),
              onPressed: onDelete,
            ),
          ],
        ),
      ),
    );
  }
}
