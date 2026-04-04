import 'package:flutter/material.dart';

class NotificationsSettingsPage extends StatefulWidget {
  const NotificationsSettingsPage({super.key});

  @override
  State<NotificationsSettingsPage> createState() => _NotificationsSettingsPageState();
}

class _NotificationsSettingsPageState extends State<NotificationsSettingsPage> {
  bool _tasksDue = true;
  bool _reminders = true;
  bool _syncComplete = false;
  bool _appUpdates = true;

  @override
  Widget build(BuildContext context) {
    final items = [
      _NotifItem(label: 'Tasks due', desc: 'Remind me when tasks are due', value: _tasksDue, onChanged: (v) => setState(() => _tasksDue = v)),
      _NotifItem(label: 'Reminders', desc: 'Calendar event reminders', value: _reminders, onChanged: (v) => setState(() => _reminders = v)),
      _NotifItem(label: 'Sync complete', desc: 'Notify when sync finishes', value: _syncComplete, onChanged: (v) => setState(() => _syncComplete = v)),
      _NotifItem(label: 'App updates', desc: 'Notify about new versions', value: _appUpdates, onChanged: (v) => setState(() => _appUpdates = v)),
    ];

    return Scaffold(
      appBar: AppBar(title: const Text('Notifications')),
      body: SafeArea(
        child: ListView(
          padding: const EdgeInsets.all(16),
          children: [
            Card(
              child: Padding(
                padding: const EdgeInsets.symmetric(vertical: 8),
                child: Column(
                  children: [
                    for (int i = 0; i < items.length; i++) ...[
                      if (i > 0) const Divider(height: 1, indent: 16, endIndent: 16),
                      ListTile(
                        title: Text(items[i].label),
                        subtitle: Text(items[i].desc),
                        trailing: Transform.scale(
                          scale: 0.8,
                          child: Switch(value: items[i].value, onChanged: items[i].onChanged),
                        ),
                      ),
                    ],
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _NotifItem {
  final String label;
  final String desc;
  final bool value;
  final ValueChanged<bool> onChanged;
  const _NotifItem({required this.label, required this.desc, required this.value, required this.onChanged});
}
