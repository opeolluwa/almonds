import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

import 'settings/profile_settings_page.dart';
import '../profile_notifier.dart';
import 'settings/appearance_settings_page.dart';
import 'settings/locale_settings_page.dart';
import 'settings/workspaces_settings_page.dart';
import 'settings/backup_settings_page.dart';
import 'settings/notifications_settings_page.dart';
import 'settings/reminder_settings_page.dart';
import 'settings/about_settings_page.dart';

class SettingsPage extends StatelessWidget {
  const SettingsPage({super.key});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;
    final isDark = theme.brightness == Brightness.dark;

    void go(Widget page) {
      Navigator.push(context, MaterialPageRoute(builder: (_) => page));
    }

    return Scaffold(
      body: SafeArea(
        child: ListView(
          padding: const EdgeInsets.fromLTRB(16, 4, 16, 32),
          children: [
            // ── Profile card ────────────────────────────────────────────────
            _ProfileCard(onTap: () => go(const ProfileSettingsPage())),
            const SizedBox(height: 24),

            // ── Customise ───────────────────────────────────────────────────
            _SectionLabel('Customise'),
            _SettingsGroup(
              isDark: isDark,
              colorScheme: colorScheme,
              theme: theme,
              items: [
                _SettingsItem(
                  icon: HeroIcons.paintBrush,
                  label: 'Appearance',
                  subtitle: 'Theme, colors & font size',
                  color: colorScheme.primary,
                  onTap: () => go(const AppearanceSettingsPage()),
                ),
                _SettingsItem(
                  icon: HeroIcons.language,
                  label: 'Locale',
                  subtitle: 'Language, region & date format',
                  color: colorScheme.primary,
                  onTap: () => go(const LocaleSettingsPage()),
                ),
                _SettingsItem(
                  icon: HeroIcons.briefcase,
                  label: 'Workspaces',
                  subtitle: 'Manage your workspaces',
                  color: colorScheme.primary,
                  onTap: () => go(const WorkspacesSettingsPage()),
                ),
              ],
            ),
            const SizedBox(height: 20),

            // ── Features ────────────────────────────────────────────────────
            _SectionLabel('Features'),
            _SettingsGroup(
              isDark: isDark,
              colorScheme: colorScheme,
              theme: theme,
              items: [
                _SettingsItem(
                  icon: HeroIcons.bell,
                  label: 'Notifications',
                  subtitle: 'Alerts, badges & sounds',
                  color: colorScheme.primary,
                  onTap: () => go(const NotificationsSettingsPage()),
                ),
                _SettingsItem(
                  icon: HeroIcons.clock,
                  label: 'Reminder',
                  subtitle: 'Default ringtone & snooze',
                  color: colorScheme.primary,
                  onTap: () => go(const ReminderSettingsPage()),
                ),
              ],
            ),
            const SizedBox(height: 20),

            // ── Data ────────────────────────────────────────────────────────
            _SectionLabel('Data'),
            _SettingsGroup(
              isDark: isDark,
              colorScheme: colorScheme,
              theme: theme,
              items: [
                _SettingsItem(
                  icon: HeroIcons.cloudArrowUp,
                  label: 'Backup & Sync',
                  subtitle: 'Export, import & cloud sync',
                  color: colorScheme.primary,
                  onTap: () => go(const BackupSettingsPage()),
                ),
              ],
            ),
            const SizedBox(height: 20),

            // ── About ───────────────────────────────────────────────────────
            _SectionLabel('Support'),
            _SettingsGroup(
              isDark: isDark,
              colorScheme: colorScheme,
              theme: theme,
              items: [
                _SettingsItem(
                  icon: HeroIcons.informationCircle,
                  label: 'About',
                  subtitle: 'Version, changelog & credits',
                  color: colorScheme.primary,
                  onTap: () => go(const AboutSettingsPage()),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

// ── Profile card ──────────────────────────────────────────────────────────────

class _ProfileCard extends StatelessWidget {
  final VoidCallback onTap;
  const _ProfileCard({required this.onTap});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;
    final isDark = theme.brightness == Brightness.dark;

    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(16),
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 28, horizontal: 20),
        child: ListenableBuilder(
            listenable: ProfileNotifier.instance,
            builder: (_, __) {
              final profile = ProfileNotifier.instance;
              return Column(
                children: [
                  Stack(
                    children: [
                      CircleAvatar(
                        radius: 48,
                        backgroundColor: colorScheme.primary,
                        child: Text(
                          profile.initials,
                          style: TextStyle(
                            color: colorScheme.onPrimary,
                            fontWeight: FontWeight.bold,
                            fontSize: 36,
                          ),
                        ),
                      ),
                      Positioned(
                        right: 0,
                        bottom: 0,
                        child: Container(
                          width: 28,
                          height: 28,
                          decoration: BoxDecoration(
                            color: isDark ? const Color(0xFF1a2540) : Colors.white,
                            shape: BoxShape.circle,
                            border: Border.all(
                              color: isDark ? const Color(0xFF2a3550) : colorScheme.outlineVariant,
                              width: 1.5,
                            ),
                          ),
                          child: Icon(
                            Icons.camera_alt_rounded,
                            size: 14,
                            color: isDark ? const Color(0xFF8a9dc6) : Colors.black87,
                          ),
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 14),
                  Text(
                    profile.fullName.isNotEmpty ? profile.fullName : 'Not set',
                    style: theme.textTheme.titleLarge?.copyWith(
                      fontWeight: FontWeight.bold,
                      color: isDark ? Colors.white : Colors.black87,
                    ),
                  ),
                  const SizedBox(height: 3),
                  Text(
                    profile.email.isNotEmpty ? profile.email : 'Tap to set up profile',
                    style: theme.textTheme.bodySmall?.copyWith(
                      color: isDark ? const Color(0xFF8a9dc6) : Colors.black54,
                    ),
                  ),
                ],
              );
            },
          ),
        ),
      );
  }
}

// ── Section label ──────────────────────────────────────────────────────────────

class _SectionLabel extends StatelessWidget {
  final String text;
  const _SectionLabel(this.text);

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(left: 4, bottom: 8),
      child: Text(
        text.toUpperCase(),
        style: Theme.of(context).textTheme.labelSmall?.copyWith(
              color: Theme.of(context).colorScheme.onSurfaceVariant,
              letterSpacing: 1.1,
              fontWeight: FontWeight.w600,
            ),
      ),
    );
  }
}

// ── Settings group (card) ─────────────────────────────────────────────────────

class _SettingsGroup extends StatelessWidget {
  final List<_SettingsItem> items;
  final bool isDark;
  final ColorScheme colorScheme;
  final ThemeData theme;

  const _SettingsGroup({
    required this.items,
    required this.isDark,
    required this.colorScheme,
    required this.theme,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: EdgeInsets.zero,
      child: Column(
        children: [
          for (int i = 0; i < items.length; i++) ...[
            if (i > 0)
              Divider(
                height: 1,
                indent: 64,
                color: isDark ? const Color(0xFF1a2540) : null,
              ),
            _SettingsTile(
              item: items[i],
              isDark: isDark,
              colorScheme: colorScheme,
              theme: theme,
              isFirst: i == 0,
              isLast: i == items.length - 1,
            ),
          ],
        ],
      ),
    );
  }
}

// ── Tile ──────────────────────────────────────────────────────────────────────

class _SettingsTile extends StatelessWidget {
  final _SettingsItem item;
  final bool isDark;
  final ColorScheme colorScheme;
  final ThemeData theme;
  final bool isFirst;
  final bool isLast;

  const _SettingsTile({
    required this.item,
    required this.isDark,
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
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        child: Row(
          children: [
            Container(
              width: 36,
              height: 36,
              decoration: BoxDecoration(
                color: isDark
                    ? item.color.withValues(alpha: 0.18)
                    : Colors.black.withValues(alpha: 0.06),
                borderRadius: BorderRadius.circular(9),
              ),
              child: Center(
                child: HeroIcon(
                  item.icon,
                  size: 18,
                  color: isDark ? item.color : Colors.black87,
                ),
              ),
            ),
            const SizedBox(width: 14),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    item.label,
                    style: theme.textTheme.bodyLarge?.copyWith(
                      fontWeight: FontWeight.w500,
                      color: isDark ? null : Colors.black87,
                    ),
                  ),
                  if (item.subtitle != null) ...[
                    const SizedBox(height: 1),
                    Text(
                      item.subtitle!,
                      style: theme.textTheme.bodySmall?.copyWith(
                        color: isDark ? colorScheme.onSurfaceVariant : Colors.black45,
                      ),
                    ),
                  ],
                ],
              ),
            ),
            HeroIcon(HeroIcons.chevronRight, size: 16, color: colorScheme.onSurfaceVariant.withValues(alpha: 0.5)),
          ],
        ),
      ),
    );
  }
}

// ── Model ─────────────────────────────────────────────────────────────────────

class _SettingsItem {
  final HeroIcons icon;
  final String label;
  final String? subtitle;
  final Color color;
  final VoidCallback onTap;

  const _SettingsItem({
    required this.icon,
    required this.label,
    this.subtitle,
    required this.color,
    required this.onTap,
  });
}
