import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

import 'settings_header_bg.dart';

class AlarmSettingsPage extends StatefulWidget {
  const AlarmSettingsPage({super.key});

  @override
  State<AlarmSettingsPage> createState() => _AlarmSettingsPageState();
}

class _AlarmSettingsPageState extends State<AlarmSettingsPage> {
  String? _defaultSound;
  int _leadTimeMinutes = 0;
  int _snoozeDurationMinutes = 5;
  bool _repeatAlarm = false;

  static const _sounds = ['Chime', 'Bell', 'Digital', 'Nature'];

  static const _leadOptions = [
    _TimeOption(value: 0, label: 'At time'),
    _TimeOption(value: 5, label: '5 min'),
    _TimeOption(value: 10, label: '10 min'),
    _TimeOption(value: 15, label: '15 min'),
    _TimeOption(value: 30, label: '30 min'),
  ];

  static const _snoozeOptions = [
    _TimeOption(value: 5, label: '5 min'),
    _TimeOption(value: 10, label: '10 min'),
    _TimeOption(value: 15, label: '15 min'),
  ];

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
                'Alarm',
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
                        child: HeroIcon(HeroIcons.clock, size: 30, color: Colors.white),
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
                // Default sound
                Card(
                  child: Padding(
                    padding: const EdgeInsets.symmetric(vertical: 8),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Padding(
                          padding: const EdgeInsets.fromLTRB(16, 12, 16, 8),
                          child: Row(
                            children: [
                              Container(
                                width: 36,
                                height: 36,
                                decoration: BoxDecoration(
                                  color: colorScheme.primary.withValues(alpha: 0.12),
                                  borderRadius: BorderRadius.circular(8),
                                ),
                                child: Center(child: HeroIcon(HeroIcons.musicalNote, size: 18, color: colorScheme.primary)),
                              ),
                              const SizedBox(width: 12),
                              Column(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                children: [
                                  Text('Default sound', style: theme.textTheme.bodyMedium),
                                  Text('Sound played when a reminder fires', style: theme.textTheme.bodySmall),
                                ],
                              ),
                            ],
                          ),
                        ),
                        ListTile(
                          dense: true,
                          leading: Radio<String?>(
                            value: null,
                            groupValue: _defaultSound,
                            onChanged: (v) => setState(() => _defaultSound = v),
                          ),
                          title: const Text('None'),
                          onTap: () => setState(() => _defaultSound = null),
                        ),
                        ..._sounds.map((s) => ListTile(
                          dense: true,
                          leading: Radio<String?>(
                            value: s,
                            groupValue: _defaultSound,
                            onChanged: (v) => setState(() => _defaultSound = v),
                          ),
                          title: Text(s),
                          trailing: IconButton(
                            icon: const Icon(Icons.play_arrow_rounded, size: 18),
                            onPressed: () {},
                          ),
                          onTap: () => setState(() => _defaultSound = s),
                        )),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 12),

                // Lead time + Snooze card
                Card(
                  child: Padding(
                    padding: const EdgeInsets.symmetric(vertical: 8),
                    child: Column(
                      children: [
                        Padding(
                          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                          child: Row(
                            children: [
                              Container(
                                width: 36,
                                height: 36,
                                decoration: BoxDecoration(
                                  color: colorScheme.primary.withValues(alpha: 0.12),
                                  borderRadius: BorderRadius.circular(8),
                                ),
                                child: Center(child: HeroIcon(HeroIcons.bellAlert, size: 18, color: colorScheme.primary)),
                              ),
                              const SizedBox(width: 12),
                              Expanded(
                                child: Column(
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  children: [
                                    Text('Remind me before', style: theme.textTheme.bodyMedium),
                                    Text('How early to trigger the alarm', style: theme.textTheme.bodySmall),
                                  ],
                                ),
                              ),
                              const SizedBox(width: 8),
                              _SegmentedPicker<int>(
                                options: _leadOptions.map((o) => _PickerOption(value: o.value, label: o.label)).toList(),
                                selected: _leadTimeMinutes,
                                onSelected: (v) => setState(() => _leadTimeMinutes = v),
                              ),
                            ],
                          ),
                        ),
                        const Divider(height: 1, indent: 16, endIndent: 16),
                        Padding(
                          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                          child: Row(
                            children: [
                              Container(
                                width: 36,
                                height: 36,
                                decoration: BoxDecoration(
                                  color: colorScheme.primary.withValues(alpha: 0.12),
                                  borderRadius: BorderRadius.circular(8),
                                ),
                                child: Center(child: HeroIcon(HeroIcons.pause, size: 18, color: colorScheme.primary)),
                              ),
                              const SizedBox(width: 12),
                              Expanded(
                                child: Column(
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  children: [
                                    Text('Snooze duration', style: theme.textTheme.bodyMedium),
                                    Text('How long to snooze when dismissed', style: theme.textTheme.bodySmall),
                                  ],
                                ),
                              ),
                              const SizedBox(width: 8),
                              _SegmentedPicker<int>(
                                options: _snoozeOptions.map((o) => _PickerOption(value: o.value, label: o.label)).toList(),
                                selected: _snoozeDurationMinutes,
                                onSelected: (v) => setState(() => _snoozeDurationMinutes = v),
                              ),
                            ],
                          ),
                        ),
                        const Divider(height: 1, indent: 16, endIndent: 16),
                        ListTile(
                          leading: Container(
                            width: 36,
                            height: 36,
                            decoration: BoxDecoration(
                              color: colorScheme.primary.withValues(alpha: 0.12),
                              borderRadius: BorderRadius.circular(8),
                            ),
                            child: Center(child: HeroIcon(HeroIcons.arrowPath, size: 18, color: colorScheme.primary)),
                          ),
                          title: const Text('Repeat alarm'),
                          subtitle: const Text('Keep alerting until manually dismissed'),
                          trailing: Transform.scale(
                            scale: 0.8,
                            child: Switch(
                              value: _repeatAlarm,
                              onChanged: (v) => setState(() => _repeatAlarm = v),
                            ),
                          ),
                        ),
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

class _TimeOption {
  final int value;
  final String label;
  const _TimeOption({required this.value, required this.label});
}

class _PickerOption<T> {
  final T value;
  final String label;
  const _PickerOption({required this.value, required this.label});
}

class _SegmentedPicker<T> extends StatelessWidget {
  final List<_PickerOption<T>> options;
  final T selected;
  final ValueChanged<T> onSelected;

  const _SegmentedPicker({required this.options, required this.selected, required this.onSelected});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Container(
      decoration: BoxDecoration(
        color: theme.colorScheme.surfaceContainerHighest,
        borderRadius: BorderRadius.circular(8),
      ),
      padding: const EdgeInsets.all(3),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: options.map((opt) {
          final isSelected = opt.value == selected;
          return GestureDetector(
            onTap: () => onSelected(opt.value),
            child: AnimatedContainer(
              duration: const Duration(milliseconds: 150),
              padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 5),
              decoration: BoxDecoration(
                color: isSelected ? theme.colorScheme.surface : Colors.transparent,
                borderRadius: BorderRadius.circular(6),
              ),
              child: Text(
                opt.label,
                style: theme.textTheme.labelSmall?.copyWith(
                  fontWeight: FontWeight.w600,
                  color: isSelected ? theme.colorScheme.onSurface : theme.colorScheme.onSurfaceVariant,
                ),
              ),
            ),
          );
        }).toList(),
      ),
    );
  }
}
