import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:heroicons/heroicons.dart';

import '../controllers/bookmark_controller.dart';
import '../models/bookmark_model.dart';

class BookmarksPage extends StatelessWidget {
  final BookmarkController controller;

  const BookmarksPage({super.key, required this.controller});

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: controller,
      builder: (context, _) {
        if (controller.loading) {
          return const Scaffold(body: Center(child: CircularProgressIndicator()));
        }
        return _BookmarksView(controller: controller);
      },
    );
  }
}

class _BookmarksView extends StatelessWidget {
  final BookmarkController controller;

  const _BookmarksView({required this.controller});

  static const _validTags = ['development', 'inspiration', 'design', 'research'];

  void _showAddSheet(BuildContext context) {
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
                    final title = titleController.text.trim();
                    final url = urlController.text.trim();
                    if (title.isNotEmpty && url.isNotEmpty) {
                      Navigator.pop(ctx);
                      await controller.create(title, url, selectedTag);
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
    final c = controller;
    final filtered = c.filtered;

    return Scaffold(
      body: SafeArea(
        child: CustomScrollView(
          slivers: [
            SliverPadding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              sliver: SliverToBoxAdapter(
                child: Column(
                  children: [
                    if (c.bookmarks.isNotEmpty) ...[
                      SingleChildScrollView(
                        scrollDirection: Axis.horizontal,
                        child: Row(
                          children: [
                            FilterChip(
                              label: const Text('All'),
                              selected: c.activeTag == null,
                              onSelected: (_) => c.setActiveTag(null),
                            ),
                            const SizedBox(width: 8),
                            ..._validTags.map((tag) => Padding(
                                  padding: const EdgeInsets.only(right: 8),
                                  child: FilterChip(
                                    label: Text(tag),
                                    selected: c.activeTag == tag,
                                    onSelected: (_) => c.setActiveTag(tag),
                                  ),
                                )),
                          ],
                        ),
                      ),
                      const SizedBox(height: 8),
                    ],
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
                          onDelete: () => c.delete(filtered[i]),
                        ),
                        childCount: filtered.length,
                      ),
                    ),
                  ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => _showAddSheet(context),
        child: const HeroIcon(HeroIcons.plus),
      ),
    );
  }
}

class _BookmarkTile extends StatelessWidget {
  final Bookmark bookmark;
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
            style: TextStyle(
              color: theme.colorScheme.onSecondaryContainer,
              fontWeight: FontWeight.bold,
            ),
          ),
        ),
        title: Text(bookmark.title, maxLines: 1, overflow: TextOverflow.ellipsis),
        subtitle: Text(
          bookmark.url,
          maxLines: 1,
          overflow: TextOverflow.ellipsis,
          style: theme.textTheme.bodySmall,
        ),
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
                child: Text(
                  bookmark.tag!,
                  style: TextStyle(
                    fontSize: 11,
                    color: theme.colorScheme.onPrimaryContainer,
                  ),
                ),
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
