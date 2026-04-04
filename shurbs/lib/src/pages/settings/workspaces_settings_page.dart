import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

import '../../theme_notifier.dart';

class WorkspacesSettingsPage extends StatefulWidget {
  const WorkspacesSettingsPage({super.key});

  @override
  State<WorkspacesSettingsPage> createState() => _WorkspacesSettingsPageState();
}

class _WorkspacesSettingsPageState extends State<WorkspacesSettingsPage> {
  late List<_WorkspaceItem> _workspaces;
  String _defaultId = 'personal';

  @override
  void initState() {
    super.initState();
    _workspaces = workspaces
        .map((w) => _WorkspaceItem(id: w.id, name: w.name, icon: w.icon, isHidden: false))
        .toList();
  }

  void _setDefault(String id) => setState(() => _defaultId = id);

  void _toggleHidden(String id) {
    setState(() {
      final idx = _workspaces.indexWhere((w) => w.id == id);
      if (idx != -1) {
        final w = _workspaces[idx];
        _workspaces[idx] = _WorkspaceItem(id: w.id, name: w.name, icon: w.icon, isHidden: !w.isHidden);
      }
    });
  }

  void _delete(String id) {
    setState(() => _workspaces.removeWhere((w) => w.id == id));
  }

  void _showEditDialog(BuildContext context, _WorkspaceItem ws) {
    final nameCtrl = TextEditingController(text: ws.name);
    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        title: const Text('Edit Workspace'),
        content: TextField(
          controller: nameCtrl,
          decoration: const InputDecoration(labelText: 'Name'),
          autofocus: true,
        ),
        actions: [
          TextButton(onPressed: () => Navigator.pop(ctx), child: const Text('Cancel')),
          FilledButton(
            onPressed: () {
              final newName = nameCtrl.text.trim();
              if (newName.isNotEmpty) {
                setState(() {
                  final idx = _workspaces.indexWhere((w) => w.id == ws.id);
                  if (idx != -1) {
                    final w = _workspaces[idx];
                    _workspaces[idx] = _WorkspaceItem(id: w.id, name: newName, icon: w.icon, isHidden: w.isHidden);
                  }
                });
              }
              Navigator.pop(ctx);
            },
            child: const Text('Save'),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Scaffold(
      appBar: AppBar(title: const Text('Workspaces')),
      body: SafeArea(
        child: ListView(
          padding: const EdgeInsets.all(16),
          children: [
            ..._workspaces.map((ws) {
              final isDefault = ws.id == _defaultId;
              return Padding(
                padding: const EdgeInsets.only(bottom: 12),
                child: Card(
                  child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: Row(
                      children: [
                        Container(
                          width: 42,
                          height: 42,
                          decoration: BoxDecoration(
                            color: colorScheme.primaryContainer,
                            borderRadius: BorderRadius.circular(10),
                          ),
                          child: Center(
                            child: HeroIcon(ws.icon, color: colorScheme.onPrimaryContainer, size: 20),
                          ),
                        ),
                        const SizedBox(width: 14),
                        Expanded(
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Row(
                                children: [
                                  Text(ws.name, style: theme.textTheme.titleSmall?.copyWith(fontWeight: FontWeight.w600)),
                                  if (isDefault) ...[
                                    const SizedBox(width: 8),
                                    Container(
                                      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
                                      decoration: BoxDecoration(
                                        color: colorScheme.primary.withValues(alpha: 0.15),
                                        borderRadius: BorderRadius.circular(20),
                                      ),
                                      child: Text(
                                        'Default',
                                        style: theme.textTheme.labelSmall?.copyWith(color: colorScheme.primary),
                                      ),
                                    ),
                                  ],
                                ],
                              ),
                              if (ws.isHidden)
                                Text('Hidden', style: theme.textTheme.bodySmall?.copyWith(color: colorScheme.onSurfaceVariant)),
                            ],
                          ),
                        ),
                        PopupMenuButton<String>(
                          icon: const HeroIcon(HeroIcons.ellipsisVertical, size: 18),
                          onSelected: (action) {
                            switch (action) {
                              case 'edit':
                                _showEditDialog(context, ws);
                              case 'default':
                                _setDefault(ws.id);
                              case 'hide':
                                _toggleHidden(ws.id);
                              case 'delete':
                                _delete(ws.id);
                            }
                          },
                          itemBuilder: (_) => [
                            const PopupMenuItem(value: 'edit', child: Text('Edit')),
                            if (!isDefault)
                              const PopupMenuItem(value: 'default', child: Text('Set as default')),
                            PopupMenuItem(value: 'hide', child: Text(ws.isHidden ? 'Show' : 'Hide')),
                            if (!isDefault)
                              PopupMenuItem(
                                value: 'delete',
                                child: Text('Delete', style: TextStyle(color: colorScheme.error)),
                              ),
                          ],
                        ),
                      ],
                    ),
                  ),
                ),
              );
            }),
          ],
        ),
      ),
    );
  }
}

class _WorkspaceItem {
  final String id;
  final String name;
  final HeroIcons icon;
  final bool isHidden;
  const _WorkspaceItem({required this.id, required this.name, required this.icon, required this.isHidden});
}
