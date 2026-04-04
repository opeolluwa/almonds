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
  AccentSwatch(
    label: 'Violet',
    primary: Color(0xFF7c3aed),
    primaryContainer: Color(0xFF4c1d95),
    onPrimaryContainer: Color(0xFFede9fe),
    secondary: Color(0xFFa78bfa),
  ),
  AccentSwatch(
    label: 'Sky',
    primary: Color(0xFF0284c7),
    primaryContainer: Color(0xFF0c4a6e),
    onPrimaryContainer: Color(0xFFe0f2fe),
    secondary: Color(0xFF38bdf8),
  ),
  AccentSwatch(
    label: 'Emerald',
    primary: Color(0xFF059669),
    primaryContainer: Color(0xFF065f46),
    onPrimaryContainer: Color(0xFFd1fae5),
    secondary: Color(0xFF34d399),
  ),
  AccentSwatch(
    label: 'Amber',
    primary: Color(0xFFd97706),
    primaryContainer: Color(0xFF78350f),
    onPrimaryContainer: Color(0xFFfef3c7),
    secondary: Color(0xFFfbbf24),
  ),
  AccentSwatch(
    label: 'Coral',
    primary: Color(0xFFea580c),
    primaryContainer: Color(0xFF7c2d12),
    onPrimaryContainer: Color(0xFFffedd5),
    secondary: Color(0xFFfb923c),
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
