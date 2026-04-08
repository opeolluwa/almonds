import 'package:flutter/material.dart';

enum CalendarEventColor { indigo, amber, emerald, rose, blue, orange }

extension CalendarEventColorExt on CalendarEventColor {
  Color get value {
    switch (this) {
      case CalendarEventColor.indigo:
        return const Color(0xFF6366F1);
      case CalendarEventColor.amber:
        return const Color(0xFFF59E0B);
      case CalendarEventColor.emerald:
        return const Color(0xFF10B981);
      case CalendarEventColor.rose:
        return const Color(0xFFF43F5E);
      case CalendarEventColor.blue:
        return const Color(0xFF3B82F6);
      case CalendarEventColor.orange:
        return const Color(0xFFEA580C);
    }
  }

  String get label {
    switch (this) {
      case CalendarEventColor.indigo:
        return 'Indigo';
      case CalendarEventColor.amber:
        return 'Amber';
      case CalendarEventColor.emerald:
        return 'Emerald';
      case CalendarEventColor.rose:
        return 'Rose';
      case CalendarEventColor.blue:
        return 'Blue';
      case CalendarEventColor.orange:
        return 'Orange';
    }
  }
}

class CalendarEvent {
  final String id;
  String title;
  String? description;
  DateTime date;
  /// Minutes from midnight (null = all-day)
  int? startMinute;
  int? endMinute;
  CalendarEventColor color;
  /// 'local' | 'google'
  final String source;

  CalendarEvent({
    required this.id,
    required this.title,
    this.description,
    required this.date,
    this.startMinute,
    this.endMinute,
    this.color = CalendarEventColor.indigo,
    this.source = 'local',
  });

  bool get isAllDay => startMinute == null;

  String get startTimeLabel {
    if (startMinute == null) return 'All day';
    final h = startMinute! ~/ 60;
    final m = startMinute! % 60;
    final suffix = h < 12 ? 'AM' : 'PM';
    final hour = h == 0 ? 12 : (h > 12 ? h - 12 : h);
    return '$hour:${m.toString().padLeft(2, '0')} $suffix';
  }

  String get endTimeLabel {
    if (endMinute == null) return '';
    final h = endMinute! ~/ 60;
    final m = endMinute! % 60;
    final suffix = h < 12 ? 'AM' : 'PM';
    final hour = h == 0 ? 12 : (h > 12 ? h - 12 : h);
    return '$hour:${m.toString().padLeft(2, '0')} $suffix';
  }
}
