import 'package:flutter/foundation.dart';

import '../models/todo_model.dart';
import 'bookmark_controller.dart';
import 'note_controller.dart';
import 'reminder_controller.dart';
import 'todo_controller.dart';

class HomeController extends ChangeNotifier {
  final TodoController todoController;
  final NoteController noteController;
  final BookmarkController bookmarkController;
  final ReminderController reminderController;

  HomeController({
    required this.todoController,
    required this.noteController,
    required this.bookmarkController,
    required this.reminderController,
  }) {
    todoController.addListener(notifyListeners);
    noteController.addListener(notifyListeners);
    bookmarkController.addListener(notifyListeners);
    reminderController.addListener(notifyListeners);
  }

  bool get loading =>
      todoController.loading ||
      noteController.loading ||
      bookmarkController.loading ||
      reminderController.loading;

  int get todoCount => todoController.todos.length;
  int get noteCount => noteController.notes.length;
  int get bookmarkCount => bookmarkController.bookmarks.length;
  int get reminderCount => reminderController.reminders.length;
  int get activeTodoCount => todoController.activeCount;

  Todo? get topTodo => todoController.topPriorityActive;

  /// All active (incomplete) todos — used for the home carousel.
  List<Todo> get activeTodos =>
      todoController.todos.where((t) => !t.completed).toList();

  List<ActivityItem> get recentActivity {
    final items = <ActivityItem>[];

    for (final note in noteController.notes.take(2)) {
      items.add(ActivityItem(
        title: note.title.isEmpty ? 'Untitled' : note.title,
        subtitle: 'Note updated',
        time: note.updatedAt,
        type: ActivityType.note,
      ));
    }

    for (final bookmark in bookmarkController.bookmarks.take(2)) {
      items.add(ActivityItem(
        title: bookmark.title,
        subtitle: 'Bookmark added',
        time: null,
        type: ActivityType.bookmark,
      ));
    }

    items.sort((a, b) {
      if (a.time == null && b.time == null) return 0;
      if (a.time == null) return 1;
      if (b.time == null) return -1;
      return b.time!.compareTo(a.time!);
    });

    return items.take(4).toList();
  }

  @override
  void dispose() {
    todoController.removeListener(notifyListeners);
    noteController.removeListener(notifyListeners);
    bookmarkController.removeListener(notifyListeners);
    reminderController.removeListener(notifyListeners);
    super.dispose();
  }
}

enum ActivityType { todo, note, bookmark, reminder }

class ActivityItem {
  final String title;
  final String subtitle;
  final DateTime? time;
  final ActivityType type;

  const ActivityItem({
    required this.title,
    required this.subtitle,
    required this.time,
    required this.type,
  });
}
