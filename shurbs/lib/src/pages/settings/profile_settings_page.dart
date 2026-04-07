import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

class ProfileSettingsPage extends StatefulWidget {
  const ProfileSettingsPage({super.key});

  @override
  State<ProfileSettingsPage> createState() => _ProfileSettingsPageState();
}

class _ProfileSettingsPageState extends State<ProfileSettingsPage> {
  final _firstNameController = TextEditingController(text: 'Adeoye');
  final _lastNameController = TextEditingController(text: 'Adefemi');
  final _emailController = TextEditingController(text: 'adeoye@wildalmonds.app');
  bool _saving = false;

  @override
  void dispose() {
    _firstNameController.dispose();
    _lastNameController.dispose();
    _emailController.dispose();
    super.dispose();
  }

  Future<void> _save() async {
    setState(() => _saving = true);
    await Future.delayed(const Duration(milliseconds: 800));
    if (mounted) setState(() => _saving = false);
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Scaffold(
      appBar: AppBar(
        title: const Text('Profile', style: TextStyle(color: Colors.black)),
        foregroundColor: Colors.black,
      ),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
                _SectionLabel('Personal info'),
                const SizedBox(height: 8),
                Card(
                  child: Padding(
                    padding: const EdgeInsets.all(20),
                    child: Column(
                      children: [
                        Row(
                          children: [
                            Expanded(
                              child: _FieldWithIcon(
                                icon: HeroIcons.user,
                                controller: _firstNameController,
                                label: 'First Name',
                              ),
                            ),
                            const SizedBox(width: 12),
                            Expanded(
                              child: _FieldWithIcon(
                                icon: HeroIcons.user,
                                controller: _lastNameController,
                                label: 'Last Name',
                              ),
                            ),
                          ],
                        ),
                        const SizedBox(height: 16),
                        _FieldWithIcon(
                          icon: HeroIcons.envelope,
                          controller: _emailController,
                          label: 'Email',
                          keyboardType: TextInputType.emailAddress,
                        ),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 24),
                SizedBox(
                  width: double.infinity,
                  height: 52,
                  child: FilledButton(
                    onPressed: _saving ? null : _save,
                    child: _saving
                        ? SizedBox(
                            width: 20,
                            height: 20,
                            child: CircularProgressIndicator(
                              strokeWidth: 2,
                              color: colorScheme.onPrimary,
                            ),
                          )
                        : const Text('Save changes', style: TextStyle(fontSize: 16, fontWeight: FontWeight.w600)),
                  ),
                ),
                const SizedBox(height: 16),
                Card(
                  child: ListTile(
                    leading: Container(
                      width: 36,
                      height: 36,
                      decoration: BoxDecoration(
                        color: colorScheme.error.withValues(alpha: 0.12),
                        borderRadius: BorderRadius.circular(8),
                      ),
                      child: Center(child: HeroIcon(HeroIcons.trash, size: 18, color: colorScheme.error)),
                    ),
                    title: Text('Delete account', style: TextStyle(color: colorScheme.error)),
                    subtitle: const Text('Permanently remove your data'),
                    trailing: HeroIcon(HeroIcons.chevronRight, size: 16, color: colorScheme.onSurfaceVariant),
                    onTap: () {},
                  ),
                ),
                const SizedBox(height: 32),
        ],
      ),
    );
  }
}

class _SectionLabel extends StatelessWidget {
  final String text;
  const _SectionLabel(this.text);

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Padding(
      padding: const EdgeInsets.only(left: 4, bottom: 2),
      child: Text(
        text.toUpperCase(),
        style: theme.textTheme.labelSmall?.copyWith(
          color: theme.colorScheme.primary,
          fontWeight: FontWeight.w700,
          letterSpacing: 1.2,
        ),
      ),
    );
  }
}

class _FieldWithIcon extends StatelessWidget {
  final HeroIcons icon;
  final TextEditingController controller;
  final String label;
  final TextInputType? keyboardType;

  const _FieldWithIcon({
    required this.icon,
    required this.controller,
    required this.label,
    this.keyboardType,
  });

  @override
  Widget build(BuildContext context) {
    return TextField(
      controller: controller,
      keyboardType: keyboardType,
      decoration: InputDecoration(
        labelText: label,
        prefixIcon: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 12),
          child: HeroIcon(icon, size: 18),
        ),
        prefixIconConstraints: const BoxConstraints(minWidth: 0, minHeight: 0),
      ),
    );
  }
}
