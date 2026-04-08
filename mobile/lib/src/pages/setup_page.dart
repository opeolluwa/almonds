import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

import '../profile_notifier.dart';
import '../rust/api/workspaces.dart';
import '../shell.dart';
import '../theme_notifier.dart';

class SetupPage extends StatefulWidget {
  const SetupPage({super.key});

  @override
  State<SetupPage> createState() => _SetupPageState();
}

class _SetupPageState extends State<SetupPage> {
  final _pageController = PageController();
  int _step = 0;

  // Profile
  final _firstNameCtrl = TextEditingController();
  final _lastNameCtrl = TextEditingController();
  final _emailCtrl = TextEditingController();

  // Workspace
  String _selectedWorkspace = 'personal';
  bool _showNameError = false;

  @override
  void dispose() {
    _pageController.dispose();
    _firstNameCtrl.dispose();
    _lastNameCtrl.dispose();
    _emailCtrl.dispose();
    super.dispose();
  }

  void _next() {
    if (_step == 0) {
      if (_firstNameCtrl.text.trim().isEmpty) {
        setState(() => _showNameError = true);
        return;
      }
      setState(() => _showNameError = false);
      _pageController.nextPage(
        duration: const Duration(milliseconds: 360),
        curve: Curves.easeInOut,
      );
      setState(() => _step = 1);
    } else {
      _finish();
    }
  }

  Future<void> _finish() async {
    final firstName = _firstNameCtrl.text.trim();
    final lastName = _lastNameCtrl.text.trim();
    final email = _emailCtrl.text.trim();

    // Create the workspace first so we have an ID to attach the profile to.
    final wsName = _selectedWorkspace == 'work'
        ? 'Work'
        : _selectedWorkspace == 'school'
            ? 'School'
            : 'Personal';
    String? wsId;
    try {
      final raw = await createWorkspace(name: wsName, description: '');
      wsId = (jsonDecode(raw) as Map<String, dynamic>)['identifier'] as String?;
    } catch (e) {
      debugPrint('SetupPage workspace create error: $e');
    }

    if (firstName.isNotEmpty || lastName.isNotEmpty || email.isNotEmpty) {
      await ProfileNotifier.instance.save(
        newFirstName: firstName,
        newLastName: lastName,
        newEmail: email,
        workspaceId: wsId,
      );
    }

    activeWorkspaceNotifier.value = _selectedWorkspace;

    if (!mounted) return;
    Navigator.of(context).pushReplacement(
      MaterialPageRoute(builder: (_) => const AppShell()),
    );
  }

