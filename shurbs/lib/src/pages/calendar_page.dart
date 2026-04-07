import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

import '../controllers/calendar_controller.dart';
import '../models/calendar_event_model.dart';

// ── Entry point ───────────────────────────────────────────────────────────────

class CalendarPage extends StatefulWidget {
  final CalendarController controller;

  const CalendarPage({super.key, required this.controller});

  @override
  State<CalendarPage> createState() => _CalendarPageState();
}

// ── View modes ────────────────────────────────────────────────────────────────

enum _View { monthly, weekly, daily }

// ── State ─────────────────────────────────────────────────────────────────────

class _CalendarPageState extends State<CalendarPage> {
  _View _view = _View.monthly;
  DateTime _focused = DateTime(DateTime.now().year, DateTime.now().month);
  DateTime _selected = DateTime(
    DateTime.now().year,
    DateTime.now().month,
    DateTime.now().day,
  );

  // ── Navigation ─────────────────────────────────────────────────────────────

  void _prev() {
    setState(() {
      switch (_view) {
        case _View.monthly:
          _focused = DateTime(_focused.year, _focused.month - 1);
          break;
        case _View.weekly:
          _focused = _focused.subtract(const Duration(days: 7));
          _selected = _selected.subtract(const Duration(days: 7));
          break;
        case _View.daily:
          _selected = _selected.subtract(const Duration(days: 1));
          _focused = _selected;
          break;
      }
    });
  }

  void _next() {
    setState(() {
      switch (_view) {
        case _View.monthly:
          _focused = DateTime(_focused.year, _focused.month + 1);
          break;
        case _View.weekly:
          _focused = _focused.add(const Duration(days: 7));
          _selected = _selected.add(const Duration(days: 7));
          break;
        case _View.daily:
          _selected = _selected.add(const Duration(days: 1));
          _focused = _selected;
          break;
      }
    });
  }

  void _onDayTap(DateTime day) {
    setState(() {
      _selected = day;
      _focused = DateTime(day.year, day.month);
    });
    if (_view == _View.monthly) {
      _showDaySheet(day);
    }
  }

  // ── Header label ───────────────────────────────────────────────────────────

  String get _headerLabel {
    const months = [
      'January', 'February', 'March', 'April', 'May', 'June',
      'July', 'August', 'September', 'October', 'November', 'December'
    ];
    switch (_view) {
      case _View.monthly:
        return '${months[_focused.month - 1]} ${_focused.year}';
      case _View.weekly:
        final weekStart = _mondayOf(_focused);
        final weekEnd = weekStart.add(const Duration(days: 6));
        if (weekStart.month == weekEnd.month) {
          return '${months[weekStart.month - 1]} ${weekStart.year}';
        }
        return '${months[weekStart.month - 1]} – ${months[weekEnd.month - 1]} ${weekEnd.year}';
      case _View.daily:
        const days = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
        return '${days[_selected.weekday - 1]}, ${months[_selected.month - 1]} ${_selected.day}';
    }
  }

  // ── Add event sheet ────────────────────────────────────────────────────────

  void _showAddEventSheet([DateTime? forDate]) {
    showAddEventSheet(context, widget.controller, forDate ?? _selected);
  }

