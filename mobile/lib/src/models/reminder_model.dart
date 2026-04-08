import 'package:flutter/material.dart';

/// The three bundled alarm sounds. The value matches the filename stem in
/// `res/raw/` (Android) and `Runner/` (iOS), e.g. `funny_alarm` → `funny_alarm.mp3`.
enum AlarmSound {
  funnyAlarm,
  softPiano,
  relaxingGuitar;

  /// Filename stem used for Android raw-resource and iOS bundle lookup.
  String get stem {
    switch (this) {
      case AlarmSound.funnyAlarm:
        return 'funny_alarm';
      case AlarmSound.softPiano:
        return 'soft_piano';
      case AlarmSound.relaxingGuitar:
        return 'relaxing_guitar';
    }
  }

  String get assetPath => 'assets/sounds/$stem.mp3';

  String get label {
    switch (this) {
      case AlarmSound.funnyAlarm:
        return 'Funny alarm';
      case AlarmSound.softPiano:
        return 'Soft piano';
      case AlarmSound.relaxingGuitar:
        return 'Relaxing guitar';
    }
  }

  static AlarmSound? fromStem(String? stem) {
    if (stem == null) return null;
    for (final v in AlarmSound.values) {
      if (v.stem == stem) return v;
    }
    return null;
  }
}

class Reminder {
  final String id;
  String title;
  String? description;
  DateTime remindAt;
  bool recurring;
  String? recurrenceRule;
  AlarmSound? alarmSound;

  Reminder({
    required this.id,
    required this.title,
    this.description,
    required this.remindAt,
    this.recurring = false,
    this.recurrenceRule,
    this.alarmSound,
  });

  factory Reminder.fromJson(Map<String, dynamic> j) => Reminder(
        id: j['identifier'] as String,
        title: j['title'] as String,
        description: j['description'] as String?,
        remindAt: DateTime.parse(j['remindAt'] as String).toLocal(),
        recurring: j['recurring'] as bool? ?? false,
        recurrenceRule: j['recurrenceRule'] as String?,
        alarmSound: AlarmSound.fromStem(j['alarmSound'] as String?),
      );

  TimeOfDay get time => TimeOfDay(hour: remindAt.hour, minute: remindAt.minute);

  List<String> get days {
    if (recurrenceRule == null || recurrenceRule!.isEmpty) return [];
    return recurrenceRule!.split(',').where((d) => d.isNotEmpty).toList();
  }
}
