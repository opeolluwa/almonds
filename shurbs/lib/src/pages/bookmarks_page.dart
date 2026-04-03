import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

class BookmarksPage extends StatefulWidget {
  const BookmarksPage({super.key});

  @override
  State<BookmarksPage> createState() => _BookmarksPageState();
}

class _BookmarksPageState extends State<BookmarksPage> {
  final List<_Bookmark> _bookmarks = [
    _Bookmark(id: 1, title: 'Flutter Documentation', url: 'https://flutter.dev/docs', tag: 'dev'),
    _Bookmark(id: 2, title: 'Dart Language Tour', url: 'https://dart.dev/guides', tag: 'dev'),
    _Bookmark(id: 3, title: 'Material Design 3', url: 'https://m3.material.io', tag: 'design'),
    _Bookmark(id: 4, title: 'pub.dev packages', url: 'https://pub.dev', tag: 'dev'),
  ];

  String _search = '';
  String? _activeTag;

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

  void _addBookmark() {
    final titleController = TextEditingController();
    final urlController = TextEditingController();
    final tagController = TextEditingController();

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
        child: Column(
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
            TextField(
              controller: tagController,
              decoration: const InputDecoration(hintText: 'Tag (optional)', border: OutlineInputBorder()),
            ),
            const SizedBox(height: 16),
            SizedBox(
              width: double.infinity,
              child: FilledButton(
                onPressed: () {
                  if (titleController.text.trim().isNotEmpty && urlController.text.trim().isNotEmpty) {
                    setState(() {
                      _bookmarks.insert(
                        0,
                        _Bookmark(
                          id: DateTime.now().millisecondsSinceEpoch,
                          title: titleController.text.trim(),
                          url: urlController.text.trim(),
                          tag: tagController.text.trim().isEmpty ? null : tagController.text.trim(),
                        ),
                      );
                    });
                    Navigator.pop(ctx);
                  }
                },
                child: const Text('Save Bookmark'),
              ),
            ),
          ],
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;

    return Scaffold(
      body: SafeArea(
        child: CustomScrollView(
          slivers: [
            const SliverAppBar.large(title: Text('Bookmarks')),
            SliverPadding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              sliver: SliverToBoxAdapter(
                child: Column(
                  children: [
                    SearchBar(
                      hintText: 'Search bookmarks…',
                      leading: const Icon(Icons.search),
                      onChanged: (v) => setState(() => _search = v),
                    ),
                    const SizedBox(height: 12),
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
                                  onSelected: (_) => setState(() => _activeTag = _activeTag == tag ? null : tag),
                                ),
                              )),
                        ],
                      ),
                    ),
                    const SizedBox(height: 8),
                  ],
                ),
              ),
            ),
            _filtered.isEmpty
                ? SliverFillRemaining(
                    child: Center(
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          Icon(Icons.bookmark_border, size: 64, color: colorScheme.outlineVariant),
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
                          bookmark: _filtered[i],
                          onDelete: () => setState(() => _bookmarks.remove(_filtered[i])),
                        ),
                        childCount: _filtered.length,
                      ),
                    ),
                  ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _addBookmark,
        child: const Icon(Icons.add),
      ),
    );
  }
}

class _Bookmark {
  final int id;
  final String title;
  final String url;
  final String? tag;

  _Bookmark({required this.id, required this.title, required this.url, this.tag});
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
              icon: const Icon(Icons.copy, size: 18),
              onPressed: () {
                Clipboard.setData(ClipboardData(text: bookmark.url));
                ScaffoldMessenger.of(context).showSnackBar(
                  const SnackBar(content: Text('URL copied'), duration: Duration(seconds: 2)),
                );
              },
            ),
            IconButton(
              icon: const Icon(Icons.delete_outline, size: 18),
              onPressed: onDelete,
            ),
          ],
        ),
      ),
    );
  }
}
