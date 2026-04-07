import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';
import 'package:url_launcher/url_launcher.dart';


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
      appBar: AppBar(
        title: const Text('About', style: TextStyle(color: Colors.black)),
        foregroundColor: Colors.black,
      ),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
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
                      child: Center(child: HeroIcon(HeroIcons.codeBracket, size: 18, color: colorScheme.primary)),
                    ),
                    title: const Text('Built openly on GitHub'),
                    subtitle: const Text('MIT License · github.com/opeolluwa/almonds'),
                    trailing: HeroIcon(HeroIcons.arrowTopRightOnSquare, size: 16, color: colorScheme.onSurfaceVariant),
                    onTap: () => launchUrl(
                      Uri.parse('https://github.com/opeolluwa/almonds'),
                      mode: LaunchMode.externalApplication,
                    ),
                  ),
                ),
                const SizedBox(height: 32),
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
