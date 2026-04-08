import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

import '../controllers/reminder_controller.dart';
import '../models/reminder_model.dart';

class RemindersPage extends StatelessWidget {
  final ReminderController controller;

  const RemindersPage({super.key, required this.controller});

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: controller,
      builder: (context, _) {
        if (controller.loading) {
          return const Scaffold(body: Center(child: CircularProgressIndicator()));
        }
        return _RemindersView(controller: controller);
      },
    );
  }
}

class _RemindersView extends StatelessWidget {
  final ReminderController controller;

  const _RemindersView({required this.controller});

  static String _toRfc3339(DateTime dt) {
    final offset = dt.timeZoneOffset;
    final sign = offset.isNegative ? '-' : '+';
    final hh = offset.inHours.abs().toString().padLeft(2, '0');
    final mm = (offset.inMinutes.abs() % 60).toString().padLeft(2, '0');
    final base = dt.toIso8601String().split('.').first;
    return '$base$sign$hh:$mm';
  }

  void _showAddSheet(BuildContext context) {
    final labelController = TextEditingController();
    TimeOfDay selectedTime = TimeOfDay.now();
    final selectedDays = <String>{};
    AlarmSound? selectedSound;

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
                controller: labelController,
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
                    onSelected: (v) => setModalState(
                      () => v ? selectedDays.add(day) : selectedDays.remove(day),
                    ),
                  );
                }).toList(),
              ),
              const SizedBox(height: 12),
              // ── Sound picker ───────────────────────────────────────────────
              Row(
                children: [
                  const HeroIcon(HeroIcons.musicalNote, size: 18),
                  const SizedBox(width: 8),
                  Expanded(
                    child: DropdownButton<AlarmSound?>(
                      value: selectedSound,
                      isExpanded: true,
                      underline: const SizedBox.shrink(),
                      hint: const Text('Default sound'),
                      items: [
                        const DropdownMenuItem<AlarmSound?>(
                          value: null,
                          child: Text('Default sound'),
                        ),
                        ...AlarmSound.values.map(
                          (s) => DropdownMenuItem<AlarmSound?>(
                            value: s,
                            child: Text(s.label),
                          ),
                        ),
                      ],
                      onChanged: (v) => setModalState(() => selectedSound = v),
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 16),
              SizedBox(
                width: double.infinity,
                child: FilledButton(
                  onPressed: () async {
                    final label = labelController.text.trim();
                    if (label.isNotEmpty) {
                      Navigator.pop(ctx);
                      final now = DateTime.now();
                      final dt = DateTime(
                        now.year,
                        now.month,
                        now.day,
                        selectedTime.hour,
                        selectedTime.minute,
                      );
                      final rule = selectedDays.join(',');
                      await controller.create(
                        title: label,
                        remindAt: _toRfc3339(dt),
                        recurring: selectedDays.isNotEmpty,
                        recurrenceRule: rule.isEmpty ? null : rule,
                        alarmSound: selectedSound,
                      );
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
    final c = controller;

    return Scaffold(
      body: SafeArea(
        child: CustomScrollView(
          slivers: [
            c.reminders.isEmpty
                ? SliverFillRemaining(
                    child: Center(
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          HeroIcon(
                            HeroIcons.clock,
                            size: 64,
                            color: Theme.of(context).colorScheme.outlineVariant,
                          ),
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
                          reminder: c.reminders[i],
                          onToggle: () => c.toggleRecurring(c.reminders[i]),
                          onDelete: () => c.delete(c.reminders[i]),
                        ),
                        childCount: c.reminders.length,
                      ),
                    ),
                  ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => _showAddSheet(context),
        child: const HeroIcon(HeroIcons.plus),
      ),
    );
  }
}

class _ReminderTile extends StatelessWidget {
  final Reminder reminder;
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
            IconButton(
              icon: const HeroIcon(HeroIcons.trash, size: 20),
              onPressed: onDelete,
            ),
          ],
        ),
      ),
    );
  }
}
