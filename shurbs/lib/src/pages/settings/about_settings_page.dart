import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

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
      appBar: AppBar(title: const Text('About')),
      body: SafeArea(
        child: ListView(
          padding: const EdgeInsets.all(16),
          children: [
            Card(
              child: Padding(
                padding: const EdgeInsets.all(20),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      children: [
                        Container(
                          width: 44,
                          height: 44,
                          decoration: BoxDecoration(
                            color: colorScheme.primaryContainer,
                            borderRadius: BorderRadius.circular(10),
                          ),
                          child: Center(
                            child: HeroIcon(HeroIcons.sparkles, color: colorScheme.onPrimaryContainer, size: 22),
                          ),
                        ),
                        const SizedBox(width: 14),
                        Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text('Wild Almonds', style: theme.textTheme.titleMedium?.copyWith(fontWeight: FontWeight.bold)),
                            Text('Your personal productivity suite', style: theme.textTheme.bodySmall),
                          ],
                        ),
                      ],
                    ),
                    const SizedBox(height: 20),
                    ...info.map((item) => Column(
                      children: [
                        Padding(
                          padding: const EdgeInsets.symmetric(vertical: 10),
                          child: Row(
                            mainAxisAlignment: MainAxisAlignment.spaceBetween,
                            children: [
                              Text(item.label, style: theme.textTheme.bodyMedium?.copyWith(color: colorScheme.onSurfaceVariant)),
                              Text(
                                item.value,
                                style: item.mono
                                    ? theme.textTheme.bodyMedium?.copyWith(fontFamily: 'monospace')
                                    : theme.textTheme.bodyMedium,
                              ),
                            ],
                          ),
                        ),
                        if (item != info.last) const Divider(height: 1),
                      ],
                    )),
                    const SizedBox(height: 16),
                    TextButton.icon(
                      onPressed: () {},
                      icon: const HeroIcon(HeroIcons.arrowPath, size: 16),
                      label: const Text('Check for updates'),
                    ),
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

class _InfoItem {
  final String label;
  final String value;
  final bool mono;
  const _InfoItem({required this.label, required this.value, required this.mono});
}
