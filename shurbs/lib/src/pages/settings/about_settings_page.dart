import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

import 'settings_header_bg.dart';

class AboutSettingsPage extends StatelessWidget {
  const AboutSettingsPage({super.key});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    const info = [
      _InfoItem(label: 'Version', value: '1.0.0', mono: true),
      _InfoItem(label: 'Build', value: '2026.04.04', mono: true),
      _InfoItem(label: 'License', value: 'MIT', mono: false),
      _InfoItem(label: 'Platform', value: 'Android / iOS', mono: false),
    ];

    return Scaffold(
      body: CustomScrollView(
        slivers: [
          SliverAppBar(
            expandedHeight: 200,
            pinned: true,
            flexibleSpace: FlexibleSpaceBar(
              background: SettingsHeaderBackground(
                colors: [colorScheme.primary, colorScheme.primaryContainer],
                child: SafeArea(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      const SizedBox(height: 32),
                      Container(
                        width: 72,
                        height: 72,
                        decoration: BoxDecoration(
                          color: Colors.white.withValues(alpha: 0.15),
                          borderRadius: BorderRadius.circular(20),
                        ),
                        child: const Center(
                          child: HeroIcon(HeroIcons.sparkles, size: 34, color: Colors.white),
                        ),
                      ),
                      const SizedBox(height: 12),
                      const Text(
                        'Wild Almonds',
                        style: TextStyle(
                          color: Colors.white,
                          fontWeight: FontWeight.bold,
                          fontSize: 18,
                        ),
                      ),
                      const SizedBox(height: 4),
                      Text(
                        'Your personal productivity suite',
                        style: TextStyle(
                          color: Colors.white.withValues(alpha: 0.75),
                          fontSize: 13,
                        ),
                      ),
                    ],
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
                        ...info.map((item) => Column(
                          children: [
                            if (item != info.first)
                              const Divider(height: 1, indent: 16, endIndent: 16),
                            ListTile(
                              title: Text(
                                item.label,
                                style: theme.textTheme.bodyMedium?.copyWith(
                                  color: colorScheme.onSurfaceVariant,
                                ),
                              ),
                              trailing: Text(
                                item.value,
                                style: item.mono
                                    ? theme.textTheme.bodyMedium?.copyWith(fontFamily: 'monospace')
                                    : theme.textTheme.bodyMedium,
                              ),
                            ),
                          ],
                        )),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 12),
                Card(
                  child: ListTile(
                    leading: Container(
                      width: 36,
                      height: 36,
                      decoration: BoxDecoration(
                        color: colorScheme.primary.withValues(alpha: 0.12),
                        borderRadius: BorderRadius.circular(8),
                      ),
                      child: Center(child: HeroIcon(HeroIcons.arrowPath, size: 18, color: colorScheme.primary)),
                    ),
                    title: const Text('Check for updates'),
                    subtitle: const Text('You are on the latest version'),
                    trailing: HeroIcon(HeroIcons.chevronRight, size: 16, color: colorScheme.onSurfaceVariant),
                    onTap: () {},
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

class _InfoItem {
  final String label;
  final String value;
  final bool mono;
  const _InfoItem({required this.label, required this.value, required this.mono});
}
