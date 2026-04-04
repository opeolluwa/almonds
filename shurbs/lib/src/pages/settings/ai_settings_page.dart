import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

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
      appBar: AppBar(title: const Text('AI & Ollama')),
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
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Text('AI & Ollama', style: theme.textTheme.titleSmall?.copyWith(fontWeight: FontWeight.w600)),
                        Row(
                          children: [
                            Container(
                              width: 8,
                              height: 8,
                              decoration: BoxDecoration(
                                color: _connected ? Colors.green : colorScheme.error,
                                shape: BoxShape.circle,
                              ),
                            ),
                            const SizedBox(width: 6),
                            Text(
                              _connected ? 'Connected' : 'Disconnected',
                              style: theme.textTheme.bodySmall,
                            ),
                          ],
                        ),
                      ],
                    ),
                    const SizedBox(height: 20),
                    Text('Server URL', style: theme.textTheme.labelMedium),
                    const SizedBox(height: 8),
                    TextField(
                      controller: _serverUrlController,
                      style: const TextStyle(fontFamily: 'monospace'),
                      decoration: const InputDecoration(hintText: 'http://localhost:11434'),
                    ),
                    const SizedBox(height: 20),
                    Text('Default Model', style: theme.textTheme.labelMedium),
                    const SizedBox(height: 8),
                    ..._models.map((model) {
                      final selected = _selectedModel == model;
                      return ListTile(
                        leading: const HeroIcon(HeroIcons.cpuChip, size: 18),
                        title: Text(model),
                        trailing: selected ? Icon(Icons.check_rounded, color: colorScheme.primary, size: 18) : null,
                        selected: selected,
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                        onTap: () => setState(() => _selectedModel = model),
                        contentPadding: const EdgeInsets.symmetric(horizontal: 12),
                        dense: true,
                      );
                    }),
                    const SizedBox(height: 16),
                    Align(
                      alignment: Alignment.centerRight,
                      child: FilledButton(onPressed: () {}, child: const Text('Save')),
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
