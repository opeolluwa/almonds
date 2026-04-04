import 'package:flutter/material.dart';

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

  static const _accentColors = [
    _AccentOption(label: 'Rose', color: Color(0xFFe11d48)),
    _AccentOption(label: 'Emerald', color: Color(0xFF10b981)),
    _AccentOption(label: 'Sky', color: Color(0xFF0ea5e9)),
    _AccentOption(label: 'Amber', color: Color(0xFFf59e0b)),
  ];
  int _selectedAccent = 0;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Scaffold(
      appBar: AppBar(title: const Text('Appearance')),
      body: SafeArea(
        child: ListView(
          padding: const EdgeInsets.all(16),
          children: [
            Card(
              child: Padding(
                padding: const EdgeInsets.symmetric(vertical: 8),
                child: Column(
                  children: [
                    // Dark mode
                    ListTile(
                      title: const Text('Dark mode'),
                      subtitle: const Text('Switch between light and dark theme'),
                      trailing: Transform.scale(
                        scale: 0.8,
                        child: ValueListenableBuilder(
                          valueListenable: themeModeNotifier,
                          builder: (_, __, ___) => Switch(
                            value: _isDark,
                            onChanged: (v) => setState(() {
                              themeModeNotifier.value = v ? ThemeMode.dark : ThemeMode.light;
                            }),
                          ),
                        ),
                      ),
                    ),
                    const Divider(height: 1, indent: 16, endIndent: 16),

                    // Font size
                    Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                      child: Row(
                        children: [
                          Expanded(
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Text('Font size', style: theme.textTheme.bodyMedium),
                                const SizedBox(height: 2),
                                Text('Adjust text size across the app', style: theme.textTheme.bodySmall),
                              ],
                            ),
                          ),
                          Container(
                            decoration: BoxDecoration(
                              color: theme.colorScheme.surfaceContainerHighest,
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
                                      color: selected ? theme.colorScheme.surface : Colors.transparent,
                                      borderRadius: BorderRadius.circular(6),
                                      boxShadow: selected
                                          ? [BoxShadow(color: Colors.black.withValues(alpha: 0.1), blurRadius: 2)]
                                          : null,
                                    ),
                                    child: Text(
                                      sz.toUpperCase(),
                                      style: theme.textTheme.labelSmall?.copyWith(
                                        fontWeight: FontWeight.w600,
                                        color: selected
                                            ? theme.colorScheme.onSurface
                                            : theme.colorScheme.onSurfaceVariant,
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
                    const Divider(height: 1, indent: 16, endIndent: 16),

                    // Accent color
                    Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                      child: Row(
                        children: [
                          Expanded(
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Text('Accent color', style: theme.textTheme.bodyMedium),
                                const SizedBox(height: 2),
                                Text('Primary highlight color', style: theme.textTheme.bodySmall),
                              ],
                            ),
                          ),
                          Row(
                            children: List.generate(_accentColors.length, (i) {
                              final selected = _selectedAccent == i;
                              return Padding(
                                padding: const EdgeInsets.only(left: 8),
                                child: GestureDetector(
                                  onTap: () => setState(() => _selectedAccent = i),
                                  child: AnimatedContainer(
                                    duration: const Duration(milliseconds: 150),
                                    width: 26,
                                    height: 26,
                                    decoration: BoxDecoration(
                                      color: _accentColors[i].color,
                                      shape: BoxShape.circle,
                                      border: selected
                                          ? Border.all(color: _accentColors[i].color.withValues(alpha: 0.4), width: 3)
                                          : null,
                                      boxShadow: selected
                                          ? [BoxShadow(color: _accentColors[i].color.withValues(alpha: 0.4), blurRadius: 6)]
                                          : null,
                                    ),
                                  ),
                                ),
                              );
                            }),
                          ),
                        ],
                      ),
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

class _AccentOption {
  final String label;
  final Color color;
  const _AccentOption({required this.label, required this.color});
}
