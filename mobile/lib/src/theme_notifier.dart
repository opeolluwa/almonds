import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

final themeModeNotifier = ValueNotifier<ThemeMode>(ThemeMode.system);

class AccentSwatch {
  final String label;
  final Color primary;
  final Color primaryContainer;
  final Color onPrimaryContainer;
  final Color secondary;

  const AccentSwatch({
    required this.label,
    required this.primary,
    required this.primaryContainer,
    required this.onPrimaryContainer,
    required this.secondary,
  });
}

const accentSwatches = <AccentSwatch>[
  AccentSwatch(
    label: 'Rose',
    primary: Color(0xFFd02752),
    primaryContainer: Color(0xFFa11d3f),
    onPrimaryContainer: Color(0xFFf9d0d9),
    secondary: Color(0xFFed7694),
  ),
];

final accentColorNotifier = ValueNotifier<AccentSwatch>(accentSwatches[0]);

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
