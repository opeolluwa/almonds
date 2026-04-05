import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

import '../rust/api/reminders.dart';

class _Reminder {
  final String id;
  String title;
  String? description;
  DateTime remindAt;
  bool recurring;
  String? recurrenceRule;

  _Reminder({
    required this.id,
    required this.title,
    this.description,
    required this.remindAt,
    this.recurring = false,
    this.recurrenceRule,
  });

  factory _Reminder.fromJson(Map<String, dynamic> j) => _Reminder(
        id: j['identifier'] as String,
        title: j['title'] as String,
        description: j['description'] as String?,
        remindAt: DateTime.parse(j['remindAt'] as String),
        recurring: j['recurring'] as bool? ?? false,
        recurrenceRule: j['recurrenceRule'] as String?,
      );

  TimeOfDay get time => TimeOfDay(hour: remindAt.hour, minute: remindAt.minute);

  List<String> get days {
    if (recurrenceRule == null || recurrenceRule!.isEmpty) return [];
    return recurrenceRule!.split(',').where((d) => d.isNotEmpty).toList();
  }
}

class AlarmsPage extends StatefulWidget {
  const AlarmsPage({super.key});

  @override
  State<AlarmsPage> createState() => _AlarmsPageState();
}

class _AlarmsPageState extends State<AlarmsPage> {
  List<_Reminder> _reminders = [];
  bool _loading = true;

  @override
  void initState() {
    super.initState();
    _loadReminders();
  }

  Future<void> _loadReminders() async {
    try {
      final raw = await getAllReminders();
      final list = (jsonDecode(raw) as List).cast<Map<String, dynamic>>();
      setState(() {
        _reminders = list.map(_Reminder.fromJson).toList();
        _loading = false;
      });
    } catch (_) {
      setState(() => _loading = false);
    }
  }

  Future<void> _deleteReminder(_Reminder r) async {
    try {
      await deleteReminder(identifier: r.id);
      setState(() => _reminders.removeWhere((x) => x.id == r.id));
    } catch (_) {}
  }

  Future<void> _toggleReminder(_Reminder r) async {
    // Toggle recurring flag as a proxy for enabled/disabled
    try {
      final newRecurring = !r.recurring;
      await updateReminder(identifier: r.id, recurring: newRecurring);
      setState(() => r.recurring = newRecurring);
    } catch (_) {}
  }

  void _addReminder() {
    final controller = TextEditingController();
    TimeOfDay selectedTime = TimeOfDay.now();
    final selectedDays = <String>{};

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
              Text('New Reminder', style: Theme.of(ctx).textTheme.titleLarge),
              const SizedBox(height: 16),
              TextField(
                controller: controller,
                autofocus: true,
                decoration: const InputDecoration(
                  hintText: 'Reminder label',
                  border: OutlineInputBorder(),
                ),
              ),
              const SizedBox(height: 12),
              OutlinedButton.icon(
                onPressed: () async {
                  final time = await showTimePicker(context: ctx, initialTime: selectedTime);
                  if (time != null) setModalState(() => selectedTime = time);
                },
                icon: const HeroIcon(HeroIcons.clock, size: 20),
                label: Text(selectedTime.format(ctx)),
              ),
              const SizedBox(height: 12),
              Wrap(
                spacing: 8,
                children: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'].map((day) {
                  final active = selectedDays.contains(day);
                  return FilterChip(
                    label: Text(day),
                    selected: active,
                    onSelected: (v) => setModalState(() => v ? selectedDays.add(day) : selectedDays.remove(day)),
                  );
                }).toList(),
              ),
              const SizedBox(height: 16),
              SizedBox(
                width: double.infinity,
                child: FilledButton(
                  onPressed: () async {
                    if (controller.text.trim().isNotEmpty) {
                      Navigator.pop(ctx);
                      try {
                        final now = DateTime.now();
                        final dt = DateTime(now.year, now.month, now.day, selectedTime.hour, selectedTime.minute);
                        final remindAt = dt.toIso8601String();
                        final rule = selectedDays.join(',');
                        final raw = await createReminder(
                          title: controller.text.trim(),
                          remindAt: remindAt,
                          recurring: selectedDays.isNotEmpty,
                          recurrenceRule: rule.isEmpty ? null : rule,
                        );
                        final json = jsonDecode(raw) as Map<String, dynamic>;
                        setState(() => _reminders.add(_Reminder.fromJson(json)));
                      } catch (_) {}
                    }
                  },
                  child: const Text('Save Reminder'),
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
    if (_loading) {
      return const Scaffold(body: Center(child: CircularProgressIndicator()));
    }

    return Scaffold(
      body: SafeArea(
        child: CustomScrollView(
          slivers: [
            _reminders.isEmpty
                ? SliverFillRemaining(
                    child: Center(
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          HeroIcon(HeroIcons.clock, size: 64, color: Theme.of(context).colorScheme.outlineVariant),
                          const SizedBox(height: 12),
                          const Text('No reminders set'),
                        ],
                      ),
                    ),
                  )
                : SliverPadding(
                    padding: const EdgeInsets.symmetric(horizontal: 16),
                    sliver: SliverList(
                      delegate: SliverChildBuilderDelegate(
                        (ctx, i) => _ReminderTile(
                          reminder: _reminders[i],
                          onToggle: () => _toggleReminder(_reminders[i]),
                          onDelete: () => _deleteReminder(_reminders[i]),
                        ),
                        childCount: _reminders.length,
                      ),
                    ),
                  ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _addReminder,
        child: const HeroIcon(HeroIcons.plus),
      ),
    );
  }
}

class _ReminderTile extends StatelessWidget {
  final _Reminder reminder;
  final VoidCallback onToggle;
  final VoidCallback onDelete;

  const _ReminderTile({required this.reminder, required this.onToggle, required this.onDelete});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final enabled = reminder.recurring;
    final time = reminder.time;

    return Card(
      margin: const EdgeInsets.only(bottom: 8),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        child: Row(
          children: [
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    '${time.hour.toString().padLeft(2, '0')}:${time.minute.toString().padLeft(2, '0')}',
                    style: theme.textTheme.headlineMedium?.copyWith(
                      color: enabled ? theme.colorScheme.onSurface : theme.colorScheme.outline,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(reminder.title, style: theme.textTheme.bodyMedium),
                  if (reminder.days.isNotEmpty) ...[
                    const SizedBox(height: 4),
                    Text(
                      reminder.days.join(' · '),
                      style: theme.textTheme.bodySmall?.copyWith(color: theme.colorScheme.primary),
                    ),
                  ],
                ],
              ),
            ),
            Transform.scale(
              scale: 0.75,
              child: Switch(value: enabled, onChanged: (_) => onToggle()),
            ),
            IconButton(icon: const HeroIcon(HeroIcons.trash, size: 20), onPressed: onDelete),
          ],
        ),
      ),
    );
  }
}
