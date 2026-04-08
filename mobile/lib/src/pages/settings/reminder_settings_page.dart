import 'package:audioplayers/audioplayers.dart';
import 'package:flutter/material.dart';

import '../../models/reminder_model.dart';

class ReminderSettingsPage extends StatefulWidget {
  const ReminderSettingsPage({super.key});

  @override
  State<ReminderSettingsPage> createState() => _ReminderSettingsPageState();
}

class _ReminderSettingsPageState extends State<ReminderSettingsPage> {
  AlarmSound? _defaultSound;
  int _leadTimeMinutes = 5;
  int _snoozeDurationMinutes = 5;
  bool _repeatReminder = false;

  AlarmSound? _playing;
  final _player = AudioPlayer();

  static const _leadOptions = [
    (value: 0, label: 'At time'),
    (value: 5, label: '5 min'),
    (value: 10, label: '10 min'),
    (value: 15, label: '15 min'),
    (value: 20, label: '20 min'),
  ];

  static const _snoozeOptions = [
    (value: 5, label: '5 min'),
    (value: 10, label: '10 min'),
    (value: 15, label: '15 min'),
  ];

  @override
  void initState() {
    super.initState();
    _player.onPlayerComplete.listen((_) {
      if (mounted) setState(() => _playing = null);
    });
  }

  @override
  void dispose() {
    _player.dispose();
    super.dispose();
  }

  Future<void> _togglePlay(AlarmSound sound) async {
    if (_playing == sound) {
      await _player.stop();
      setState(() => _playing = null);
    } else {
      await _player.stop();
      await _player.play(AssetSource(sound.assetPath.replaceFirst('assets/', '')));
      setState(() => _playing = sound);
    }
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;
    final isDark = theme.brightness == Brightness.dark;

    return Scaffold(
      appBar: AppBar(
        title: const Text(
          'Reminder',
          style: TextStyle(color: Colors.black, fontWeight: FontWeight.w600),
        ),
        foregroundColor: Colors.black,
        backgroundColor: Colors.transparent,
        elevation: 0,
        scrolledUnderElevation: 0,
      ),
      body: ListView(
        children: [
          // ── Default sound ─────────────────────────────────────────────────
          _SectionHeader(
            title: 'Default sound',
            subtitle: 'Sound played when a reminder fires',
            isDark: isDark,
            theme: theme,
          ),

          _SoundRow(
            label: 'None',
            selected: _defaultSound == null,
            playing: false,
            isDark: isDark,
            colorScheme: colorScheme,
            theme: theme,
            onTap: () => setState(() => _defaultSound = null),
            onPlay: null,
          ),

          ...AlarmSound.values.map((s) => _SoundRow(
                label: _soundLabel(s),
                selected: _defaultSound == s,
                playing: _playing == s,
                isDark: isDark,
                colorScheme: colorScheme,
                theme: theme,
                onTap: () => setState(() => _defaultSound = s),
                onPlay: () => _togglePlay(s),
              )),

          const SizedBox(height: 24),

          // ── Remind me before ──────────────────────────────────────────────
          _SectionHeader(
            title: 'Remind me before',
            subtitle: 'How early to trigger the alarm',
            isDark: isDark,
            theme: theme,
          ),
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
            child: _PillRow<int>(
              options: _leadOptions
                  .map((o) => _PillOpt(value: o.value, label: o.label))
                  .toList(),
              selected: _leadTimeMinutes,
              onSelected: (v) => setState(() => _leadTimeMinutes = v),
              colorScheme: colorScheme,
              theme: theme,
              isDark: isDark,
            ),
          ),

          const SizedBox(height: 8),

          // ── Snooze duration ───────────────────────────────────────────────
          _SectionHeader(
            title: 'Snooze duration',
            subtitle: 'How long to snooze when dismissed',
            isDark: isDark,
            theme: theme,
          ),
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
            child: _PillRow<int>(
              options: _snoozeOptions
                  .map((o) => _PillOpt(value: o.value, label: o.label))
                  .toList(),
              selected: _snoozeDurationMinutes,
              onSelected: (v) => setState(() => _snoozeDurationMinutes = v),
              colorScheme: colorScheme,
              theme: theme,
              isDark: isDark,
            ),
          ),

          const SizedBox(height: 8),

          // ── Repeat alarm ──────────────────────────────────────────────────
          _SectionHeader(
            title: 'Repeat alarm',
            subtitle: 'Keep alerting until manually dismissed',
            isDark: isDark,
            theme: theme,
          ),
          Padding(
            padding: const EdgeInsets.fromLTRB(20, 4, 16, 32),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(
                  _repeatReminder ? 'On' : 'Off',
                  style: theme.textTheme.bodyMedium?.copyWith(
                    color: isDark ? Colors.white54 : Colors.black45,
                  ),
                ),
                Switch(
                  value: _repeatReminder,
                  onChanged: (v) => setState(() => _repeatReminder = v),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  static String _soundLabel(AlarmSound s) {
    switch (s) {
      case AlarmSound.funnyAlarm:
        return 'Funny-Alarm-317531';
      case AlarmSound.softPiano:
        return 'Soft-Piano-100-Bpm-121529';
      case AlarmSound.relaxingGuitar:
        return 'Relaxing-Guitar-Loop-V5-245859';
    }
  }
}

// ── Section header ────────────────────────────────────────────────────────────

class _SectionHeader extends StatelessWidget {
  final String title;
  final String subtitle;
  final bool isDark;
  final ThemeData theme;

  const _SectionHeader({
    required this.title,
    required this.subtitle,
    required this.isDark,
    required this.theme,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(20, 24, 20, 4),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            title,
            style: theme.textTheme.bodyLarge?.copyWith(
              fontWeight: FontWeight.w600,
              color: isDark ? Colors.white : Colors.black87,
            ),
          ),
          const SizedBox(height: 2),
          Text(
            subtitle,
            style: theme.textTheme.bodySmall?.copyWith(
              color: isDark ? Colors.white38 : Colors.black45,
            ),
          ),
        ],
      ),
    );
  }
}

// ── Sound row ─────────────────────────────────────────────────────────────────

class _SoundRow extends StatelessWidget {
  final String label;
  final bool selected;
  final bool playing;
  final bool isDark;
  final ColorScheme colorScheme;
  final ThemeData theme;
  final VoidCallback onTap;
  final VoidCallback? onPlay;

