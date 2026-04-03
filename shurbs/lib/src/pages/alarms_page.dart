import 'package:flutter/material.dart';

class AlarmsPage extends StatefulWidget {
  const AlarmsPage({super.key});

  @override
  State<AlarmsPage> createState() => _AlarmsPageState();
}

class _AlarmsPageState extends State<AlarmsPage> {
  final List<_Alarm> _alarms = [
    _Alarm(id: 1, title: 'Morning standup', time: TimeOfDay(hour: 9, minute: 0), days: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri']),
    _Alarm(id: 2, title: 'Lunch break', time: TimeOfDay(hour: 13, minute: 0), days: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri'], enabled: false),
    _Alarm(id: 3, title: 'Evening workout', time: TimeOfDay(hour: 18, minute: 30), days: ['Mon', 'Wed', 'Fri']),
  ];

  void _addAlarm() {
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
              Text('New Alarm', style: Theme.of(ctx).textTheme.titleLarge),
              const SizedBox(height: 16),
              TextField(
                controller: controller,
                autofocus: true,
                decoration: const InputDecoration(
                  hintText: 'Alarm label',
                  border: OutlineInputBorder(),
                ),
              ),
              const SizedBox(height: 12),
              OutlinedButton.icon(
                onPressed: () async {
                  final time = await showTimePicker(context: ctx, initialTime: selectedTime);
                  if (time != null) setModalState(() => selectedTime = time);
                },
                icon: const Icon(Icons.access_time),
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
                  onPressed: () {
                    if (controller.text.trim().isNotEmpty) {
                      setState(() {
                        _alarms.add(_Alarm(
                          id: DateTime.now().millisecondsSinceEpoch,
                          title: controller.text.trim(),
                          time: selectedTime,
                          days: selectedDays.toList(),
                        ));
                      });
                      Navigator.pop(ctx);
                    }
                  },
                  child: const Text('Save Alarm'),
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
    return Scaffold(
      body: SafeArea(
        child: CustomScrollView(
          slivers: [
            const SliverAppBar(pinned: true, title: Text('Alarms')),
            _alarms.isEmpty
                ? SliverFillRemaining(
                    child: Center(
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          Icon(Icons.alarm_off, size: 64, color: Theme.of(context).colorScheme.outlineVariant),
                          const SizedBox(height: 12),
                          const Text('No alarms set'),
                        ],
                      ),
                    ),
                  )
                : SliverPadding(
                    padding: const EdgeInsets.symmetric(horizontal: 16),
                    sliver: SliverList(
                      delegate: SliverChildBuilderDelegate(
                        (ctx, i) => _AlarmTile(
                          alarm: _alarms[i],
                          onToggle: () => setState(() => _alarms[i].enabled = !_alarms[i].enabled),
                          onDelete: () => setState(() => _alarms.removeAt(i)),
                        ),
                        childCount: _alarms.length,
                      ),
                    ),
                  ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _addAlarm,
        child: const Icon(Icons.add),
      ),
    );
  }
}

class _Alarm {
  final int id;
  final String title;
  final TimeOfDay time;
  final List<String> days;
  bool enabled;

  _Alarm({required this.id, required this.title, required this.time, required this.days, this.enabled = true});
}

class _AlarmTile extends StatelessWidget {
  final _Alarm alarm;
  final VoidCallback onToggle;
  final VoidCallback onDelete;

  const _AlarmTile({required this.alarm, required this.onToggle, required this.onDelete});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

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
                    alarm.time.format(context),
                    style: theme.textTheme.headlineMedium?.copyWith(
                      color: alarm.enabled ? theme.colorScheme.onSurface : theme.colorScheme.outline,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(alarm.title, style: theme.textTheme.bodyMedium),
                  const SizedBox(height: 4),
                  if (alarm.days.isNotEmpty)
                    Text(
                      alarm.days.join(' · '),
                      style: theme.textTheme.bodySmall?.copyWith(color: theme.colorScheme.primary),
                    ),
                ],
              ),
            ),
            Switch(value: alarm.enabled, onChanged: (_) => onToggle()),
            IconButton(icon: const Icon(Icons.delete_outline, size: 18), onPressed: onDelete),
          ],
        ),
      ),
    );
  }
}
