import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

final themeModeNotifier = ValueNotifier<ThemeMode>(ThemeMode.system);

class Workspace {
  final String id;
  final String name;
  final HeroIcons icon;

  const Workspace({required this.id, required this.name, required this.icon});
}

final workspaces = <Workspace>[
  Workspace(id: 'personal', name: 'Personal', icon: HeroIcons.user),
  Workspace(id: 'work', name: 'Work', icon: HeroIcons.briefcase),
  Workspace(id: 'school', name: 'School', icon: HeroIcons.academicCap),
];

final activeWorkspaceNotifier = ValueNotifier<String>('personal');
