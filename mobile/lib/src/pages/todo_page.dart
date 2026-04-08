import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

import '../controllers/todo_controller.dart';
import '../models/todo_model.dart';

class TodoPage extends StatelessWidget {
  final TodoController controller;

  const TodoPage({super.key, required this.controller});

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: controller,
      builder: (context, _) {
        if (controller.loading) {
          return const Scaffold(body: Center(child: CircularProgressIndicator()));
        }
        return _TodoView(controller: controller);
      },
    );
  }
}

class _TodoView extends StatefulWidget {
  final TodoController controller;

  const _TodoView({required this.controller});

  @override
  State<_TodoView> createState() => _TodoViewState();
}

class _TodoViewState extends State<_TodoView> {
  late final TextEditingController _searchController;

  @override
  void initState() {
    super.initState();
    _searchController = TextEditingController();
  }

  @override
  void dispose() {
    _searchController.dispose();
    super.dispose();
  }

  void _showAddSheet(BuildContext context) {
    final titleController = TextEditingController();
    String priority = 'medium';

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
              Text('New Todo', style: Theme.of(ctx).textTheme.titleLarge),
              const SizedBox(height: 16),
              TextField(
                controller: titleController,
                autofocus: true,
                decoration: const InputDecoration(
                  hintText: 'What needs to be done?',
                  border: OutlineInputBorder(),
                ),
              ),
              const SizedBox(height: 12),
              SegmentedButton<String>(
                segments: const [
                  ButtonSegment(value: 'high', label: Text('High')),
                  ButtonSegment(value: 'medium', label: Text('Medium')),
                  ButtonSegment(value: 'low', label: Text('Low')),
                ],
                selected: {priority},
                onSelectionChanged: (val) => setModalState(() => priority = val.first),
              ),
              const SizedBox(height: 16),
              SizedBox(
                width: double.infinity,
                child: FilledButton(
                  onPressed: () async {
                    final title = titleController.text.trim();
                    if (title.isNotEmpty) {
                      Navigator.pop(ctx);
                      await widget.controller.create(title, priority);
                    }
                  },
                  child: const Text('Add Todo'),
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
    final c = widget.controller;
    final filtered = c.filtered;

    return Scaffold(
      body: CustomScrollView(
        slivers: [
          if (c.todos.isNotEmpty) ...[
            SliverPadding(
              padding: const EdgeInsets.fromLTRB(16, 12, 16, 0),
              sliver: SliverToBoxAdapter(
                child: SegmentedButton<String>(
                  segments: const [
                    ButtonSegment(value: 'all', label: Text('All')),
                    ButtonSegment(value: 'active', label: Text('Active')),
                    ButtonSegment(value: 'completed', label: Text('Done')),
                  ],
                  selected: {c.filter},
                  onSelectionChanged: (val) => c.setFilter(val.first),
                ),
              ),
            ),
            const SliverPadding(padding: EdgeInsets.only(top: 12)),
          ],
          SliverPadding(
            padding: const EdgeInsets.symmetric(horizontal: 16),
            sliver: filtered.isEmpty
                ? SliverFillRemaining(
                    child: Center(
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          HeroIcon(HeroIcons.checkCircle, size: 64, color: colorScheme.outlineVariant),
                          const SizedBox(height: 12),
                          Text('No todos here', style: Theme.of(context).textTheme.bodyLarge),
                        ],
                      ),
                    ),
                  )
                : SliverList(
                    delegate: SliverChildBuilderDelegate(
                      (ctx, i) => _TodoTile(
                        todo: filtered[i],
                        onToggle: () => c.toggleDone(filtered[i]),
                        onDelete: () => c.delete(filtered[i]),
                      ),
                      childCount: filtered.length,
                    ),
                  ),
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => _showAddSheet(context),
        child: const HeroIcon(HeroIcons.plus),
      ),
    );
  }
}

class _TodoTile extends StatelessWidget {
  final Todo todo;
  final VoidCallback onToggle;
  final VoidCallback onDelete;

  const _TodoTile({required this.todo, required this.onToggle, required this.onDelete});

  Color _priorityColor(BuildContext context) {
    switch (todo.priority) {
      case 'high':
        return Colors.red;
      case 'medium':
        return Colors.orange;
      default:
        return Colors.green;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.only(bottom: 8),
      child: ListTile(
        leading: Checkbox(
          value: todo.completed,
          onChanged: (_) => onToggle(),
        ),
        title: Text(
          todo.title,
          style: todo.completed ? const TextStyle(decoration: TextDecoration.lineThrough) : null,
        ),
        trailing: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
              decoration: BoxDecoration(
                color: _priorityColor(context).withValues(alpha: 0.15),
                borderRadius: BorderRadius.circular(12),
              ),
              child: Text(
                todo.priority,
                style: TextStyle(fontSize: 11, color: _priorityColor(context)),
              ),
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