  void _showDaySheet(DateTime day) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (_) => _DaySheet(day: day, controller: widget.controller),
    );
  }

  // ── Build ──────────────────────────────────────────────────────────────────

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final screenH = MediaQuery.of(context).size.height;

    return ListenableBuilder(
      listenable: widget.controller,
      builder: (context, _) => Scaffold(
        body: SafeArea(
          child: Column(
          children: [
            // ── View toggle ─────────────────────────────────────────────────
            Padding(
              padding: const EdgeInsets.fromLTRB(16, 8, 16, 0),
              child: SegmentedButton<_View>(
                segments: const [
                  ButtonSegment(value: _View.daily, label: Text('Day')),
                  ButtonSegment(value: _View.weekly, label: Text('Week')),
                  ButtonSegment(value: _View.monthly, label: Text('Month')),
                ],
                selected: {_view},
                onSelectionChanged: (s) => setState(() => _view = s.first),
                style: const ButtonStyle(
                  tapTargetSize: MaterialTapTargetSize.shrinkWrap,
                  visualDensity: VisualDensity.compact,
                ),
                expandedInsets: EdgeInsets.zero,
              ),
            ),

            // ── Nav row ─────────────────────────────────────────────────────
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  IconButton(
                    onPressed: _prev,
                    icon: const HeroIcon(HeroIcons.chevronLeft, size: 20),
                    visualDensity: VisualDensity.compact,
                  ),
                  GestureDetector(
                    onTap: () => setState(() {
                      final now = DateTime.now();
                      _selected = DateTime(now.year, now.month, now.day);
                      _focused = DateTime(now.year, now.month);
                    }),
                    child: Text(
                      _headerLabel,
                      style: Theme.of(context).textTheme.titleMedium?.copyWith(
                            fontWeight: FontWeight.w700,
                            letterSpacing: -0.3,
                          ),
                    ),
                  ),
                  IconButton(
                    onPressed: _next,
                    icon: const HeroIcon(HeroIcons.chevronRight, size: 20),
                    visualDensity: VisualDensity.compact,
                  ),
                ],
              ),
            ),

            // ── Calendar body ───────────────────────────────────────────────
            if (_view == _View.monthly)
              SizedBox(
                height: screenH * 0.35,
                child: _MonthView(
                  focused: _focused,
                  selected: _selected,
                  controller: widget.controller,
                  onDayTap: _onDayTap,
                ),
              )
            else
              Expanded(
                child: _view == _View.weekly
                    ? _WeekView(
                        focused: _focused,
                        selected: _selected,
                        controller: widget.controller,
                        onDayTap: (d) => setState(() => _selected = d),
                      )
                    : _DayView(
                        day: _selected,
                        controller: widget.controller,
                        onAddEvent: _showAddEventSheet,
                      ),
              ),
          ],
          ),
        ),
        floatingActionButton: FloatingActionButton(
          onPressed: () => _showAddEventSheet(),
          backgroundColor: cs.primary,
          foregroundColor: cs.onPrimary,
          child: const HeroIcon(HeroIcons.plus, size: 22),
        ),
      ),
    );
  }
}

// ── Month view ────────────────────────────────────────────────────────────────

class _MonthView extends StatelessWidget {
  final DateTime focused;
  final DateTime selected;
  final CalendarController controller;
  final ValueChanged<DateTime> onDayTap;

  const _MonthView({
    required this.focused,
    required this.selected,
    required this.controller,
    required this.onDayTap,
  });

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final theme = Theme.of(context);

    // Day-of-week headers (Mon first)
    const dow = ['M', 'T', 'W', 'T', 'F', 'S', 'S'];

    final firstOfMonth = DateTime(focused.year, focused.month, 1);
    // Dart: Monday=1 … Sunday=7; we want Mon=0 offset
    final startOffset = (firstOfMonth.weekday - 1) % 7;
    final daysInMonth = DateTime(focused.year, focused.month + 1, 0).day;
    final totalCells = startOffset + daysInMonth;
    final rows = (totalCells / 7).ceil();
    final today = DateTime.now();
    final todayDay = DateTime(today.year, today.month, today.day);

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 12),
      child: Column(
        children: [
          // DOW header
          Row(
            children: dow.map((d) => Expanded(
              child: Center(
                child: Text(
                  d,
                  style: theme.textTheme.labelSmall?.copyWith(
                    color: cs.onSurfaceVariant,
                    fontWeight: FontWeight.w600,
                  ),
                ),
              ),
            )).toList(),
          ),
          const SizedBox(height: 4),
          // Grid
          Expanded(
            child: Column(
              children: List.generate(rows, (row) {
                return Expanded(
                  child: Row(
                    children: List.generate(7, (col) {
                      final cellIndex = row * 7 + col;
                      final dayNum = cellIndex - startOffset + 1;
                      final inMonth = dayNum >= 1 && dayNum <= daysInMonth;

                      if (!inMonth) {
                        // Show adjacent-month day (faded)
                        int adjDay;
                        DateTime adjDate;
                        if (dayNum < 1) {
                          final prevMonth = DateTime(focused.year, focused.month, 0);
                          adjDay = prevMonth.day + dayNum;
                          adjDate = DateTime(prevMonth.year, prevMonth.month, adjDay);
                        } else {
                          adjDay = dayNum - daysInMonth;
                          adjDate = DateTime(focused.year, focused.month + 1, adjDay);
                        }
                        return Expanded(
                          child: _DayCell(
                            date: adjDate,
                            label: '$adjDay',
                            faded: true,
                            isToday: false,
                            isSelected: false,
                            dots: const [],
                            onTap: () => onDayTap(adjDate),
                          ),
                        );
                      }

                      final date = DateTime(focused.year, focused.month, dayNum);
                      final isToday = date == todayDay;
                      final isSelected =
                          date == DateTime(selected.year, selected.month, selected.day);

                      return Expanded(
                        child: _DayCell(
                          date: date,
                          label: '$dayNum',
                          faded: false,
                          isToday: isToday,
                          isSelected: isSelected,
                          dots: controller.dotColorsForDate(date),
                          onTap: () => onDayTap(date),
                        ),
                      );
                    }),
                  ),
                );
              }),
            ),
          ),
        ],
      ),
    );
  }
}

