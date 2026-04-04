import 'package:flutter/material.dart';

class LocaleSettingsPage extends StatefulWidget {
  const LocaleSettingsPage({super.key});

  @override
  State<LocaleSettingsPage> createState() => _LocaleSettingsPageState();
}

class _LocaleSettingsPageState extends State<LocaleSettingsPage> {
  static const _locales = [
    _LocaleOption(code: 'en', label: 'English', flag: '🇺🇸'),
    _LocaleOption(code: 'fr', label: 'Français', flag: '🇫🇷'),
    _LocaleOption(code: 'de', label: 'Deutsch', flag: '🇩🇪'),
    _LocaleOption(code: 'es', label: 'Español', flag: '🇪🇸'),
    _LocaleOption(code: 'pt', label: 'Português', flag: '🇧🇷'),
    _LocaleOption(code: 'ar', label: 'العربية', flag: '🇸🇦'),
    _LocaleOption(code: 'zh', label: '中文', flag: '🇨🇳'),
    _LocaleOption(code: 'ja', label: '日本語', flag: '🇯🇵'),
  ];

  String _selected = 'en';

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Scaffold(
      appBar: AppBar(title: const Text('Locale')),
      body: SafeArea(
        child: ListView(
          padding: const EdgeInsets.all(16),
          children: [
            Card(
              child: Padding(
                padding: const EdgeInsets.symmetric(vertical: 8),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Padding(
                      padding: const EdgeInsets.fromLTRB(16, 12, 16, 8),
                      child: Text('Language', style: theme.textTheme.labelLarge),
                    ),
                    ..._locales.map((locale) {
                      final selected = _selected == locale.code;
                      return ListTile(
                        leading: Text(locale.flag, style: const TextStyle(fontSize: 22)),
                        title: Text(locale.label),
                        trailing: selected
                            ? Icon(Icons.check_rounded, color: theme.colorScheme.primary)
                            : null,
                        selected: selected,
                        onTap: () => setState(() => _selected = locale.code),
                      );
                    }),
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

class _LocaleOption {
  final String code;
  final String label;
  final String flag;
  const _LocaleOption({required this.code, required this.label, required this.flag});
}
