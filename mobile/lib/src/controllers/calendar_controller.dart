import 'package:flutter/material.dart';

import '../models/calendar_event_model.dart';
import '../controllers/todo_controller.dart';
import '../controllers/reminder_controller.dart';

enum CalendarItemType { todo, reminder, event }

class CalendarDayItem {
  final CalendarItemType type;
  final String id;
  final String title;
  final String? description;
  /// Minutes from midnight; null = all-day
  final int? startMinute;
  final Color color;
  final bool completed;

  const CalendarDayItem({
    required this.type,
    required this.id,
    required this.title,
    this.description,
    this.startMinute,
    required this.color,
    this.completed = false,
  });

  String get timeLabel {
    if (startMinute == null) return 'All day';
    final h = startMinute! ~/ 60;
    final m = startMinute! % 60;
    final suffix = h < 12 ? 'AM' : 'PM';
    final hour = h == 0 ? 12 : (h > 12 ? h - 12 : h);
    return '$hour:${m.toString().padLeft(2, '0')} $suffix';
  }
}

class CalendarController extends ChangeNotifier {
  final TodoController todoController;
  final ReminderController reminderController;

  CalendarController({
    required this.todoController,
    required this.reminderController,
  }) {
    todoController.addListener(_onDep);
    reminderController.addListener(_onDep);
  }

  void _onDep() => notifyListeners();

  final List<CalendarEvent> _events = [];
  bool _googleConnected = false;
  String? _googleEmail;
  bool _googleLoading = false;

  bool get googleConnected => _googleConnected;
  String? get googleEmail => _googleEmail;
  bool get googleLoading => _googleLoading;
  List<CalendarEvent> get events => List.unmodifiable(_events);

  // ── Query ──────────────────────────────────────────────────────────────────

  List<CalendarDayItem> itemsForDate(DateTime date) {
    final day = DateTime(date.year, date.month, date.day);
    final items = <CalendarDayItem>[];

    for (final todo in todoController.todos) {
      if (todo.dueDate != null) {
        final d = DateTime(todo.dueDate!.year, todo.dueDate!.month, todo.dueDate!.day);
        if (d == day) {
          items.add(CalendarDayItem(
            type: CalendarItemType.todo,
            id: todo.id,
            title: todo.title,
            startMinute: null,
            color: const Color(0xFF6366F1),
            completed: todo.completed,
          ));
        }
      }
    }

    for (final r in reminderController.reminders) {
      final d = DateTime(r.remindAt.year, r.remindAt.month, r.remindAt.day);
      if (d == day) {
        items.add(CalendarDayItem(
          type: CalendarItemType.reminder,
          id: r.id,
          title: r.title,
          description: r.description,
          startMinute: r.remindAt.hour * 60 + r.remindAt.minute,
          color: const Color(0xFFF59E0B),
        ));
      }
    }

    for (final e in _events) {
      final d = DateTime(e.date.year, e.date.month, e.date.day);
      if (d == day) {
        items.add(CalendarDayItem(
          type: CalendarItemType.event,
          id: e.id,
          title: e.title,
          description: e.description,
          startMinute: e.startMinute,
          color: e.color.value,
        ));
      }
    }

    items.sort((a, b) => (a.startMinute ?? -1).compareTo(b.startMinute ?? -1));
    return items;
  }

  bool hasItemsOnDate(DateTime date) => itemsForDate(date).isNotEmpty;

  /// Returns up to [max] distinct dot colors for a date (for the month grid).
  List<Color> dotColorsForDate(DateTime date, {int max = 3}) {
    final items = itemsForDate(date);
    final seen = <Color>{};
    final colors = <Color>[];
    for (final item in items) {
      if (seen.add(item.color) && colors.length < max) {
        colors.add(item.color);
      }
    }
    return colors;
  }

  // ── Events CRUD ────────────────────────────────────────────────────────────

  void addEvent(CalendarEvent event) {
    _events.add(event);
    notifyListeners();
  }

  void removeEvent(String id) {
    _events.removeWhere((e) => e.id == id);
    notifyListeners();
  }

  void updateEvent(CalendarEvent updated) {
    final i = _events.indexWhere((e) => e.id == updated.id);
    if (i != -1) {
      _events[i] = updated;
      notifyListeners();
    }
  }

  // ── Google Calendar ────────────────────────────────────────────────────────
  //
  // To fully enable Google Calendar sync:
  // 1. Create an OAuth 2.0 client ID in the Google Cloud Console.
  // 2. Add the reversed client ID as a URL scheme in ios/Runner/Info.plist.
  // 3. Add the client ID to android/app/build.gradle (or strings.xml).
  // 4. Run `flutter pub get` after adding `google_sign_in` to pubspec.yaml.
  // 5. Uncomment the google_sign_in implementation below and remove the stub.

  Future<void> connectGoogle() async {
    _googleLoading = true;
    notifyListeners();

    // ── Stub — replace with real google_sign_in flow ──────────────────────
    // final _googleSignIn = GoogleSignIn(
    //   scopes: ['https://www.googleapis.com/auth/calendar.readonly'],
    // );
    // try {
    //   final account = await _googleSignIn.signIn();
    //   if (account != null) {
    //     _googleConnected = true;
    //     _googleEmail = account.email;
    //     await _syncGoogleEvents(account);
    //   }
    // } catch (e) {
    //   debugPrint('Google sign-in error: $e');
    // }
    // ─────────────────────────────────────────────────────────────────────

    await Future.delayed(const Duration(milliseconds: 600));
    _googleLoading = false;
    _googleConnected = true;
    _googleEmail = 'Configure OAuth to sync'; // replaced by real email after setup
    notifyListeners();
  }

  Future<void> disconnectGoogle() async {
    _googleConnected = false;
    _googleEmail = null;
    _events.removeWhere((e) => e.source == 'google');
    notifyListeners();
  }

  @override
  void dispose() {
    todoController.removeListener(_onDep);
    reminderController.removeListener(_onDep);
    super.dispose();
  }
}
