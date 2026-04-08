import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

import '../../theme_notifier.dart' show themeModeNotifier;

class AppearanceSettingsPage extends StatefulWidget {
  const AppearanceSettingsPage({super.key});

  @override
  State<AppearanceSettingsPage> createState() => _AppearanceSettingsPageState();
}

class _AppearanceSettingsPageState extends State<AppearanceSettingsPage> {
  bool get _isDark => themeModeNotifier.value == ThemeMode.dark;
  String _fontSize = 'md';
  static const _fontSizes = ['sm', 'md', 'lg'];

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Scaffold(
      appBar: AppBar(
        title: const Text('Appearance', style: TextStyle(color: Colors.black)),
        foregroundColor: Colors.black,
      ),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
                // Dark mode + Font size card
                Card(
                  child: Padding(
                    padding: const EdgeInsets.symmetric(vertical: 8),
                    child: Column(
                      children: [
                        ValueListenableBuilder<ThemeMode>(
                          valueListenable: themeModeNotifier,
                          builder: (_, __, ___) => ListTile(
                            leading: Container(
                              width: 36,
                              height: 36,
                              decoration: BoxDecoration(
                                color: colorScheme.primary.withValues(alpha: 0.12),
                                borderRadius: BorderRadius.circular(8),
                              ),
                              child: Center(child: HeroIcon(HeroIcons.moon, size: 18, color: colorScheme.primary)),
                            ),
                            title: const Text('Dark mode'),
                            subtitle: const Text('Switch between light and dark theme'),
                            trailing: Transform.scale(
                              scale: 0.8,
                              child: Switch(
                                value: _isDark,
                                onChanged: (v) => setState(() {
                                  themeModeNotifier.value = v ? ThemeMode.dark : ThemeMode.light;
                                }),
                              ),
                            ),
                          ),
                        ),
                        const Divider(height: 1, indent: 16, endIndent: 16),
                        Padding(
                          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
                          child: Row(
                            children: [
                              Container(
                                width: 36,
                                height: 36,
                                decoration: BoxDecoration(
                                  color: colorScheme.primary.withValues(alpha: 0.12),
                                  borderRadius: BorderRadius.circular(8),
                                ),
                                child: Center(child: HeroIcon(HeroIcons.adjustmentsHorizontal, size: 18, color: colorScheme.primary)),
                              ),
                              const SizedBox(width: 14),
                              Expanded(
                                child: Column(
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  children: [
                                    Text('Font size', style: theme.textTheme.bodyMedium),
                                    Text('Adjust text size across the app', style: theme.textTheme.bodySmall),
                                  ],
                                ),
                              ),
                              Container(
                                decoration: BoxDecoration(
                                  color: colorScheme.surfaceContainerHighest,
                                  borderRadius: BorderRadius.circular(8),
                                ),
                                padding: const EdgeInsets.all(3),
                                child: Row(
                                  children: _fontSizes.map((sz) {
                                    final selected = _fontSize == sz;
                                    return GestureDetector(
                                      onTap: () => setState(() => _fontSize = sz),
                                      child: AnimatedContainer(
                                        duration: const Duration(milliseconds: 150),
                                        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                                        decoration: BoxDecoration(
                                          color: selected ? colorScheme.surface : Colors.transparent,
                                          borderRadius: BorderRadius.circular(6),
                                          boxShadow: selected
                                              ? [BoxShadow(color: Colors.black.withValues(alpha: 0.1), blurRadius: 2)]
                                              : null,
                                        ),
                                        child: Text(
                                          sz.toUpperCase(),
                                          style: theme.textTheme.labelSmall?.copyWith(
                                            fontWeight: FontWeight.w600,
                                            color: selected ? colorScheme.onSurface : colorScheme.onSurfaceVariant,
                                          ),
                                        ),
                                      ),
                                    );
                                  }).toList(),
                                ),
                              ),
                            ],
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 32),
        ],
      ),
    );
  }
}
