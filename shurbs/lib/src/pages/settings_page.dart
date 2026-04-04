import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

import 'settings/profile_settings_page.dart';
import 'settings/appearance_settings_page.dart';
import 'settings/locale_settings_page.dart';
import 'settings/workspaces_settings_page.dart';
import 'settings/backup_settings_page.dart';
import 'settings/ai_settings_page.dart';
import 'settings/notifications_settings_page.dart';
import 'settings/alarm_settings_page.dart';
import 'settings/about_settings_page.dart';

class SettingsPage extends StatelessWidget {
  const SettingsPage({super.key});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    void go(Widget page) {
      Navigator.push(context, MaterialPageRoute(builder: (_) => page));
    }

    final sections = [
      _SettingsItem(icon: HeroIcons.user, label: 'Profile', onTap: () => go(const ProfileSettingsPage())),
      _SettingsItem(icon: HeroIcons.pencil, label: 'Appearance', onTap: () => go(const AppearanceSettingsPage())),
      _SettingsItem(icon: HeroIcons.language, label: 'Locale', onTap: () => go(const LocaleSettingsPage())),
      _SettingsItem(icon: HeroIcons.briefcase, label: 'Workspaces', onTap: () => go(const WorkspacesSettingsPage())),
      _SettingsItem(icon: HeroIcons.cloudArrowUp, label: 'Backup & Sync', onTap: () => go(const BackupSettingsPage())),
      _SettingsItem(icon: HeroIcons.cpuChip, label: 'AI & Ollama', onTap: () => go(const AiSettingsPage())),
      _SettingsItem(icon: HeroIcons.inboxArrowDown, label: 'Notifications', onTap: () => go(const NotificationsSettingsPage())),
      _SettingsItem(icon: HeroIcons.bell, label: 'Alarm', onTap: () => go(const AlarmSettingsPage())),
      _SettingsItem(icon: HeroIcons.informationCircle, label: 'About', onTap: () => go(const AboutSettingsPage())),
    ];

    return Scaffold(
      body: SafeArea(
        child: ListView(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
          children: [
            Card(
              child: Column(
                children: [
                  for (int i = 0; i < sections.length; i++) ...[
                    if (i > 0) const Divider(height: 1, indent: 56),
                    _SettingsTile(
                      item: sections[i],
                      colorScheme: colorScheme,
                      theme: theme,
                      isFirst: i == 0,
                      isLast: i == sections.length - 1,
                    ),
                  ],
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _SettingsTile extends StatelessWidget {
  final _SettingsItem item;
  final ColorScheme colorScheme;
  final ThemeData theme;
  final bool isFirst;
  final bool isLast;

  const _SettingsTile({
    required this.item,
    required this.colorScheme,
    required this.theme,
    required this.isFirst,
    required this.isLast,
  });

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: item.onTap,
      borderRadius: BorderRadius.vertical(
        top: isFirst ? const Radius.circular(12) : Radius.zero,
        bottom: isLast ? const Radius.circular(12) : Radius.zero,
      ),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
        child: Row(
          children: [
            HeroIcon(item.icon, size: 20, color: colorScheme.onSurfaceVariant),
            const SizedBox(width: 18),
            Expanded(child: Text(item.label, style: theme.textTheme.bodyLarge)),
            HeroIcon(HeroIcons.chevronRight, size: 20, color: colorScheme.onSurfaceVariant),
          ],
        ),
      ),
    );
  }
}

class _SettingsItem {
  final HeroIcons icon;
  final String label;
  final VoidCallback onTap;
  const _SettingsItem({required this.icon, required this.label, required this.onTap});
}
