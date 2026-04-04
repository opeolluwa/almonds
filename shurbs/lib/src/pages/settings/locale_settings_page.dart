import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

import 'settings_header_bg.dart';

class LocaleSettingsPage extends StatefulWidget {
  const LocaleSettingsPage({super.key});

  @override
  State<LocaleSettingsPage> createState() => _LocaleSettingsPageState();
}

class _LocaleSettingsPageState extends State<LocaleSettingsPage> {
  static const _locales = [
    _LocaleOption(code: 'en', label: 'English', flag: '🇺🇸', native: 'English'),
    _LocaleOption(code: 'fr', label: 'French', flag: '🇫🇷', native: 'Français'),
    _LocaleOption(code: 'de', label: 'German', flag: '🇩🇪', native: 'Deutsch'),
    _LocaleOption(code: 'es', label: 'Spanish', flag: '🇪🇸', native: 'Español'),
    _LocaleOption(code: 'pt', label: 'Portuguese', flag: '🇧🇷', native: 'Português'),
    _LocaleOption(code: 'ar', label: 'Arabic', flag: '🇸🇦', native: 'العربية'),
    _LocaleOption(code: 'zh', label: 'Chinese', flag: '🇨🇳', native: '中文'),
    _LocaleOption(code: 'ja', label: 'Japanese', flag: '🇯🇵', native: '日本語'),
  ];

  String _selected = 'en';

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Scaffold(
      body: CustomScrollView(
        slivers: [
          SliverAppBar(
            expandedHeight: 180,
            pinned: true,
            flexibleSpace: FlexibleSpaceBar(
              title: const Text(
                'Locale',
                style: TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 16),
              ),
              titlePadding: const EdgeInsets.only(left: 56, bottom: 16),
              background: SettingsHeaderBackground(
                colors: [colorScheme.primary, colorScheme.primaryContainer],
                child: SafeArea(
                  child: Align(
                    alignment: Alignment.center,
                    child: Container(
                      width: 64,
                      height: 64,
                      decoration: BoxDecoration(
                        color: Colors.white.withValues(alpha: 0.15),
                        borderRadius: BorderRadius.circular(18),
                      ),
                      child: const Center(
                        child: HeroIcon(HeroIcons.globeAlt, size: 30, color: Colors.white),
                      ),
                    ),
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
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Padding(
                          padding: const EdgeInsets.fromLTRB(16, 12, 16, 8),
                          child: Text(
                            'LANGUAGE',
                            style: theme.textTheme.labelSmall?.copyWith(
                              color: colorScheme.primary,
                              fontWeight: FontWeight.w700,
                              letterSpacing: 1.2,
                            ),
                          ),
                        ),
                        ..._locales.map((locale) {
                          final selected = _selected == locale.code;
                          return Column(
                            children: [
                              if (locale != _locales.first)
                                const Divider(height: 1, indent: 16, endIndent: 16),
                              ListTile(
                                leading: Container(
                                  width: 40,
                                  height: 40,
                                  decoration: BoxDecoration(
                                    color: selected
                                        ? colorScheme.primary.withValues(alpha: 0.12)
                                        : colorScheme.surfaceContainerHighest,
                                    borderRadius: BorderRadius.circular(10),
                                  ),
                                  child: Center(
                                    child: Text(locale.flag, style: const TextStyle(fontSize: 20)),
                                  ),
                                ),
                                title: Text(locale.label),
                                subtitle: Text(
                                  locale.native,
                                  style: theme.textTheme.bodySmall?.copyWith(
                                    color: selected ? colorScheme.primary : null,
                                  ),
                                ),
                                trailing: selected
                                    ? Icon(Icons.check_circle_rounded, color: colorScheme.primary, size: 20)
                                    : null,
                                selected: selected,
                                onTap: () => setState(() => _selected = locale.code),
                              ),
                            ],
                          );
                        }),
                        const SizedBox(height: 8),
                      ],
                    ),
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

class _LocaleOption {
  final String code;
  final String label;
  final String flag;
  final String native;
  const _LocaleOption({required this.code, required this.label, required this.flag, required this.native});
}
