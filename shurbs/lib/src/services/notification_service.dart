import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter_local_notifications/flutter_local_notifications.dart';
import 'package:flutter_timezone/flutter_timezone.dart';
import 'package:timezone/data/latest_all.dart' as tz;
import 'package:timezone/timezone.dart' as tz;

import '../models/reminder_model.dart';

class NotificationService {
  NotificationService._();
  static final instance = NotificationService._();

  final _plugin = FlutterLocalNotificationsPlugin();
  bool _initialized = false;

  // Notification channel
  static const _androidDetails = AndroidNotificationDetails(
    'shurbs_reminders',
    'Reminders',
    channelDescription: 'Scheduled reminder alerts',
    importance: Importance.max,
    priority: Priority.high,
    playSound: true,
    enableVibration: true,
    fullScreenIntent: true,
  );

  static const _details = NotificationDetails(
    android: _androidDetails,
    iOS: DarwinNotificationDetails(
      presentAlert: true,
      presentBadge: true,
      presentSound: true,
    ),
  );

  // Day abbreviation → (flutter_local_notifications Day, Dart weekday int)
  static const _dayMap = <String, (Day, int)>{
    'Mon': (Day.monday, DateTime.monday),
    'Tue': (Day.tuesday, DateTime.tuesday),
    'Wed': (Day.wednesday, DateTime.wednesday),
    'Thu': (Day.thursday, DateTime.thursday),
    'Fri': (Day.friday, DateTime.friday),
    'Sat': (Day.saturday, DateTime.saturday),
    'Sun': (Day.sunday, DateTime.sunday),
  };

  Future<void> init() async {
    if (_initialized) return;

    tz.initializeTimeZones();
    final tzName = await FlutterTimezone.getLocalTimezone();
    tz.setLocalLocation(tz.getLocation(tzName));

    const androidInit = AndroidInitializationSettings('@mipmap/ic_launcher');
    const iosInit = DarwinInitializationSettings(
      requestAlertPermission: true,
      requestBadgePermission: true,
      requestSoundPermission: true,
    );

    await _plugin.initialize(
      const InitializationSettings(android: androidInit, iOS: iosInit),
    );

    if (Platform.isAndroid) {
      final android = _plugin
          .resolvePlatformSpecificImplementation<
              AndroidFlutterLocalNotificationsPlugin>();
      await android?.requestNotificationsPermission();
      await android?.requestExactAlarmsPermission();
    }

    _initialized = true;
  }

  /// Derives a stable positive int notification ID from a reminder UUID.
  /// dayOffset 0 = one-shot; 1–7 = per-weekday slots.
  int _idFor(String reminderId, [int dayOffset = 0]) {
    final hex = reminderId.replaceAll('-', '').substring(0, 8);
    return (int.parse(hex, radix: 16) + dayOffset) & 0x7FFFFFFF;
  }

  Future<void> scheduleReminder(Reminder reminder) async {
    if (!_initialized) return;

    // Always cancel first to avoid duplicate notifications.
    await cancelReminder(reminder.id);

    final now = tz.TZDateTime.now(tz.local);
    final hour = reminder.remindAt.hour;
    final minute = reminder.remindAt.minute;

    final days = reminder.days;

    if (days.isEmpty) {
      // ── One-shot or daily ────────────────────────────────────────────────────
      if (reminder.recurring) {
        // No days selected but marked recurring → repeat daily
        var scheduled = tz.TZDateTime(
            tz.local, now.year, now.month, now.day, hour, minute);
        if (scheduled.isBefore(now)) {
          scheduled = scheduled.add(const Duration(days: 1));
        }
        await _plugin.zonedSchedule(
          _idFor(reminder.id),
          reminder.title,
          null,
          scheduled,
          _details,
          matchDateTimeComponents: DateTimeComponents.time,
          androidScheduleMode: AndroidScheduleMode.exactAllowWhileIdle, uiLocalNotificationDateInterpretation: UILocalNotificationDateInterpretation.absoluteTime,
        );
      } else {
        // One-shot: fire at the next occurrence of remindAt
        var scheduled = tz.TZDateTime(
            tz.local, now.year, now.month, now.day, hour, minute);
        if (scheduled.isBefore(now)) {
          scheduled = scheduled.add(const Duration(days: 1));
        }
        await _plugin.zonedSchedule(
          _idFor(reminder.id),
          reminder.title,
          null,
          scheduled,
          _details,
          androidScheduleMode: AndroidScheduleMode.exactAllowWhileIdle, uiLocalNotificationDateInterpretation: UILocalNotificationDateInterpretation.absoluteTime,
        );
      }
    } else {
      // ── Weekly on selected days ───────────────────────────────────────────
      for (var i = 0; i < days.length; i++) {
        final entry = _dayMap[days[i]];
        if (entry == null) continue;
        final (_, weekday) = entry;

        final scheduled = _nextWeekday(weekday, hour, minute, now);

        await _plugin.zonedSchedule(
          _idFor(reminder.id, i + 1),
          reminder.title,
          null,
          scheduled,
          _details,
          matchDateTimeComponents: DateTimeComponents.dayOfWeekAndTime,
          androidScheduleMode: AndroidScheduleMode.exactAllowWhileIdle, uiLocalNotificationDateInterpretation: UILocalNotificationDateInterpretation.absoluteTime,
        );
      }
    }
  }

  tz.TZDateTime _nextWeekday(
      int weekday, int hour, int minute, tz.TZDateTime now) {
    var dt =
        tz.TZDateTime(tz.local, now.year, now.month, now.day, hour, minute);
    while (dt.weekday != weekday || dt.isBefore(now)) {
      dt = dt.add(const Duration(days: 1));
    }
    return dt;
  }

  Future<void> cancelReminder(String reminderId) async {
    if (!_initialized) return;
    // Cancel one-shot/daily slot and all 7 weekday slots
    await _plugin.cancel(_idFor(reminderId));
    for (var i = 1; i <= 7; i++) {
      await _plugin.cancel(_idFor(reminderId, i));
    }
  }

  /// Call on app launch to restore alarms lost after a device reboot.
  Future<void> rescheduleAll(List<Reminder> reminders) async {
    for (final r in reminders) {
      final hasTime = r.remindAt.isAfter(DateTime.now());
      if (r.recurring || r.days.isNotEmpty || hasTime) {
        await scheduleReminder(r);
      }
    }
  }
}
