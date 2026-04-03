import 'package:flutter/material.dart';

class TodoPage extends StatefulWidget {
  const TodoPage({super.key});

  @override
  State<TodoPage> createState() => _TodoPageState();
}

class _TodoPageState extends State<TodoPage> {
  final List<_Todo> _todos = [
    _Todo(id: 1, title: 'Buy groceries', priority: 'high'),
    _Todo(id: 2, title: 'Review pull request', priority: 'medium'),
    _Todo(id: 3, title: 'Read Flutter docs', priority: 'low', completed: true),
  ];

  String _filter = 'all';

  List<_Todo> get _filtered {
    switch (_filter) {
      case 'active':
        return _todos.where((t) => !t.completed).toList();
      case 'completed':
        return _todos.where((t) => t.completed).toList();
      default:
        return _todos;
    }
  }

  void _addTodo() {
    final controller = TextEditingController();
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
                controller: controller,
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
                  onPressed: () {
                    if (controller.text.trim().isNotEmpty) {
                      setState(() {
                        _todos.insert(
                          0,
                          _Todo(
                            id: DateTime.now().millisecondsSinceEpoch,
                            title: controller.text.trim(),
                            priority: priority,
                          ),
                        );
                      });
                      Navigator.pop(ctx);
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
    return Scaffold(
      body: SafeArea(
        child: CustomScrollView(
          slivers: [
            SliverAppBar.large(
              title: const Text('Todo'),
              actions: [
                IconButton(icon: const Icon(Icons.filter_list), onPressed: () {}),
                const SizedBox(width: 8),
              ],
            ),
            SliverPadding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              sliver: SliverToBoxAdapter(
                child: SegmentedButton<String>(
                  segments: const [
                    ButtonSegment(value: 'all', label: Text('All')),
                    ButtonSegment(value: 'active', label: Text('Active')),
                    ButtonSegment(value: 'completed', label: Text('Done')),
                  ],
                  selected: {_filter},
                  onSelectionChanged: (val) => setState(() => _filter = val.first),
                ),
              ),
            ),
            const SliverPadding(padding: EdgeInsets.only(top: 12)),
            SliverPadding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              sliver: _filtered.isEmpty
                  ? SliverToBoxAdapter(
                      child: Center(
                        child: Padding(
                          padding: const EdgeInsets.only(top: 80),
                          child: Column(
                            children: [
                              Icon(Icons.check_box_outlined, size: 64, color: Theme.of(context).colorScheme.outlineVariant),
                              const SizedBox(height: 12),
                              Text('No todos here', style: Theme.of(context).textTheme.bodyLarge),
                            ],
                          ),
                        ),
                      ),
                    )
                  : SliverList(
                      delegate: SliverChildBuilderDelegate(
                        (ctx, i) => _TodoTile(
                          todo: _filtered[i],
                          onToggle: () => setState(() => _filtered[i].completed = !_filtered[i].completed),
                          onDelete: () => setState(() => _todos.remove(_filtered[i])),
                        ),
                        childCount: _filtered.length,
                      ),
                    ),
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _addTodo,
        child: const Icon(Icons.add),
      ),
    );
  }
}

class _Todo {
  final int id;
  final String title;
  final String priority;
  bool completed;

  _Todo({required this.id, required this.title, required this.priority, this.completed = false});
}

class _TodoTile extends StatelessWidget {
  final _Todo todo;
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
          style: todo.completed
              ? const TextStyle(decoration: TextDecoration.lineThrough)
              : null,
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
              icon: const Icon(Icons.delete_outline, size: 18),
              onPressed: onDelete,
            ),
          ],
        ),
      ),
    );
  }
}
