import 'package:flutter/material.dart';

import '../theme_notifier.dart';

class SettingsPage extends StatefulWidget {
  const SettingsPage({super.key});

  @override
  State<SettingsPage> createState() => _SettingsPageState();
}

class _SettingsPageState extends State<SettingsPage> {
  bool get _darkMode => themeModeNotifier.value == ThemeMode.dark;
  bool _notifications = true;
  bool _alarmSound = true;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Scaffold(
      body: SafeArea(
        child: CustomScrollView(
          slivers: [
            const SliverAppBar(pinned: true, title: Text('Settings')),
            SliverPadding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              sliver: SliverList(
                delegate: SliverChildListDelegate([
                  // Profile card
                  Card(
                    child: Padding(
                      padding: const EdgeInsets.all(16),
                      child: Row(
                        children: [
                          CircleAvatar(
                            radius: 32,
                            backgroundColor: colorScheme.primaryContainer,
                            child: Icon(Icons.person, size: 32, color: colorScheme.onPrimaryContainer),
                          ),
                          const SizedBox(width: 16),
                          Expanded(
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Text('User', style: theme.textTheme.titleMedium?.copyWith(fontWeight: FontWeight.bold)),
                                Text('user@wildalmonds.app', style: theme.textTheme.bodySmall),
                              ],
                            ),
                          ),
                          TextButton(onPressed: () {}, child: const Text('Edit')),
                        ],
                      ),
                    ),
                  ),
                  const SizedBox(height: 24),

                  // Appearance
                  _SectionHeader(title: 'Appearance'),
                  Card(
                    child: Column(
                      children: [
                        SwitchListTile(
                          title: const Text('Dark Mode'),
                          secondary: const Icon(Icons.dark_mode_outlined),
                          value: _darkMode,
                          onChanged: (v) => setState(() {
                            themeModeNotifier.value = v ? ThemeMode.dark : ThemeMode.light;
                          }),
                        ),
                      ],
                    ),
                  ),
                  const SizedBox(height: 16),

                  // Notifications
                  _SectionHeader(title: 'Notifications'),
                  Card(
                    child: Column(
                      children: [
                        SwitchListTile(
                          title: const Text('Push Notifications'),
                          secondary: const Icon(Icons.notifications_outlined),
                          value: _notifications,
                          onChanged: (v) => setState(() => _notifications = v),
                        ),
                        const Divider(height: 1),
                        SwitchListTile(
                          title: const Text('Alarm Sound'),
                          secondary: const Icon(Icons.volume_up_outlined),
                          value: _alarmSound,
                          onChanged: (v) => setState(() => _alarmSound = v),
                        ),
                      ],
                    ),
                  ),
                  const SizedBox(height: 16),

                  // Data
                  _SectionHeader(title: 'Data'),
                  Card(
                    child: Column(
                      children: [
                        ListTile(
                          leading: const Icon(Icons.upload_outlined),
                          title: const Text('Export Data'),
                          trailing: const Icon(Icons.chevron_right),
                          onTap: () {},
                        ),
                        const Divider(height: 1),
                        ListTile(
                          leading: const Icon(Icons.download_outlined),
                          title: const Text('Import Data'),
                          trailing: const Icon(Icons.chevron_right),
                          onTap: () {},
                        ),
                        const Divider(height: 1),
                        ListTile(
                          leading: Icon(Icons.delete_forever_outlined, color: colorScheme.error),
                          title: Text('Clear All Data', style: TextStyle(color: colorScheme.error)),
                          onTap: () => _confirmClearData(context),
                        ),
                      ],
                    ),
                  ),
                  const SizedBox(height: 16),

                  // About
                  _SectionHeader(title: 'About'),
                  Card(
                    child: Column(
                      children: [
                        ListTile(
                          leading: const Icon(Icons.info_outline),
                          title: const Text('Version'),
                          trailing: Text('1.0.0', style: theme.textTheme.bodySmall),
                        ),
                        const Divider(height: 1),
                        ListTile(
                          leading: const Icon(Icons.code),
                          title: const Text('Wild Almonds'),
                          trailing: const Icon(Icons.chevron_right),
                          onTap: () {},
                        ),
                      ],
                    ),
                  ),
                  const SizedBox(height: 32),
                ]),
              ),
            ),
          ],
        ),
      ),
    );
  }

  void _confirmClearData(BuildContext context) {
    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        title: const Text('Clear All Data?'),
        content: const Text('This will permanently delete all your todos, alarms, and bookmarks. This cannot be undone.'),
        actions: [
          TextButton(onPressed: () => Navigator.pop(ctx), child: const Text('Cancel')),
          FilledButton(
            style: FilledButton.styleFrom(backgroundColor: Theme.of(ctx).colorScheme.error),
            onPressed: () => Navigator.pop(ctx),
            child: const Text('Clear'),
          ),
        ],
      ),
    );
  }
}

class _SectionHeader extends StatelessWidget {
  final String title;

  const _SectionHeader({required this.title});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(left: 4, bottom: 8),
      child: Text(
        title,
        style: Theme.of(context).textTheme.labelLarge?.copyWith(
          color: Theme.of(context).colorScheme.primary,
        ),
      ),
    );
  }
}
