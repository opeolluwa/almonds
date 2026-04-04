import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

import 'settings_header_bg.dart';

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
      body: CustomScrollView(
        slivers: [
          SliverAppBar(
            expandedHeight: 180,
            pinned: true,
            flexibleSpace: FlexibleSpaceBar(
              title: const Text(
                'Backup & Sync',
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
                        child: HeroIcon(HeroIcons.cloudArrowUp, size: 30, color: Colors.white),
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
                    padding: const EdgeInsets.all(16),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          'Choose where your data is stored and synced.',
                          style: theme.textTheme.bodySmall,
                        ),
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
                                    color: selected
                                        ? colorScheme.primary
                                        : colorScheme.outline.withValues(alpha: 0.5),
                                    width: selected ? 1.5 : 1,
                                  ),
                                  color: selected
                                      ? colorScheme.primary.withValues(alpha: 0.06)
                                      : Colors.transparent,
                                ),
                                child: Row(
                                  children: [
                                    Container(
                                      width: 40,
                                      height: 40,
                                      decoration: BoxDecoration(
                                        color: selected
                                            ? colorScheme.primary.withValues(alpha: 0.12)
                                            : colorScheme.surfaceContainerHighest,
                                        borderRadius: BorderRadius.circular(10),
                                      ),
                                      child: Center(
                                        child: HeroIcon(
                                          opt.icon,
                                          size: 20,
                                          color: selected
                                              ? colorScheme.primary
                                              : colorScheme.onSurfaceVariant,
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
                                              fontWeight: FontWeight.w600,
                                              color: selected ? colorScheme.primary : null,
                                            ),
                                          ),
                                          const SizedBox(height: 2),
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
                          const SizedBox(height: 4),
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
                                      Text(
                                        'Almond Cloud',
                                        style: theme.textTheme.bodyMedium?.copyWith(
                                          fontWeight: FontWeight.w600,
                                          color: colorScheme.primary,
                                        ),
                                      ),
                                      const SizedBox(height: 2),
                                      Text(
                                        'Secure, encrypted sync. Plans start free.',
                                        style: theme.textTheme.bodySmall,
                                      ),
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
                const SizedBox(height: 32),
              ]),
            ),
          ),
        ],
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
