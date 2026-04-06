import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

import '../rust/api/todo.dart';

class _Todo {
  final String id;
  String title;
  String priority;
  bool completed;

  _Todo({required this.id, required this.title, required this.priority, this.completed = false});

  factory _Todo.fromJson(Map<String, dynamic> j) => _Todo(
        id: j['identifier'] as String,
        title: j['title'] as String,
        priority: j['priority'] as String? ?? 'medium',
        completed: j['done'] as bool? ?? false,
      );
}

class TodoPage extends StatefulWidget {
  const TodoPage({super.key});

  @override
  State<TodoPage> createState() => _TodoPageState();
}

class _TodoPageState extends State<TodoPage> {
  List<_Todo> _todos = [];
  bool _loading = true;
  String _filter = 'all';
  String _search = '';

  @override
  void initState() {
    super.initState();
    _loadTodos();
  }

  Future<void> _loadTodos() async {
    try {
      final raw = await getAllTodos();
      final list = (jsonDecode(raw) as List).cast<Map<String, dynamic>>();
      setState(() {
        _todos = list.map(_Todo.fromJson).toList();
        _loading = false;
      });
    } catch (_) {
      setState(() => _loading = false);
    }
  }

  List<_Todo> get _filtered {
    List<_Todo> list;
    switch (_filter) {
      case 'active':
        list = _todos.where((t) => !t.completed).toList();
        break;
      case 'completed':
        list = _todos.where((t) => t.completed).toList();
        break;
      default:
        list = List.from(_todos);
    }
    if (_search.trim().isNotEmpty) {
      list = list.where((t) => t.title.toLowerCase().contains(_search.toLowerCase())).toList();
    }
    return list;
  }

  Future<void> _toggleDone(_Todo todo) async {
    try {
      await markTodoDone(identifier: todo.id, done: !todo.completed);
      setState(() => todo.completed = !todo.completed);
    } catch (_) {}
  }

  Future<void> _deleteTodo(_Todo todo) async {
    try {
      await deleteTodo(identifier: todo.id);
      setState(() => _todos.removeWhere((t) => t.id == todo.id));
    } catch (_) {}
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
                  onPressed: () async {
                    if (controller.text.trim().isNotEmpty) {
                      Navigator.pop(ctx);
                      try {
                        final raw = await createTodo(
                          title: controller.text.trim(),
                          priority: priority,
                        );
                        final json = jsonDecode(raw) as Map<String, dynamic>;
                        setState(() => _todos.insert(0, _Todo.fromJson(json)));
                      } catch (_) {}
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

    if (_loading) {
      return const Scaffold(body: Center(child: CircularProgressIndicator()));
    }

    final filtered = _filtered;

    return Scaffold(
      body: CustomScrollView(
        slivers: [
          if (_todos.isNotEmpty) ...[
            SliverPadding(
              padding: const EdgeInsets.fromLTRB(16, 12, 16, 0),
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
                        onToggle: () => _toggleDone(filtered[i]),
                        onDelete: () => _deleteTodo(filtered[i]),
                      ),
                      childCount: filtered.length,
                    ),
                  ),
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _addTodo,
        child: const HeroIcon(HeroIcons.plus),
      ),
    );
  }
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