  const _SoundRow({
    required this.label,
    required this.selected,
    required this.playing,
    required this.isDark,
    required this.colorScheme,
    required this.theme,
    required this.onTap,
    required this.onPlay,
  });

  @override
  Widget build(BuildContext context) {
    final bg = selected
        ? colorScheme.primary.withValues(alpha: isDark ? 0.22 : 0.10)
        : Colors.transparent;

    return AnimatedContainer(
      duration: const Duration(milliseconds: 160),
      color: bg,
      child: InkWell(
        onTap: onTap,
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 13),
          child: Row(
            children: [
              SizedBox(
                width: 20,
                height: 20,
                child: Radio<bool>(
                  value: true,
                  groupValue: selected,
                  onChanged: (_) => onTap(),
                  activeColor: colorScheme.primary,
                  materialTapTargetSize: MaterialTapTargetSize.shrinkWrap,
                  visualDensity: VisualDensity.compact,
                ),
              ),
              const SizedBox(width: 14),
              Expanded(
                child: Text(
                  label,
                  style: theme.textTheme.bodyMedium?.copyWith(
                    color: isDark ? Colors.white : Colors.black87,
                    fontWeight:
                        selected ? FontWeight.w600 : FontWeight.w400,
                  ),
                ),
              ),
              if (onPlay != null)
                GestureDetector(
                  onTap: onPlay,
                  child: Icon(
                    playing
                        ? Icons.stop_rounded
                        : Icons.play_arrow_rounded,
                    size: 22,
                    color: isDark ? Colors.white54 : Colors.black38,
                  ),
                ),
            ],
          ),
        ),
      ),
    );
  }
}

// ── Pill row ──────────────────────────────────────────────────────────────────

class _PillOpt<T> {
  final T value;
  final String label;
  const _PillOpt({required this.value, required this.label});
}

class _PillRow<T> extends StatelessWidget {
  final List<_PillOpt<T>> options;
  final T selected;
  final ValueChanged<T> onSelected;
  final ColorScheme colorScheme;
  final ThemeData theme;
  final bool isDark;

  const _PillRow({
    required this.options,
    required this.selected,
    required this.onSelected,
    required this.colorScheme,
    required this.theme,
    required this.isDark,
  });

  @override
  Widget build(BuildContext context) {
    return Wrap(
      spacing: 8,
      runSpacing: 8,
      children: options.map((opt) {
        final isSelected = opt.value == selected;
        return GestureDetector(
          onTap: () => onSelected(opt.value),
          child: AnimatedContainer(
            duration: const Duration(milliseconds: 150),
            padding:
                const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
            decoration: BoxDecoration(
              color: isSelected
                  ? colorScheme.primary
                  : (isDark
                      ? Colors.white.withValues(alpha: 0.07)
                      : Colors.black.withValues(alpha: 0.06)),
              borderRadius: BorderRadius.circular(20),
            ),
            child: Text(
              opt.label,
              style: theme.textTheme.labelMedium?.copyWith(
                fontWeight: FontWeight.w600,
                color: isSelected
                    ? Colors.white
                    : (isDark ? Colors.white60 : Colors.black54),
              ),
            ),
          ),
        );
      }).toList(),
    );
  }
}
