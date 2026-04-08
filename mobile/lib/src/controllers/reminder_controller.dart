import 'dart:convert';

import 'package:flutter/foundation.dart';

import '../models/reminder_model.dart';
import '../rust/api/reminders.dart';
import '../services/notification_service.dart';

class ReminderController extends ChangeNotifier {
  List<Reminder> _reminders = [];
  bool loading = true;
  String? _workspaceId;

  List<Reminder> get reminders => List.unmodifiable(_reminders);

  static List<Reminder> _parse(String raw) {
    final list = (jsonDecode(raw) as List).cast<Map<String, dynamic>>();
    return list.map(Reminder.fromJson).toList();
  }

  Future<void> load(String workspaceId) async {
    _workspaceId = workspaceId;
    try {
      final raw = await getAllReminders(metaWorkspaceId: workspaceId);
      _reminders = await compute(_parse, raw);
    } catch (e) {
      debugPrint('ReminderController.load error: $e');
    }
    loading = false;
    notifyListeners();
    // Restore any alarms that were lost after a device reboot.
    await NotificationService.instance.rescheduleAll(_reminders);
  }

  Future<void> create({
    required String title,
    required String remindAt,
    required bool recurring,
    String? recurrenceRule,
    AlarmSound? alarmSound,
  }) async {
    if (_workspaceId == null) return;
    try {
      final raw = await createReminder(
        title: title,
        remindAt: remindAt,
        recurring: recurring,
        recurrenceRule: recurrenceRule,
        alarmSound: alarmSound?.stem,
        metaWorkspaceId: _workspaceId,
      );
      final json = jsonDecode(raw) as Map<String, dynamic>;
      final reminder = Reminder.fromJson(json);
      _reminders.add(reminder);
      notifyListeners();
      await NotificationService.instance.scheduleReminder(reminder);
    } catch (e) {
      debugPrint('ReminderController.create error: $e');
    }
  }

  Future<void> toggleRecurring(Reminder reminder) async {
    if (_workspaceId == null) return;
    try {
      final next = !reminder.recurring;
      await updateReminder(
        identifier: reminder.id,
        recurring: next,
        metaWorkspaceId: _workspaceId,
      );
      reminder.recurring = next;
      notifyListeners();
      if (next) {
        await NotificationService.instance.scheduleReminder(reminder);
      } else {
        await NotificationService.instance.cancelReminder(reminder.id);
      }
    } catch (e) {
      debugPrint('ReminderController.toggleRecurring error: $e');
    }
  }

  Future<void> delete(Reminder reminder) async {
    if (_workspaceId == null) return;
    try {
      await deleteReminder(identifier: reminder.id, metaWorkspaceId: _workspaceId);
      _reminders.removeWhere((r) => r.id == reminder.id);
      notifyListeners();
      await NotificationService.instance.cancelReminder(reminder.id);
    } catch (e) {
      debugPrint('ReminderController.delete error: $e');
    }
  }
}
