import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

import '../../theme_notifier.dart';

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
                const SizedBox(height: 16),

                // Accent color card
                ValueListenableBuilder<AccentSwatch>(
                  valueListenable: accentColorNotifier,
                  builder: (_, selectedAccent, __) => Card(
                    child: Padding(
                      padding: const EdgeInsets.all(16),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Row(
                            children: [
                              Container(
                                width: 36,
                                height: 36,
                                decoration: BoxDecoration(
                                  color: colorScheme.primary.withValues(alpha: 0.12),
                                  borderRadius: BorderRadius.circular(8),
                                ),
                                child: Center(child: HeroIcon(HeroIcons.swatch, size: 18, color: colorScheme.primary)),
                              ),
                              const SizedBox(width: 14),
                              Column(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                children: [
                                  Text('Accent color', style: theme.textTheme.bodyMedium),
                                  Text('Primary highlight color', style: theme.textTheme.bodySmall),
                                ],
                              ),
                            ],
                          ),
                          const SizedBox(height: 16),
                          Row(
                            mainAxisAlignment: MainAxisAlignment.spaceBetween,
                            children: accentSwatches.map((swatch) {
                              final isSelected = selectedAccent.label == swatch.label;
                              return GestureDetector(
                                onTap: () => accentColorNotifier.value = swatch,
                                child: AnimatedContainer(
                                  duration: const Duration(milliseconds: 180),
                                  width: 40,
                                  height: 40,
                                  decoration: BoxDecoration(
                                    color: swatch.primary,
                                    shape: BoxShape.circle,
                                    border: isSelected
                                        ? Border.all(color: Colors.white, width: 2.5)
                                        : null,
                                    boxShadow: isSelected
                                        ? [BoxShadow(color: swatch.primary.withValues(alpha: 0.5), blurRadius: 10, spreadRadius: 1)]
                                        : null,
                                  ),
                                  child: isSelected
                                      ? const Icon(Icons.check_rounded, color: Colors.white, size: 18)
                                      : null,
                                ),
                              );
                            }).toList(),
                          ),
                          const SizedBox(height: 12),
                          Center(
                            child: Text(
                              selectedAccent.label,
                              style: theme.textTheme.labelMedium?.copyWith(
                                color: selectedAccent.primary,
                                fontWeight: FontWeight.w600,
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
                const SizedBox(height: 32),
        ],
      ),
    );
  }
}