  @override
  Widget build(BuildContext context) {
    final accent = Theme.of(context).colorScheme.primary;

    return Scaffold(
      backgroundColor: const Color(0xFFF8F6F2),
      body: SafeArea(
        child: Column(
          children: [
            // ── Step indicator ──────────────────────────────────────────────
            Padding(
              padding: const EdgeInsets.fromLTRB(24, 20, 24, 0),
              child: Row(
                children: [
                  _StepDot(number: 1, active: _step == 0, done: _step > 0, color: accent),
                  Expanded(
                    child: Container(
                      height: 2,
                      margin: const EdgeInsets.symmetric(horizontal: 6),
                      decoration: BoxDecoration(
                        color: _step > 0 ? accent : accent.withValues(alpha: 0.2),
                        borderRadius: BorderRadius.circular(1),
                      ),
                    ),
                  ),
                  _StepDot(number: 2, active: _step == 1, done: false, color: accent),
                ],
              ),
            ),

            // ── Steps ───────────────────────────────────────────────────────
            Expanded(
              child: PageView(
                controller: _pageController,
                physics: const NeverScrollableScrollPhysics(),
                children: [
                  _ProfileStep(
                    firstNameCtrl: _firstNameCtrl,
                    lastNameCtrl: _lastNameCtrl,
                    emailCtrl: _emailCtrl,
                    showError: _showNameError,
                    onFirstNameChanged: (_) {
                      if (_showNameError) setState(() => _showNameError = false);
                    },
                  ),
                  _WorkspaceStep(
                    selected: _selectedWorkspace,
                    onSelect: (id) => setState(() => _selectedWorkspace = id),
                    accent: accent,
                  ),
                ],
              ),
            ),

            // ── Bottom bar ──────────────────────────────────────────────────
            Padding(
              padding: const EdgeInsets.fromLTRB(24, 0, 24, 32),
              child: Column(
                children: [
                  SizedBox(
                    width: double.infinity,
                    height: 54,
                    child: FilledButton(
                      onPressed: _next,
                      style: FilledButton.styleFrom(
                        backgroundColor: accent,
                        shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(14),
                        ),
                      ),
                      child: Text(
                        _step == 0 ? 'Continue' : 'Get started',
                        style: const TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

// ── Step dot ──────────────────────────────────────────────────────────────────

class _StepDot extends StatelessWidget {
  final int number;
  final bool active;
  final bool done;
  final Color color;
  const _StepDot({required this.number, required this.active, required this.done, required this.color});

  @override
  Widget build(BuildContext context) {
    final filled = active || done;
    return AnimatedContainer(
      duration: const Duration(milliseconds: 260),
      width: 28,
      height: 28,
      decoration: BoxDecoration(
        color: filled ? color : color.withValues(alpha: 0.12),
        shape: BoxShape.circle,
      ),
      child: Center(
        child: done
            ? const Icon(Icons.check_rounded, color: Colors.white, size: 14)
            : Text(
                '$number',
                style: TextStyle(
                  color: filled ? Colors.white : color.withValues(alpha: 0.4),
                  fontSize: 12,
                  fontWeight: FontWeight.w700,
                ),
              ),
      ),
    );
  }
}

// ── Profile step ──────────────────────────────────────────────────────────────

class _ProfileStep extends StatelessWidget {
  final TextEditingController firstNameCtrl;
  final TextEditingController lastNameCtrl;
  final TextEditingController emailCtrl;
  final bool showError;
  final ValueChanged<String> onFirstNameChanged;

  const _ProfileStep({
    required this.firstNameCtrl,
    required this.lastNameCtrl,
    required this.emailCtrl,
    required this.showError,
    required this.onFirstNameChanged,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final accent = Theme.of(context).colorScheme.primary;

    return ListView(
      padding: const EdgeInsets.fromLTRB(24, 32, 24, 16),
      children: [
        Text(
          'Nice to meet you',
          style: theme.textTheme.headlineMedium?.copyWith(
            fontWeight: FontWeight.w700,
            color: const Color(0xFF1a1a2e),
            letterSpacing: -0.5,
          ),
        ),
        const SizedBox(height: 8),
        Text(
          'Tell us a little about yourself so we can personalise your experience.',
          style: theme.textTheme.bodyMedium?.copyWith(
            color: const Color(0xFF1a1a2e).withValues(alpha: 0.5),
            height: 1.5,
          ),
        ),
        const SizedBox(height: 32),

        // Name row
        Row(
          children: [
            Expanded(
              child: _Field(
                controller: firstNameCtrl,
                label: 'First name',
                icon: HeroIcons.user,
                hasError: showError,
                onChanged: onFirstNameChanged,
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: _Field(
                controller: lastNameCtrl,
                label: 'Last name',
                icon: HeroIcons.user,
              ),
            ),
          ],
        ),

        // Error message
        AnimatedSize(
          duration: const Duration(milliseconds: 200),
          child: showError
              ? Padding(
                  padding: const EdgeInsets.only(top: 8, left: 2),
                  child: Row(
                    children: [
                      Icon(Icons.info_outline_rounded, size: 14, color: accent),
                      const SizedBox(width: 5),
                      Text(
                        'First name is required to continue',
                        style: theme.textTheme.labelSmall?.copyWith(color: accent),
                      ),
                    ],
                  ),
                )
              : const SizedBox.shrink(),
        ),

        const SizedBox(height: 16),

        _Field(
          controller: emailCtrl,
          label: 'Email address',
          icon: HeroIcons.envelope,
          keyboardType: TextInputType.emailAddress,
        ),
      ],
    );
  }
}

class _Field extends StatelessWidget {
  final TextEditingController controller;
  final String label;
  final HeroIcons icon;
  final TextInputType? keyboardType;
  final bool hasError;
  final ValueChanged<String>? onChanged;

  const _Field({
    required this.controller,
    required this.label,
    required this.icon,
    this.keyboardType,
    this.hasError = false,
    this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    final primary = Theme.of(context).colorScheme.primary;
    final errorColor = primary;

    return TextField(
      controller: controller,
      keyboardType: keyboardType,
      textCapitalization: TextCapitalization.words,
      onChanged: onChanged,
      decoration: InputDecoration(
        labelText: label,
        filled: true,
        fillColor: hasError ? errorColor.withValues(alpha: 0.04) : Colors.white,
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide.none,
        ),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(
            color: hasError ? errorColor.withValues(alpha: 0.5) : Colors.black.withValues(alpha: 0.07),
          ),
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: BorderSide(color: hasError ? errorColor : primary, width: 1.5),
        ),
        prefixIcon: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 12),
          child: HeroIcon(icon, size: 18, color: hasError ? errorColor : Theme.of(context).colorScheme.onSurfaceVariant),
        ),
        prefixIconConstraints: const BoxConstraints(minWidth: 0, minHeight: 0),
      ),
    );
  }
}

// ── Workspace step ────────────────────────────────────────────────────────────

const _kWorkspaceOptions = [
  (id: 'personal', label: 'Personal', subtitle: 'Todos, notes & reminders for everyday life', icon: HeroIcons.home),
  (id: 'work',     label: 'Work',     subtitle: 'Projects, meetings & professional tasks',    icon: HeroIcons.briefcase),
  (id: 'school',   label: 'School',   subtitle: 'Assignments, study notes & deadlines',       icon: HeroIcons.academicCap),
];

class _WorkspaceStep extends StatelessWidget {
  final String selected;
  final ValueChanged<String> onSelect;
  final Color accent;

  const _WorkspaceStep({
    required this.selected,
    required this.onSelect,
    required this.accent,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return ListView(
      padding: const EdgeInsets.fromLTRB(24, 32, 24, 16),
      children: [
        Text(
          'Choose a workspace',
          style: theme.textTheme.headlineMedium?.copyWith(
            fontWeight: FontWeight.w700,
            color: const Color(0xFF1a1a2e),
            letterSpacing: -0.5,
          ),
        ),
        const SizedBox(height: 8),
        Text(
          'You can add more workspaces or switch between them later in settings.',
          style: theme.textTheme.bodyMedium?.copyWith(
            color: const Color(0xFF1a1a2e).withValues(alpha: 0.5),
            height: 1.5,
          ),
        ),
        const SizedBox(height: 28),

        for (final ws in _kWorkspaceOptions) ...[
          _WorkspaceTile(
            id: ws.id,
            label: ws.label,
            subtitle: ws.subtitle,
            icon: ws.icon,
            isSelected: selected == ws.id,
            accent: accent,
            onTap: () => onSelect(ws.id),
          ),
          const SizedBox(height: 12),
        ],
      ],
    );
  }
}

class _WorkspaceTile extends StatelessWidget {
  final String id;
  final String label;
  final String subtitle;
  final HeroIcons icon;
  final bool isSelected;
  final Color accent;
  final VoidCallback onTap;

  const _WorkspaceTile({
    required this.id,
    required this.label,
    required this.subtitle,
    required this.icon,
    required this.isSelected,
    required this.accent,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return GestureDetector(
      onTap: onTap,
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 200),
        padding: const EdgeInsets.all(16),
        decoration: BoxDecoration(
          color: Colors.white,
          borderRadius: BorderRadius.circular(14),
          border: Border.all(
            color: isSelected ? accent : Colors.black.withValues(alpha: 0.07),
            width: isSelected ? 2 : 1,
          ),
          boxShadow: isSelected
              ? [BoxShadow(color: accent.withValues(alpha: 0.15), blurRadius: 12, offset: const Offset(0, 4))]
              : null,
        ),
        child: Row(
          children: [
            AnimatedContainer(
              duration: const Duration(milliseconds: 200),
              width: 44,
              height: 44,
              decoration: BoxDecoration(
                color: isSelected ? accent : accent.withValues(alpha: 0.08),
                borderRadius: BorderRadius.circular(11),
              ),
              child: Center(
                child: HeroIcon(
                  icon,
                  size: 20,
                  color: isSelected ? Colors.white : accent,
                ),
              ),
            ),
            const SizedBox(width: 14),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    label,
                    style: theme.textTheme.titleSmall?.copyWith(
                      fontWeight: FontWeight.w600,
                      color: const Color(0xFF1a1a2e),
                    ),
                  ),
                  const SizedBox(height: 2),
                  Text(
                    subtitle,
                    style: theme.textTheme.bodySmall?.copyWith(
                      color: const Color(0xFF1a1a2e).withValues(alpha: 0.45),
                    ),
                  ),
                ],
              ),
            ),
            const SizedBox(width: 8),
            AnimatedContainer(
              duration: const Duration(milliseconds: 200),
              width: 22,
              height: 22,
              decoration: BoxDecoration(
                color: isSelected ? accent : Colors.transparent,
                shape: BoxShape.circle,
                border: Border.all(
                  color: isSelected ? accent : Colors.black.withValues(alpha: 0.2),
                  width: 1.5,
                ),
              ),
              child: isSelected
                  ? const Icon(Icons.check_rounded, color: Colors.white, size: 13)
                  : null,
            ),
          ],
        ),
      ),
    );
  }
}
