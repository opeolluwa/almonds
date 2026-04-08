class Todo {
  final String id;
  String title;
  String priority;
  bool completed;
  DateTime? dueDate;

  Todo({
    required this.id,
    required this.title,
    required this.priority,
    this.completed = false,
    this.dueDate,
  });

  factory Todo.fromJson(Map<String, dynamic> j) => Todo(
        id: j['identifier'] as String,
        title: j['title'] as String,
        priority: j['priority'] as String? ?? 'medium',
        completed: j['done'] as bool? ?? false,
        dueDate: j['dueDate'] != null ? DateTime.tryParse(j['dueDate'] as String) : null,
      );
}