class _DayCell extends StatelessWidget {
  final DateTime date;
  final String label;
  final bool faded;
  final bool isToday;
  final bool isSelected;
  final List<Color> dots;
  final VoidCallback onTap;

  const _DayCell({
    required this.date,
    required this.label,
    required this.faded,
    required this.isToday,
    required this.isSelected,
    required this.dots,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final theme = Theme.of(context);

    Color? bgColor;
    Color labelColor = faded ? cs.onSurface.withValues(alpha: 0.3) : cs.onSurface;

    if (isSelected) {
      bgColor = cs.primary;
      labelColor = cs.onPrimary;
    } else if (isToday) {
      bgColor = cs.primary.withValues(alpha: 0.12);
      labelColor = cs.primary;
    }

    return GestureDetector(
      onTap: onTap,
      child: Center(
        child: AspectRatio(
          aspectRatio: 1,
          child: Container(
            margin: const EdgeInsets.all(1),
            decoration: BoxDecoration(
              color: bgColor,
              borderRadius: BorderRadius.circular(10),
            ),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Text(
                  label,
                  style: theme.textTheme.bodySmall?.copyWith(
                    color: labelColor,
                    fontWeight: isToday || isSelected ? FontWeight.w700 : FontWeight.w400,
                  ),
                ),
                if (dots.isNotEmpty && !isSelected) ...[
                  const SizedBox(height: 3),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: dots
                        .map((c) => Container(
                              width: 4,
                              height: 4,
                              margin: const EdgeInsets.symmetric(horizontal: 1),
                              decoration: BoxDecoration(color: c, shape: BoxShape.circle),
                            ))
                        .toList(),
                  ),
                ],
              ],
            ),
          ),
        ),
      ),
    );
  }
}

// ── Week view ─────────────────────────────────────────────────────────────────

class _WeekView extends StatelessWidget {
  final DateTime focused;
  final DateTime selected;
  final CalendarController controller;
  final ValueChanged<DateTime> onDayTap;

