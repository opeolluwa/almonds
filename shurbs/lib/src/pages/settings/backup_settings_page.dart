import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

enum _BackupProvider { local, cloud, selfHosted }

class BackupSettingsPage extends StatefulWidget {
  const BackupSettingsPage({super.key});

  @override
  State<BackupSettingsPage> createState() => _BackupSettingsPageState();
}

class _BackupSettingsPageState extends State<BackupSettingsPage> {
  _BackupProvider _selected = _BackupProvider.local;
  final _apiUrlController = TextEditingController();
  final _apiKeyController = TextEditingController();

  @override
  void dispose() {
    _apiUrlController.dispose();
    _apiKeyController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    final options = [
      _ProviderOption(
        key: _BackupProvider.local,
        label: 'Local only',
        desc: 'Data stays on this device, no sync.',
        icon: HeroIcons.computerDesktop,
      ),
      _ProviderOption(
        key: _BackupProvider.cloud,
        label: 'Almond Cloud',
        desc: 'Sync across devices via Almond Cloud.',
        icon: HeroIcons.cloud,
      ),
      _ProviderOption(
        key: _BackupProvider.selfHosted,
        label: 'Self Hosted',
        desc: 'Use your own server to sync data.',
        icon: HeroIcons.server,
      ),
    ];

    return Scaffold(
      appBar: AppBar(title: const Text('Backup & Sync')),
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
                    Text('Backup & Sync', style: theme.textTheme.titleSmall?.copyWith(fontWeight: FontWeight.w600)),
                    const SizedBox(height: 4),
                    Text('Choose where your data is stored and synced.', style: theme.textTheme.bodySmall),
                    const SizedBox(height: 16),
                    ...options.map((opt) {
                      final selected = _selected == opt.key;
                      return Padding(
                        padding: const EdgeInsets.only(bottom: 8),
                        child: GestureDetector(
                          onTap: () => setState(() => _selected = opt.key),
                          child: AnimatedContainer(
                            duration: const Duration(milliseconds: 150),
                            padding: const EdgeInsets.all(14),
                            decoration: BoxDecoration(
                              borderRadius: BorderRadius.circular(10),
                              border: Border.all(
                                color: selected ? colorScheme.primary : colorScheme.outline.withValues(alpha: 0.5),
                                width: selected ? 1.5 : 1,
                              ),
                              color: selected ? colorScheme.primary.withValues(alpha: 0.06) : Colors.transparent,
                            ),
                            child: Row(
                              children: [
                                Container(
                                  width: 36,
                                  height: 36,
                                  decoration: BoxDecoration(
                                    color: selected
                                        ? colorScheme.primary.withValues(alpha: 0.12)
                                        : colorScheme.surfaceContainerHighest,
                                    borderRadius: BorderRadius.circular(8),
                                  ),
                                  child: Center(
                                    child: HeroIcon(
                                      opt.icon,
                                      size: 18,
                                      color: selected ? colorScheme.primary : colorScheme.onSurfaceVariant,
                                    ),
                                  ),
                                ),
                                const SizedBox(width: 12),
                                Expanded(
                                  child: Column(
                                    crossAxisAlignment: CrossAxisAlignment.start,
                                    children: [
                                      Text(
                                        opt.label,
                                        style: theme.textTheme.bodyMedium?.copyWith(
                                          fontWeight: FontWeight.w500,
                                          color: selected ? colorScheme.primary : null,
                                        ),
                                      ),
                                      Text(opt.desc, style: theme.textTheme.bodySmall),
                                    ],
                                  ),
                                ),
                                if (selected)
                                  Icon(Icons.check_circle_rounded, color: colorScheme.primary, size: 20),
                              ],
                            ),
                          ),
                        ),
                      );
                    }),
                    if (_selected == _BackupProvider.cloud) ...[
                      const SizedBox(height: 8),
                      Container(
                        padding: const EdgeInsets.all(14),
                        decoration: BoxDecoration(
                          color: colorScheme.primary.withValues(alpha: 0.06),
                          borderRadius: BorderRadius.circular(10),
                          border: Border.all(color: colorScheme.primary.withValues(alpha: 0.2)),
                        ),
                        child: Row(
                          children: [
                            Expanded(
                              child: Column(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                children: [
                                  Text('Almond Cloud', style: theme.textTheme.bodyMedium?.copyWith(
                                    fontWeight: FontWeight.w600, color: colorScheme.primary,
                                  )),
                                  Text('Secure, encrypted sync. Plans start free.', style: theme.textTheme.bodySmall),
                                ],
                              ),
                            ),
                            FilledButton.icon(
                              onPressed: () {},
                              icon: const HeroIcon(HeroIcons.arrowTopRightOnSquare, size: 14),
                              label: const Text('View plans'),
                            ),
                          ],
                        ),
                      ),
                    ],
                    if (_selected == _BackupProvider.selfHosted) ...[
                      const SizedBox(height: 16),
                      TextField(
                        controller: _apiUrlController,
                        decoration: const InputDecoration(
                          labelText: 'API Endpoint',
                          hintText: 'https://sync.example.com/api',
                        ),
                      ),
                      const SizedBox(height: 12),
                      TextField(
                        controller: _apiKeyController,
                        obscureText: true,
                        decoration: const InputDecoration(
                          labelText: 'API Key',
                          hintText: 'sk-••••••••••••',
                        ),
                      ),
                      const SizedBox(height: 16),
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          TextButton.icon(
                            onPressed: () {},
                            icon: const HeroIcon(HeroIcons.signal, size: 16),
                            label: const Text('Test connection'),
                          ),
                          FilledButton(onPressed: () {}, child: const Text('Save')),
                        ],
                      ),
                    ],
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

class _ProviderOption {
  final _BackupProvider key;
  final String label;
  final String desc;
  final HeroIcons icon;
  const _ProviderOption({required this.key, required this.label, required this.desc, required this.icon});
}
