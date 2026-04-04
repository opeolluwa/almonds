import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

import 'settings_header_bg.dart';

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
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    final items = [
      _NotifItem(
        icon: HeroIcons.checkCircle,
        label: 'Tasks due',
        desc: 'Remind me when tasks are due',
        value: _tasksDue,
        onChanged: (v) => setState(() => _tasksDue = v),
      ),
      _NotifItem(
        icon: HeroIcons.bell,
        label: 'Reminders',
        desc: 'Calendar event reminders',
        value: _reminders,
        onChanged: (v) => setState(() => _reminders = v),
      ),
      _NotifItem(
        icon: HeroIcons.arrowPath,
        label: 'Sync complete',
        desc: 'Notify when sync finishes',
        value: _syncComplete,
        onChanged: (v) => setState(() => _syncComplete = v),
      ),
      _NotifItem(
        icon: HeroIcons.sparkles,
        label: 'App updates',
        desc: 'Notify about new versions',
        value: _appUpdates,
        onChanged: (v) => setState(() => _appUpdates = v),
      ),
    ];

    return Scaffold(
      body: CustomScrollView(
        slivers: [
          SliverAppBar(
            expandedHeight: 180,
            pinned: true,
            flexibleSpace: FlexibleSpaceBar(
              title: const Text(
                'Notifications',
                style: TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 16),
              ),
              titlePadding: const EdgeInsets.only(left: 56, bottom: 16),
              background: SettingsHeaderBackground(
                colors: [colorScheme.primary, colorScheme.primaryContainer],
                child: SafeArea(
                  child: Align(
                    alignment: Alignment.center,
                    child: Container(
                      width: 64,
                      height: 64,
                      decoration: BoxDecoration(
                        color: Colors.white.withValues(alpha: 0.15),
                        borderRadius: BorderRadius.circular(18),
                      ),
                      child: const Center(
                        child: HeroIcon(HeroIcons.bell, size: 30, color: Colors.white),
                      ),
                    ),
                  ),
                ),
              ),
            ),
          ),
          SliverPadding(
            padding: const EdgeInsets.all(16),
            sliver: SliverList(
              delegate: SliverChildListDelegate([
                Card(
                  child: Padding(
                    padding: const EdgeInsets.symmetric(vertical: 8),
                    child: Column(
                      children: [
                        for (int i = 0; i < items.length; i++) ...[
                          if (i > 0) const Divider(height: 1, indent: 16, endIndent: 16),
                          ListTile(
                            leading: Container(
                              width: 36,
                              height: 36,
                              decoration: BoxDecoration(
                                color: colorScheme.primary.withValues(alpha: 0.12),
                                borderRadius: BorderRadius.circular(8),
                              ),
                              child: Center(child: HeroIcon(items[i].icon, size: 18, color: colorScheme.primary)),
                            ),
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
                const SizedBox(height: 32),
              ]),
            ),
          ),
        ],
      ),
    );
  }
}

class _NotifItem {
  final HeroIcons icon;
  final String label;
  final String desc;
  final bool value;
  final ValueChanged<bool> onChanged;
  const _NotifItem({
    required this.icon,
    required this.label,
    required this.desc,
    required this.value,
    required this.onChanged,
  });
}