  const _WeekView({
    required this.focused,
    required this.selected,
    required this.controller,
    required this.onDayTap,
  });

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final theme = Theme.of(context);
    final weekStart = _mondayOf(focused);
    final today = DateTime.now();
    final todayDay = DateTime(today.year, today.month, today.day);
    const dayLabels = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];

    final items = controller.itemsForDate(selected);

    return Column(
      children: [
        // 7-day strip
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
          child: Row(
            children: List.generate(7, (i) {
              final day = weekStart.add(Duration(days: i));
              final isToday = day == todayDay;
              final isSelected =
                  day == DateTime(selected.year, selected.month, selected.day);
              final hasDots = controller.hasItemsOnDate(day);

              return Expanded(
                child: GestureDetector(
                  onTap: () => onDayTap(day),
                  child: AnimatedContainer(
                    duration: const Duration(milliseconds: 180),
                    margin: const EdgeInsets.symmetric(horizontal: 2),
                    padding: const EdgeInsets.symmetric(vertical: 8),
                    decoration: BoxDecoration(
                      color: isSelected
                          ? cs.primary
                          : isToday
                              ? cs.primary.withValues(alpha: 0.1)
                              : Colors.transparent,
                      borderRadius: BorderRadius.circular(12),
                    ),
                    child: Column(
                      children: [
                        Text(
                          dayLabels[i],
                          style: theme.textTheme.labelSmall?.copyWith(
                            color: isSelected
                                ? cs.onPrimary
                                : isToday
                                    ? cs.primary
                                    : cs.onSurfaceVariant,
                            fontWeight: FontWeight.w600,
                          ),
                        ),
                        const SizedBox(height: 4),
                        Text(
                          '${day.day}',
                          style: theme.textTheme.bodyMedium?.copyWith(
                            color: isSelected
                                ? cs.onPrimary
                                : isToday
                                    ? cs.primary
                                    : cs.onSurface,
                            fontWeight: isSelected || isToday
                                ? FontWeight.w700
                                : FontWeight.w400,
                          ),
                        ),
                        const SizedBox(height: 4),
                        Container(
                          width: 5,
                          height: 5,
                          decoration: BoxDecoration(
                            color: hasDots
                                ? (isSelected ? cs.onPrimary : cs.primary)
                                : Colors.transparent,
                            shape: BoxShape.circle,
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              );
            }),
          ),
        ),
        Divider(height: 1, color: cs.outlineVariant.withValues(alpha: 0.4)),
        // Items for selected day
        Expanded(
          child: items.isEmpty
              ? _EmptyDay(day: selected)
              : _ItemList(items: items, day: selected),
        ),
      ],
    );
  }
}

// ── Day view ──────────────────────────────────────────────────────────────────

class _DayView extends StatelessWidget {
  final DateTime day;
  final CalendarController controller;
  final VoidCallback onAddEvent;

  const _DayView({
    required this.day,
    required this.controller,
    required this.onAddEvent,
  });

  @override
  Widget build(BuildContext context) {
    final items = controller.itemsForDate(day);

    if (items.isEmpty) return _EmptyDay(day: day, onAdd: onAddEvent);

    return _ItemList(items: items, day: day);
  }
}

// ── Shared: item list ─────────────────────────────────────────────────────────

class _ItemList extends StatelessWidget {
  final List<CalendarDayItem> items;
  final DateTime day;

  const _ItemList({required this.items, required this.day});

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final theme = Theme.of(context);

    final allDay = items.where((i) => i.startMinute == null).toList();
    final timed = items.where((i) => i.startMinute != null).toList();

    return ListView(
      padding: const EdgeInsets.fromLTRB(16, 12, 16, 80),
      children: [
        if (allDay.isNotEmpty) ...[
          Text(
            'All Day',
            style: theme.textTheme.labelSmall?.copyWith(
              color: cs.onSurfaceVariant,
              letterSpacing: 0.8,
              fontWeight: FontWeight.w600,
            ),
          ),
          const SizedBox(height: 8),
          ...allDay.map((item) => _CalendarItemCard(item: item)),
          const SizedBox(height: 16),
        ],
        if (timed.isNotEmpty) ...[
          Text(
            'Scheduled',
            style: theme.textTheme.labelSmall?.copyWith(
              color: cs.onSurfaceVariant,
              letterSpacing: 0.8,
              fontWeight: FontWeight.w600,
            ),
          ),
          const SizedBox(height: 8),
          ...timed.map((item) => _CalendarItemCard(item: item)),
        ],
      ],
    );
  }
}

class _CalendarItemCard extends StatelessWidget {
  final CalendarDayItem item;
  const _CalendarItemCard({required this.item});

  HeroIcons get _icon {
    switch (item.type) {
      case CalendarItemType.todo:
        return HeroIcons.checkCircle;
      case CalendarItemType.reminder:
        return HeroIcons.clock;
      case CalendarItemType.event:
        return HeroIcons.calendarDays;
    }
  }

  String get _typeLabel {
    switch (item.type) {
      case CalendarItemType.todo:
        return 'Todo';
      case CalendarItemType.reminder:
        return 'Reminder';
      case CalendarItemType.event:
        return 'Event';
    }
  }

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final theme = Theme.of(context);

    return Container(
      margin: const EdgeInsets.only(bottom: 8),
      padding: const EdgeInsets.all(14),
      decoration: BoxDecoration(
        color: cs.surface,
        borderRadius: BorderRadius.circular(14),
        border: Border.all(color: cs.outlineVariant.withValues(alpha: 0.4)),
        boxShadow: [
          BoxShadow(
            color: cs.shadow.withValues(alpha: 0.04),
            blurRadius: 8,
            offset: const Offset(0, 2),
          ),
        ],
      ),
      child: Row(
        children: [
          Container(
            width: 36,
            height: 36,
            decoration: BoxDecoration(
              color: item.color.withValues(alpha: 0.12),
              borderRadius: BorderRadius.circular(10),
            ),
            child: Center(child: HeroIcon(_icon, size: 18, color: item.color)),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  item.title,
                  style: theme.textTheme.bodyMedium?.copyWith(
                    fontWeight: FontWeight.w600,
                    decoration: item.completed ? TextDecoration.lineThrough : null,
                    color: item.completed ? cs.onSurfaceVariant : null,
                  ),
                  maxLines: 2,
                  overflow: TextOverflow.ellipsis,
                ),
                if (item.description != null && item.description!.isNotEmpty) ...[
                  const SizedBox(height: 2),
                  Text(
                    item.description!,
                    style: theme.textTheme.bodySmall?.copyWith(color: cs.onSurfaceVariant),
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                  ),
                ],
              ],
            ),
          ),
          const SizedBox(width: 8),
          Column(
            crossAxisAlignment: CrossAxisAlignment.end,
            children: [
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 7, vertical: 3),
                decoration: BoxDecoration(
                  color: item.color.withValues(alpha: 0.1),
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Text(
                  _typeLabel,
                  style: theme.textTheme.labelSmall?.copyWith(
                    color: item.color,
                    fontWeight: FontWeight.w600,
                  ),
                ),
              ),
              if (item.startMinute != null) ...[
                const SizedBox(height: 4),
                Text(
                  item.timeLabel,
                  style: theme.textTheme.labelSmall?.copyWith(
                    color: cs.onSurfaceVariant,
                  ),
                ),
              ],
            ],
          ),
        ],
      ),
    );
  }
}

