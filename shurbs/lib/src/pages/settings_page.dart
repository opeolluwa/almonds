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
                  color: const Color(0xFF7C3AED),
                  onTap: () => go(const AppearanceSettingsPage()),
                ),
                _SettingsItem(
                  icon: HeroIcons.language,
                  label: 'Locale',
                  subtitle: 'Language, region & date format',
                  color: const Color(0xFF0891B2),
                  onTap: () => go(const LocaleSettingsPage()),
                ),
                _SettingsItem(
                  icon: HeroIcons.briefcase,
                  label: 'Workspaces',
                  subtitle: 'Manage your workspaces',
                  color: const Color(0xFF059669),
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
                  icon: HeroIcons.cpuChip,
                  label: 'AI & Ollama',
                  subtitle: 'Local model & assistant config',
                  color: const Color(0xFFD02752),
                  onTap: () => go(const AiSettingsPage()),
                ),
                _SettingsItem(
                  icon: HeroIcons.bell,
                  label: 'Notifications',
                  subtitle: 'Alerts, badges & sounds',
                  color: const Color(0xFFEA580C),
                  onTap: () => go(const NotificationsSettingsPage()),
                ),
                _SettingsItem(
                  icon: HeroIcons.clock,
                  label: 'Alarm',
                  subtitle: 'Default ringtone & snooze',
                  color: const Color(0xFFCA8A04),
                  onTap: () => go(const AlarmSettingsPage()),
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
                  color: const Color(0xFF2563EB),
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
                  color: const Color(0xFF6B7280),
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

    return Material(
      color: Colors.transparent,
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(16),
        child: Ink(
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(16),
            gradient: LinearGradient(
              colors: isDark
                  ? [const Color(0xFF1a1030), const Color(0xFF0d1220)]
                  : [colorScheme.primaryContainer, colorScheme.primaryContainer.withValues(alpha: 0.5)],
              begin: Alignment.topLeft,
              end: Alignment.bottomRight,
            ),
            border: Border.all(
              color: isDark
                  ? const Color(0xFF2a1a4a)
                  : colorScheme.primary.withValues(alpha: 0.2),
            ),
          ),
          padding: const EdgeInsets.all(20),
          child: Row(
            children: [
              CircleAvatar(
                radius: 30,
                backgroundColor: colorScheme.primary,
                child: Text(
                  'A',
                  style: TextStyle(
                    color: colorScheme.onPrimary,
                    fontWeight: FontWeight.bold,
                    fontSize: 22,
                  ),
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      'Adeoye',
                      style: theme.textTheme.titleMedium?.copyWith(
                        fontWeight: FontWeight.bold,
                        color: isDark ? Colors.white : colorScheme.onPrimaryContainer,
                      ),
                    ),
                    const SizedBox(height: 2),
                    Text(
                      'adeoye@example.com',
                      style: theme.textTheme.bodySmall?.copyWith(
                        color: isDark
                            ? const Color(0xFF8a9dc6)
                            : colorScheme.onPrimaryContainer.withValues(alpha: 0.7),
                      ),
                    ),
                  ],
                ),
              ),
              HeroIcon(
                HeroIcons.chevronRight,
                size: 18,
                color: isDark ? const Color(0xFF8a9dc6) : colorScheme.onPrimaryContainer.withValues(alpha: 0.5),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

// ── Section label ─────────────────────────────────────────────────────────────

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
                color: item.color.withValues(alpha: isDark ? 0.18 : 0.12),
                borderRadius: BorderRadius.circular(9),
              ),
              child: Center(
                child: HeroIcon(item.icon, size: 18, color: item.color),
              ),
            ),
            const SizedBox(width: 14),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(item.label, style: theme.textTheme.bodyLarge?.copyWith(fontWeight: FontWeight.w500)),
                  if (item.subtitle != null) ...[
                    const SizedBox(height: 1),
                    Text(
                      item.subtitle!,
                      style: theme.textTheme.bodySmall?.copyWith(
                        color: colorScheme.onSurfaceVariant,
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
