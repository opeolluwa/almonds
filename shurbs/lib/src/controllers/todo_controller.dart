import 'dart:convert';

import 'package:flutter/foundation.dart';

import '../models/todo_model.dart';
import '../rust/api/todo.dart';

class TodoController extends ChangeNotifier {
  List<Todo> _todos = [];
  bool loading = true;
  String filter = 'all';
  String search = '';
  String? _workspaceId;

  List<Todo> get todos => List.unmodifiable(_todos);

  List<Todo> get filtered {
    List<Todo> list;
    switch (filter) {
      case 'active':
        list = _todos.where((t) => !t.completed).toList();
        break;
      case 'completed':
        list = _todos.where((t) => t.completed).toList();
        break;
      default:
        list = List.from(_todos);
    }
    if (search.trim().isNotEmpty) {
      final q = search.toLowerCase();
      list = list.where((t) => t.title.toLowerCase().contains(q)).toList();
    }
    return list;
  }

  int get activeCount => _todos.where((t) => !t.completed).length;

  Todo? get topPriorityActive {
    final active = _todos.where((t) => !t.completed).toList();
    if (active.isEmpty) return null;
    final high = active.where((t) => t.priority == 'high').toList();
    return high.isNotEmpty ? high.first : active.first;
  }

  static List<Todo> _parse(String raw) {
    final list = (jsonDecode(raw) as List).cast<Map<String, dynamic>>();
    return list.map(Todo.fromJson).toList();
  }

  Future<void> load(String workspaceId) async {
    _workspaceId = workspaceId;
    try {
      final raw = await getAllTodos(metaWorkspaceId: workspaceId);
      _todos = await compute(_parse, raw);
    } catch (e) {
      debugPrint('TodoController.load error: $e');
    }
    loading = false;
    notifyListeners();
  }

  Future<void> create(String title, String priority) async {
    if (_workspaceId == null) return;
    try {
      final raw = await createTodo(
        title: title,
        priority: priority,
        metaWorkspaceId: _workspaceId,
      );
      final json = jsonDecode(raw) as Map<String, dynamic>;
      _todos.insert(0, Todo.fromJson(json));
      notifyListeners();
    } catch (e) {
      debugPrint('TodoController.create error: $e');
    }
  }

  Future<void> toggleDone(Todo todo) async {
    if (_workspaceId == null) return;
    try {
      await markTodoDone(
        identifier: todo.id,
        done: !todo.completed,
        metaWorkspaceId: _workspaceId,
      );
      todo.completed = !todo.completed;
      notifyListeners();
    } catch (e) {
      debugPrint('TodoController.toggleDone error: $e');
    }
  }

  Future<void> delete(Todo todo) async {
    if (_workspaceId == null) return;
    try {
      await deleteTodo(identifier: todo.id, metaWorkspaceId: _workspaceId);
      _todos.removeWhere((t) => t.id == todo.id);
      notifyListeners();
    } catch (e) {
      debugPrint('TodoController.delete error: $e');
    }
  }

  void setFilter(String f) {
    filter = f;
    notifyListeners();
  }

  void setSearch(String s) {
    search = s;
    notifyListeners();
  }
}