// ── Empty state ───────────────────────────────────────────────────────────────

class _EmptyDay extends StatelessWidget {
  final DateTime day;
  final VoidCallback? onAdd;
  const _EmptyDay({required this.day, this.onAdd});

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final theme = Theme.of(context);

    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Container(
            padding: const EdgeInsets.all(20),
            decoration: BoxDecoration(
              color: cs.primary.withValues(alpha: 0.08),
              shape: BoxShape.circle,
            ),
            child: HeroIcon(HeroIcons.calendarDays, size: 36, color: cs.primary),
          ),
          const SizedBox(height: 16),
          Text(
            'Nothing scheduled',
            style: theme.textTheme.titleSmall?.copyWith(fontWeight: FontWeight.w600),
          ),
          const SizedBox(height: 6),
          Text(
            'Tap + to add an event',
            style: theme.textTheme.bodySmall?.copyWith(color: cs.onSurfaceVariant),
          ),
          if (onAdd != null) ...[
            const SizedBox(height: 20),
            FilledButton.tonal(
              onPressed: onAdd,
              child: const Text('Add Event'),
            ),
          ],
        ],
      ),
    );
  }
}

// ── Day bottom sheet (month-view tap) ─────────────────────────────────────────

class _DaySheet extends StatelessWidget {
  final DateTime day;
  final CalendarController controller;

  const _DaySheet({required this.day, required this.controller});

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final theme = Theme.of(context);
    const months = [
      'January', 'February', 'March', 'April', 'May', 'June',
      'July', 'August', 'September', 'October', 'November', 'December'
    ];
    const days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'];
    final label = '${days[day.weekday - 1]}, ${months[day.month - 1]} ${day.day}';
    final items = controller.itemsForDate(day);

    return Container(
      decoration: BoxDecoration(
        color: cs.surface,
        borderRadius: const BorderRadius.vertical(top: Radius.circular(24)),
      ),
      constraints: BoxConstraints(
        maxHeight: MediaQuery.of(context).size.height * 0.65,
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          // handle
          const SizedBox(height: 12),
          Container(
            width: 36,
            height: 4,
            decoration: BoxDecoration(
              color: cs.outlineVariant,
              borderRadius: BorderRadius.circular(2),
            ),
          ),
          const SizedBox(height: 16),
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 20),
            child: Row(
              children: [
                Text(
                  label,
                  style: theme.textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w700),
                ),
                const Spacer(),
                FilledButton.tonal(
                  onPressed: () {
                    Navigator.pop(context);
                    showAddEventSheet(context, controller, day);
                  },
                  style: FilledButton.styleFrom(
                    visualDensity: VisualDensity.compact,
                    padding: const EdgeInsets.symmetric(horizontal: 12),
                  ),
                  child: const Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      HeroIcon(HeroIcons.plus, size: 14),
                      SizedBox(width: 4),
                      Text('Add'),
                    ],
                  ),
                ),
              ],
            ),
          ),
          const SizedBox(height: 12),
          Flexible(
            child: items.isEmpty
                ? Padding(
                    padding: const EdgeInsets.all(24),
                    child: Text(
                      'Nothing on this day',
                      style: theme.textTheme.bodyMedium?.copyWith(
                        color: cs.onSurfaceVariant,
                      ),
                    ),
                  )
                : ListView.builder(
                    shrinkWrap: true,
                    padding: const EdgeInsets.fromLTRB(16, 0, 16, 24),
                    itemCount: items.length,
                    itemBuilder: (_, i) => _CalendarItemCard(item: items[i]),
                  ),
          ),
        ],
      ),
    );
  }
}

