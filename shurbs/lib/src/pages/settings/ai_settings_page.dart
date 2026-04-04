import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

import 'settings_header_bg.dart';

class AiSettingsPage extends StatefulWidget {
  const AiSettingsPage({super.key});

  @override
  State<AiSettingsPage> createState() => _AiSettingsPageState();
}

class _AiSettingsPageState extends State<AiSettingsPage> {
  final _serverUrlController = TextEditingController(text: 'http://localhost:11434');
  String _selectedModel = 'llama3';
  bool _connected = true;

  static const _models = ['llama3', 'codellama', 'mistral', 'gemma'];

  @override
  void dispose() {
    _serverUrlController.dispose();
    super.dispose();
  }

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
                'AI & Ollama',
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
                        child: HeroIcon(HeroIcons.cpuChip, size: 30, color: Colors.white),
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
                // Connection status banner
                AnimatedContainer(
                  duration: const Duration(milliseconds: 300),
                  margin: const EdgeInsets.only(bottom: 12),
                  padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
                  decoration: BoxDecoration(
                    color: _connected
                        ? Colors.green.withValues(alpha: 0.1)
                        : colorScheme.error.withValues(alpha: 0.1),
                    borderRadius: BorderRadius.circular(10),
                    border: Border.all(
                      color: _connected
                          ? Colors.green.withValues(alpha: 0.3)
                          : colorScheme.error.withValues(alpha: 0.3),
                    ),
                  ),
                  child: Row(
                    children: [
                      Container(
                        width: 8,
                        height: 8,
                        decoration: BoxDecoration(
                          color: _connected ? Colors.green : colorScheme.error,
                          shape: BoxShape.circle,
                          boxShadow: [
                            BoxShadow(
                              color: (_connected ? Colors.green : colorScheme.error).withValues(alpha: 0.5),
                              blurRadius: 6,
                            ),
                          ],
                        ),
                      ),
                      const SizedBox(width: 10),
                      Text(
                        _connected ? 'Ollama is connected' : 'Ollama is unreachable',
                        style: theme.textTheme.bodySmall?.copyWith(
                          color: _connected ? Colors.green : colorScheme.error,
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                      const Spacer(),
                      GestureDetector(
                        onTap: () => setState(() => _connected = !_connected),
                        child: Text(
                          'Test',
                          style: theme.textTheme.labelSmall?.copyWith(
                            color: _connected ? Colors.green : colorScheme.error,
                            decoration: TextDecoration.underline,
                          ),
                        ),
                      ),
                    ],
                  ),
                ),

                // Server URL card
                Card(
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
                              child: Center(child: HeroIcon(HeroIcons.server, size: 18, color: colorScheme.primary)),
                            ),
                            const SizedBox(width: 12),
                            Text('Server URL', style: theme.textTheme.bodyMedium?.copyWith(fontWeight: FontWeight.w500)),
                          ],
                        ),
                        const SizedBox(height: 12),
                        TextField(
                          controller: _serverUrlController,
                          style: const TextStyle(fontFamily: 'monospace', fontSize: 13),
                          decoration: const InputDecoration(hintText: 'http://localhost:11434'),
                        ),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 12),

                // Model selection card
                Card(
                  child: Padding(
                    padding: const EdgeInsets.symmetric(vertical: 8),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Padding(
                          padding: const EdgeInsets.fromLTRB(16, 12, 16, 8),
                          child: Text(
                            'DEFAULT MODEL',
                            style: theme.textTheme.labelSmall?.copyWith(
                              color: colorScheme.primary,
                              fontWeight: FontWeight.w700,
                              letterSpacing: 1.2,
                            ),
                          ),
                        ),
                        ..._models.map((model) {
                          final selected = _selectedModel == model;
                          return Column(
                            children: [
                              if (model != _models.first)
                                const Divider(height: 1, indent: 16, endIndent: 16),
                              ListTile(
                                leading: Container(
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
                                      HeroIcons.cpuChip,
                                      size: 18,
                                      color: selected ? colorScheme.primary : colorScheme.onSurfaceVariant,
                                    ),
                                  ),
                                ),
                                title: Text(model),
                                trailing: selected
                                    ? Icon(Icons.check_circle_rounded, color: colorScheme.primary, size: 20)
                                    : null,
                                selected: selected,
                                shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                                onTap: () => setState(() => _selectedModel = model),
                              ),
                            ],
                          );
                        }),
                        const SizedBox(height: 8),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 16),
                SizedBox(
                  width: double.infinity,
                  height: 48,
                  child: FilledButton(onPressed: () {}, child: const Text('Save changes')),
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
