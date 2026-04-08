import 'dart:convert';

import 'package:flutter/foundation.dart';

import '../rust/api/workspaces.dart';

class AppWorkspace {
  final String id;
  final String name;
  final bool isDefault;

  const AppWorkspace({required this.id, required this.name, required this.isDefault});

  factory AppWorkspace.fromJson(Map<String, dynamic> j) => AppWorkspace(
        id: j['identifier'] as String,
        name: j['name'] as String,
        isDefault: j['isDefault'] as bool? ?? false,
      );
}

class WorkspaceController extends ChangeNotifier {
  List<AppWorkspace> _workspaces = [];
  String? _activeWorkspaceId;
  bool loading = true;

  List<AppWorkspace> get workspaces => List.unmodifiable(_workspaces);
  String? get activeWorkspaceId => _activeWorkspaceId;

  Future<void> load() async {
    try {
      final raw = await listWorkspaces();
      final list = (jsonDecode(raw) as List).cast<Map<String, dynamic>>();
      _workspaces = list.map(AppWorkspace.fromJson).toList();

      if (_workspaces.isEmpty) {
        // Create a default workspace on first run
        final created = await createWorkspace(name: 'Personal', description: 'My workspace');
        final ws = AppWorkspace.fromJson(jsonDecode(created) as Map<String, dynamic>);
        _workspaces = [ws];
        _activeWorkspaceId = ws.id;
      } else {
        // Prefer the workspace marked as default, otherwise take the first
        final def = _workspaces.where((w) => w.isDefault).firstOrNull;
        _activeWorkspaceId = (def ?? _workspaces.first).id;
      }
    } catch (e) {
      debugPrint('WorkspaceController.load error: $e');
    }
    loading = false;
    notifyListeners();
  }

  void setActive(String id) {
    _activeWorkspaceId = id;
    notifyListeners();
  }
}