// ── Add event sheet ───────────────────────────────────────────────────────────

void showAddEventSheet(
  BuildContext context,
  CalendarController controller,
  DateTime initialDate,
) {
  showModalBottomSheet(
    context: context,
    isScrollControlled: true,
    backgroundColor: Colors.transparent,
    builder: (_) => _AddEventSheet(controller: controller, initialDate: initialDate),
  );
}

class _AddEventSheet extends StatefulWidget {
  final CalendarController controller;
  final DateTime initialDate;

  const _AddEventSheet({required this.controller, required this.initialDate});

  @override
  State<_AddEventSheet> createState() => _AddEventSheetState();
}

class _AddEventSheetState extends State<_AddEventSheet> {
  final _titleCtrl = TextEditingController();
  final _descCtrl = TextEditingController();
  late DateTime _date;
  bool _allDay = true;
  TimeOfDay _startTime = TimeOfDay.now();
  TimeOfDay _endTime = TimeOfDay(
    hour: (TimeOfDay.now().hour + 1) % 24,
    minute: TimeOfDay.now().minute,
  );
  CalendarEventColor _color = CalendarEventColor.indigo;

  @override
  void initState() {
    super.initState();
    _date = widget.initialDate;
  }

  @override
  void dispose() {
    _titleCtrl.dispose();
    _descCtrl.dispose();
    super.dispose();
  }

  void _submit() {
    if (_titleCtrl.text.trim().isEmpty) return;

    final event = CalendarEvent(
      id: DateTime.now().millisecondsSinceEpoch.toString(),
      title: _titleCtrl.text.trim(),
      description: _descCtrl.text.trim().isEmpty ? null : _descCtrl.text.trim(),
      date: _date,
      startMinute: _allDay ? null : _startTime.hour * 60 + _startTime.minute,
      endMinute: _allDay ? null : _endTime.hour * 60 + _endTime.minute,
      color: _color,
    );

    widget.controller.addEvent(event);
    Navigator.pop(context);
  }

  Future<void> _pickDate() async {
    final picked = await showDatePicker(
      context: context,
      initialDate: _date,
      firstDate: DateTime(2020),
      lastDate: DateTime(2035),
    );
    if (picked != null) setState(() => _date = picked);
  }

  Future<void> _pickStartTime() async {
    final picked = await showTimePicker(context: context, initialTime: _startTime);
    if (picked != null) setState(() => _startTime = picked);
  }

  Future<void> _pickEndTime() async {
    final picked = await showTimePicker(context: context, initialTime: _endTime);
    if (picked != null) setState(() => _endTime = picked);
  }

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final theme = Theme.of(context);
    const months = [
      'Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun',
      'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'
    ];

