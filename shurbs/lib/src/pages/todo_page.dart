import 'dart:io';

import 'package:flutter/material.dart';
import 'package:path_provider/path_provider.dart';

import '../rust/api/todo.dart';

class TodoPage extends StatefulWidget {
  const TodoPage({super.key});

  @override
  State<TodoPage> createState() => _TodoPageState();
}

class _TodoPageState extends State<TodoPage> {
  List<TodoItem> _todos = [];
  bool _loading = true;
  String? _error;
  String _filter = 'all';

  @override
  void initState() {
    super.initState();
    _init();
  }

  Future<void> _init() async {
    try {
      final dir = await getApplicationDocumentsDirectory();
      final dbPath = '${dir.path}${Platform.pathSeparator}shurbs.db';
      await initKernel(dbPath: dbPath);
      await _load();
    } catch (e) {
      setState(() {
        _error = e.toString();
        _loading = false;
      });
    }
  }

  Future<void> _load() async {
    final todos = await listTodos();
    setState(() {
      _todos = todos;
      _loading = false;
      _error = null;
    });
  }

  List<TodoItem> get _filtered {
    switch (_filter) {
      case 'active':
        return _todos.where((t) => !t.done).toList();
      case 'completed':
        return _todos.where((t) => t.done).toList();
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
                onSelectionChanged: (val) =>
                    setModalState(() => priority = val.first),
              ),
              const SizedBox(height: 16),
              SizedBox(
                width: double.infinity,
                child: FilledButton(
                  onPressed: () async {
                    final title = controller.text.trim();
                    if (title.isEmpty) return;
                    Navigator.pop(ctx);
                    await createTodo(
                      title: title,
                      description: null,
                      priority: priority,
                    );
                    await _load();
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

  Future<void> _toggle(TodoItem todo) async {
    await markTodoDone(id: todo.id, done: !todo.done);
    await _load();
  }

  Future<void> _delete(TodoItem todo) async {
    await deleteTodo(id: todo.id);
    await _load();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SafeArea(
        child: _loading
            ? const Center(child: CircularProgressIndicator())
            : _error != null
                ? Center(
                    child: Padding(
                      padding: const EdgeInsets.all(24),
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          const Icon(Icons.error_outline,
                              size: 48, color: Colors.red),
                          const SizedBox(height: 12),
                          Text(_error!,
                              textAlign: TextAlign.center,
                              style: Theme.of(context).textTheme.bodyMedium),
                          const SizedBox(height: 16),
                          FilledButton(
                              onPressed: _init, child: const Text('Retry')),
                        ],
                      ),
                    ),
                  )
                : CustomScrollView(
                    slivers: [
                      SliverAppBar(
                        pinned: true,
                        title: const Text('Todo'),
                        actions: [
                          IconButton(
                            icon: const Icon(Icons.refresh),
                            onPressed: _load,
                          ),
                          const SizedBox(width: 8),
                        ],
                      ),
                      SliverPadding(
                        padding: const EdgeInsets.symmetric(horizontal: 16),
                        sliver: SliverToBoxAdapter(
                          child: SegmentedButton<String>(
                            segments: const [
                              ButtonSegment(value: 'all', label: Text('All')),
                              ButtonSegment(
                                  value: 'active', label: Text('Active')),
                              ButtonSegment(
                                  value: 'completed', label: Text('Done')),
                            ],
                            selected: {_filter},
                            onSelectionChanged: (val) =>
                                setState(() => _filter = val.first),
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
                                        Icon(Icons.check_box_outlined,
                                            size: 64,
                                            color: Theme.of(context)
                                                .colorScheme
                                                .outlineVariant),
                                        const SizedBox(height: 12),
                                        Text('No todos here',
                                            style: Theme.of(context)
                                                .textTheme
                                                .bodyLarge),
                                      ],
                                    ),
                                  ),
                                ),
                              )
                            : SliverList(
                                delegate: SliverChildBuilderDelegate(
                                  (ctx, i) => _TodoTile(
                                    todo: _filtered[i],
                                    onToggle: () => _toggle(_filtered[i]),
                                    onDelete: () => _delete(_filtered[i]),
                                  ),
                                  childCount: _filtered.length,
                                ),
                              ),
                      ),
                    ],
                  ),
      ),
      floatingActionButton: _loading || _error != null
          ? null
          : FloatingActionButton(
              onPressed: _addTodo,
              child: const Icon(Icons.add),
            ),
    );
  }
}

class _TodoTile extends StatelessWidget {
  final TodoItem todo;
  final VoidCallback onToggle;
  final VoidCallback onDelete;

  const _TodoTile(
      {required this.todo, required this.onToggle, required this.onDelete});

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
          value: todo.done,
          onChanged: (_) => onToggle(),
        ),
        title: Text(
          todo.title,
          style: todo.done
              ? const TextStyle(decoration: TextDecoration.lineThrough)
              : null,
        ),
        subtitle: todo.description != null
            ? Text(todo.description!,
                maxLines: 1, overflow: TextOverflow.ellipsis)
            : null,
        trailing: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Container(
              padding:
                  const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
              decoration: BoxDecoration(
                color: _priorityColor(context).withValues(alpha: 0.15),
                borderRadius: BorderRadius.circular(12),
              ),
              child: Text(
                todo.priority,
                style:
                    TextStyle(fontSize: 11, color: _priorityColor(context)),
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