    return Container(
      decoration: BoxDecoration(
        color: cs.surface,
        borderRadius: const BorderRadius.vertical(top: Radius.circular(24)),
      ),
      padding: EdgeInsets.only(
        left: 20,
        right: 20,
        top: 20,
        bottom: MediaQuery.of(context).viewInsets.bottom + 24,
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Handle
          Center(
            child: Container(
              width: 36,
              height: 4,
              decoration: BoxDecoration(
                color: cs.outlineVariant,
                borderRadius: BorderRadius.circular(2),
              ),
            ),
          ),
          const SizedBox(height: 16),
          Text('New Event', style: theme.textTheme.titleLarge?.copyWith(fontWeight: FontWeight.w700)),
          const SizedBox(height: 16),

          // Title
          TextField(
            controller: _titleCtrl,
            autofocus: true,
            decoration: InputDecoration(
              hintText: 'Event title',
              border: OutlineInputBorder(borderRadius: BorderRadius.circular(12)),
              contentPadding: const EdgeInsets.symmetric(horizontal: 14, vertical: 12),
            ),
          ),
          const SizedBox(height: 10),

          // Description
          TextField(
            controller: _descCtrl,
            decoration: InputDecoration(
              hintText: 'Description (optional)',
              border: OutlineInputBorder(borderRadius: BorderRadius.circular(12)),
              contentPadding: const EdgeInsets.symmetric(horizontal: 14, vertical: 12),
            ),
          ),
          const SizedBox(height: 12),

          // Date row
          Row(
            children: [
              HeroIcon(HeroIcons.calendarDays, size: 16, color: cs.onSurfaceVariant),
              const SizedBox(width: 8),
              GestureDetector(
                onTap: _pickDate,
                child: Container(
                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 7),
                  decoration: BoxDecoration(
                    color: cs.surfaceContainerHighest.withValues(alpha: 0.6),
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Text(
                    '${months[_date.month - 1]} ${_date.day}, ${_date.year}',
                    style: theme.textTheme.bodySmall?.copyWith(fontWeight: FontWeight.w600),
                  ),
                ),
              ),
              const Spacer(),
              Text('All day', style: theme.textTheme.bodySmall),
              const SizedBox(width: 6),
              Switch.adaptive(
                value: _allDay,
                onChanged: (v) => setState(() => _allDay = v),
                materialTapTargetSize: MaterialTapTargetSize.shrinkWrap,
              ),
            ],
          ),

          // Time row
          if (!_allDay) ...[
            const SizedBox(height: 8),
            Row(
              children: [
                HeroIcon(HeroIcons.clock, size: 16, color: cs.onSurfaceVariant),
                const SizedBox(width: 8),
                GestureDetector(
                  onTap: _pickStartTime,
                  child: Container(
                    padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 7),
                    decoration: BoxDecoration(
                      color: cs.surfaceContainerHighest.withValues(alpha: 0.6),
                      borderRadius: BorderRadius.circular(8),
                    ),
                    child: Text(
                      _startTime.format(context),
                      style: theme.textTheme.bodySmall?.copyWith(fontWeight: FontWeight.w600),
                    ),
                  ),
                ),
                Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 8),
                  child: Text('→', style: theme.textTheme.bodySmall),
                ),
                GestureDetector(
                  onTap: _pickEndTime,
                  child: Container(
                    padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 7),
                    decoration: BoxDecoration(
                      color: cs.surfaceContainerHighest.withValues(alpha: 0.6),
                      borderRadius: BorderRadius.circular(8),
                    ),
                    child: Text(
                      _endTime.format(context),
                      style: theme.textTheme.bodySmall?.copyWith(fontWeight: FontWeight.w600),
                    ),
                  ),
                ),
              ],
            ),
          ],
          const SizedBox(height: 12),

          // Color picker
          Row(
            children: [
              Text('Color:', style: theme.textTheme.bodySmall?.copyWith(color: cs.onSurfaceVariant)),
              const SizedBox(width: 10),
              ...CalendarEventColor.values.map((c) => GestureDetector(
                    onTap: () => setState(() => _color = c),
                    child: Container(
                      width: 24,
                      height: 24,
                      margin: const EdgeInsets.only(right: 6),
                      decoration: BoxDecoration(
                        color: c.value,
                        shape: BoxShape.circle,
                        border: _color == c
                            ? Border.all(color: cs.onSurface, width: 2.5)
                            : null,
                      ),
                    ),
                  )),
            ],
          ),
          const SizedBox(height: 16),

          // Submit
          SizedBox(
            width: double.infinity,
            child: FilledButton(
              onPressed: _submit,
              style: FilledButton.styleFrom(
                padding: const EdgeInsets.symmetric(vertical: 14),
                shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
              ),
              child: const Text('Save Event', style: TextStyle(fontWeight: FontWeight.w600)),
            ),
          ),
        ],
      ),
    );
  }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Returns the Monday of the week containing [date].
DateTime _mondayOf(DateTime date) {
  return date.subtract(Duration(days: date.weekday - 1));
}
