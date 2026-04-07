import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

import '../controllers/home_controller.dart';
import '../models/todo_model.dart';


class HomePage extends StatelessWidget {
  final HomeController controller;

  const HomePage({super.key, required this.controller});

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: controller,
      builder: (context, _) => Scaffold(
        body: SafeArea(
          child: CustomScrollView(
            slivers: [
              SliverPadding(
                padding: const EdgeInsets.symmetric(horizontal: 20),
                sliver: SliverList(
                  delegate: SliverChildListDelegate([
                    const SizedBox(height: 8),
                    _HeroBanner(activeTodoCount: controller.activeTodoCount),
                    const SizedBox(height: 28),
                    const _SectionLabel(text: 'At a Glance'),
                    const SizedBox(height: 12),
                    _StatsRow(
                      todoCount: controller.todoCount,
                      reminderCount: controller.reminderCount,
                      bookmarkCount: controller.bookmarkCount,
                      noteCount: controller.noteCount,
                    ),
                    const SizedBox(height: 28),
                    _FocusSectionHeader(count: controller.activeTodoCount),
                    const SizedBox(height: 12),
                    _FocusCarousel(todos: controller.activeTodos),
                    const SizedBox(height: 28),
                    const _SectionLabel(text: 'Recent Activity'),
                    const SizedBox(height: 12),
                    _ActivityTimeline(items: controller.recentActivity),
                    const SizedBox(height: 40),
                  ]),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

// ─── Section label ────────────────────────────────────────────────────────────

class _SectionLabel extends StatelessWidget {
  final String text;
  const _SectionLabel({required this.text});

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    return Row(
      children: [
        Container(
          width: 3,
          height: 14,
          decoration: BoxDecoration(color: cs.primary, borderRadius: BorderRadius.circular(2)),
        ),
        const SizedBox(width: 8),
        Text(
          text,
          style: Theme.of(context).textTheme.labelLarge?.copyWith(
                color: cs.onSurfaceVariant,
                letterSpacing: 0.8,
              ),
        ),
      ],
    );
  }
}

// ─── Hero banner ──────────────────────────────────────────────────────────────

class _HeroBanner extends StatelessWidget {
  final int activeTodoCount;

  const _HeroBanner({required this.activeTodoCount});

  String get _greeting {
    final h = DateTime.now().hour;
    if (h < 12) return 'Good morning';
    if (h < 17) return 'Good afternoon';
    return 'Good evening';
  }

  String get _date {
    final now = DateTime.now();
    const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
    const days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'];
    return '${days[now.weekday - 1]}, ${months[now.month - 1]} ${now.day}';
  }

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final isDark = Theme.of(context).brightness == Brightness.dark;

    return Container(
      width: double.infinity,
      padding: const EdgeInsets.all(24),
      decoration: BoxDecoration(
        gradient: LinearGradient(
          colors: isDark
              ? [cs.primary.withValues(alpha: 0.30), cs.tertiary.withValues(alpha: 0.15)]
              : [cs.primary, cs.tertiary],
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
        ),
        borderRadius: BorderRadius.circular(24),
      ),
      child: Stack(
        children: [
          Positioned(
            right: -20,
            top: -20,
            child: Container(
              width: 110,
              height: 110,
              decoration: BoxDecoration(
                shape: BoxShape.circle,
                color: Colors.white.withValues(alpha: isDark ? 0.04 : 0.12),
              ),
            ),
          ),
          Positioned(
            right: 30,
            bottom: -30,
            child: Container(
              width: 70,
              height: 70,
              decoration: BoxDecoration(
                shape: BoxShape.circle,
                color: Colors.white.withValues(alpha: isDark ? 0.03 : 0.08),
              ),
            ),
          ),
          Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                _date,
                style: Theme.of(context).textTheme.bodySmall?.copyWith(
                      color: isDark
                          ? cs.onSurface.withValues(alpha: 0.5)
                          : Colors.white.withValues(alpha: 0.75),
                      letterSpacing: 0.5,
                    ),
              ),
              const SizedBox(height: 6),
              Text(
                '$_greeting,',
                style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                      color: isDark ? cs.onSurface : Colors.white,
                      fontWeight: FontWeight.w300,
                      letterSpacing: -0.5,
                    ),
              ),
              Text(
                'Adeoye',
                style: Theme.of(context).textTheme.displaySmall?.copyWith(
                      color: isDark ? cs.onSurface : Colors.white,
                      fontWeight: FontWeight.bold,
                      letterSpacing: -1,
                      height: 1.1,
                    ),
              ),
              const SizedBox(height: 16),
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                decoration: BoxDecoration(
                  color: Colors.white.withValues(alpha: isDark ? 0.08 : 0.2),
                  borderRadius: BorderRadius.circular(20),
                ),
                child: Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Icon(Icons.circle, size: 7, color: isDark ? cs.primary : Colors.white),
                    const SizedBox(width: 6),
                    Text(
                      activeTodoCount == 0
                          ? 'All caught up!'
                          : '$activeTodoCount task${activeTodoCount == 1 ? '' : 's'} pending today',
                      style: Theme.of(context).textTheme.bodySmall?.copyWith(
                            color: isDark ? cs.onSurface.withValues(alpha: 0.8) : Colors.white,
                            fontWeight: FontWeight.w500,
                          ),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}

// ─── Stats row ────────────────────────────────────────────────────────────────

class _StatsRow extends StatelessWidget {
  final int todoCount;
  final int reminderCount;
  final int bookmarkCount;
  final int noteCount;

  const _StatsRow({
    required this.todoCount,
    required this.reminderCount,
    required this.bookmarkCount,
    required this.noteCount,
  });

  @override
  Widget build(BuildContext context) {
    final stats = [
      _Stat('Todos', '$todoCount', HeroIcons.checkCircle, const Color(0xFF6366F1)),
      _Stat('Reminders', '$reminderCount', HeroIcons.clock, const Color(0xFFF59E0B)),
      _Stat('Bookmarks', '$bookmarkCount', HeroIcons.bookmark, const Color(0xFF8B5CF6)),
      _Stat('Notes', '$noteCount', HeroIcons.documentText, const Color(0xFF10B981)),
    ];

    return IntrinsicHeight(
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: stats
            .map((s) => Expanded(
                  child: Padding(
                    padding: EdgeInsets.only(right: s == stats.last ? 0 : 10),
                    child: _StatCard(stat: s),
                  ),
                ))
            .toList(),
      ),
    );
  }
}

class _Stat {
  final String label;
  final String count;
  final HeroIcons icon;
  final Color color;
  const _Stat(this.label, this.count, this.icon, this.color);
}

class _StatCard extends StatelessWidget {
  final _Stat stat;
  const _StatCard({required this.stat});

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;

    return Container(
      padding: const EdgeInsets.symmetric(vertical: 14, horizontal: 10),
      decoration: BoxDecoration(
        color: cs.surfaceContainerHighest.withValues(alpha: 0.5),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: cs.outlineVariant.withValues(alpha: 0.4)),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          HeroIcon(stat.icon, size: 20, color: stat.color),
          const SizedBox(height: 8),
          Text(
            stat.count,
            style: Theme.of(context).textTheme.titleLarge?.copyWith(
                  fontWeight: FontWeight.bold,
                  color: cs.onSurface,
                  height: 1,
                ),
          ),
          const SizedBox(height: 2),
          Text(
            stat.label,
            style: Theme.of(context).textTheme.labelSmall?.copyWith(color: cs.onSurfaceVariant),
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
          ),
          const SizedBox(height: 8),
          Container(
            height: 2,
            decoration: BoxDecoration(
              color: stat.color.withValues(alpha: 0.25),
              borderRadius: BorderRadius.circular(1),
            ),
            child: FractionallySizedBox(
              widthFactor: 0.6,
              child: Container(
                decoration: BoxDecoration(
                  color: stat.color,
                  borderRadius: BorderRadius.circular(1),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

// ─── Today's focus — section header with "View All" ──────────────────────────

class _FocusSectionHeader extends StatelessWidget {
  final int count;
  const _FocusSectionHeader({required this.count});

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    return Row(
      children: [
        Container(
          width: 3,
          height: 14,
          decoration: BoxDecoration(color: cs.primary, borderRadius: BorderRadius.circular(2)),
        ),
        const SizedBox(width: 8),
        Text(
          "Today's Focus",
          style: Theme.of(context).textTheme.labelLarge?.copyWith(
                color: cs.onSurfaceVariant,
                letterSpacing: 0.8,
              ),
        ),
        const Spacer(),
        if (count > 0)
          GestureDetector(
            onTap: () {
              // Bubble up to shell — navigate to todo tab (index 1)
              final scaffold = Scaffold.maybeOf(context);
              if (scaffold != null) {
                // Use a custom notification to tell the shell to navigate
                NavigateToTabNotification(1).dispatch(context);
              }
            },
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                Text(
                  'View all ($count)',
                  style: Theme.of(context).textTheme.labelSmall?.copyWith(
                        color: cs.primary,
                        fontWeight: FontWeight.w600,
                      ),
                ),
                const SizedBox(width: 4),
                HeroIcon(HeroIcons.arrowRight, size: 14, color: cs.primary),
              ],
            ),
          ),
      ],
    );
  }
}

class NavigateToTabNotification extends Notification {
  final int tabIndex;
  const NavigateToTabNotification(this.tabIndex);
}

// ─── Today's focus carousel ───────────────────────────────────────────────────

class _FocusCarousel extends StatefulWidget {
  final List<Todo> todos;
  const _FocusCarousel({required this.todos});

  @override
  State<_FocusCarousel> createState() => _FocusCarouselState();
}

class _FocusCarouselState extends State<_FocusCarousel> {
  int _page = 0;

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;

    if (widget.todos.isEmpty) {
      return Container(
        padding: const EdgeInsets.all(20),
        decoration: BoxDecoration(
          color: cs.surface,
          borderRadius: BorderRadius.circular(20),
          border: Border.all(color: cs.outlineVariant.withValues(alpha: 0.5)),
        ),
        child: Row(
          children: [
            HeroIcon(HeroIcons.checkBadge, size: 28, color: cs.primary),
            const SizedBox(width: 14),
            Text(
              'Nothing pending — great work!',
              style: Theme.of(context).textTheme.bodyMedium?.copyWith(color: cs.onSurfaceVariant),
            ),
          ],
        ),
      );
    }

    return Column(
      children: [
        SizedBox(
          height: 140,
          child: PageView.builder(
            controller: PageController(viewportFraction: 0.92),
            itemCount: widget.todos.length,
            onPageChanged: (i) => setState(() => _page = i),
            itemBuilder: (_, i) => Padding(
              padding: const EdgeInsets.only(right: 10),
              child: _FocusTodoCard(todo: widget.todos[i]),
            ),
          ),
        ),
        if (widget.todos.length > 1) ...[
          const SizedBox(height: 10),
          Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: List.generate(widget.todos.length, (i) {
              final active = i == _page;
              return AnimatedContainer(
                duration: const Duration(milliseconds: 200),
                margin: const EdgeInsets.symmetric(horizontal: 3),
                width: active ? 16 : 6,
                height: 6,
                decoration: BoxDecoration(
                  color: active ? cs.primary : cs.outlineVariant,
                  borderRadius: BorderRadius.circular(3),
                ),
              );
            }),
          ),
        ],
      ],
    );
  }
}

class _FocusTodoCard extends StatelessWidget {
  final Todo todo;
  const _FocusTodoCard({required this.todo});

  Color _priorityColor() {
    switch (todo.priority) {
      case 'high':
        return Colors.red.shade400;
      case 'medium':
        return Colors.orange.shade400;
      default:
        return Colors.green.shade400;
    }
  }

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final theme = Theme.of(context);
    final priorityColor = _priorityColor();

    return Container(
      padding: const EdgeInsets.all(20),
      decoration: BoxDecoration(
        color: cs.surface,
        borderRadius: BorderRadius.circular(20),
        border: Border.all(color: cs.outlineVariant.withValues(alpha: 0.5)),
        boxShadow: [
          BoxShadow(
            color: cs.shadow.withValues(alpha: 0.04),
            blurRadius: 12,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
                decoration: BoxDecoration(
                  color: const Color(0xFF6366F1).withValues(alpha: 0.1),
                  borderRadius: BorderRadius.circular(20),
                ),
                child: Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    const HeroIcon(HeroIcons.checkCircle, size: 16, color: Color(0xFF6366F1)),
                    const SizedBox(width: 4),
                    Text(
                      'Todo',
                      style: theme.textTheme.labelSmall?.copyWith(
                        color: const Color(0xFF6366F1),
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                  ],
                ),
              ),
              const Spacer(),
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                decoration: BoxDecoration(
                  color: priorityColor.withValues(alpha: 0.12),
                  borderRadius: BorderRadius.circular(12),
                ),
                child: Text(
                  '${todo.priority[0].toUpperCase()}${todo.priority.substring(1)}',
                  style: theme.textTheme.labelSmall?.copyWith(
                    color: priorityColor,
                    fontWeight: FontWeight.w600,
                  ),
                ),
              ),
            ],
          ),
          const SizedBox(height: 12),
          Text(
            todo.title,
            style: theme.textTheme.titleSmall?.copyWith(
              fontWeight: FontWeight.w600,
              letterSpacing: -0.2,
            ),
            maxLines: 2,
            overflow: TextOverflow.ellipsis,
          ),
          const Spacer(),
          Text(
            'Swipe to see more',
            style: theme.textTheme.labelSmall?.copyWith(
              color: cs.onSurfaceVariant.withValues(alpha: 0.5),
            ),
          ),
        ],
      ),
    );
  }
}

// ─── Activity timeline ────────────────────────────────────────────────────────

class _ActivityTimeline extends StatelessWidget {
  final List<ActivityItem> items;

  const _ActivityTimeline({required this.items});

  @override
  Widget build(BuildContext context) {
    if (items.isEmpty) {
      return Text(
        'No recent activity',
        style: Theme.of(context).textTheme.bodySmall?.copyWith(
              color: Theme.of(context).colorScheme.onSurfaceVariant,
            ),
      );
    }

    return Column(
      children: List.generate(
        items.length,
        (i) => _TimelineTile(item: items[i], isLast: i == items.length - 1),
      ),
    );
  }
}

class _TimelineTile extends StatelessWidget {
  final ActivityItem item;
  final bool isLast;

  const _TimelineTile({required this.item, required this.isLast});

  HeroIcons get _icon {
    switch (item.type) {
      case ActivityType.note:
        return HeroIcons.documentText;
      case ActivityType.bookmark:
        return HeroIcons.bookmark;
      case ActivityType.reminder:
        return HeroIcons.clock;
      case ActivityType.todo:
        return HeroIcons.checkCircle;
    }
  }

  Color get _color {
    switch (item.type) {
      case ActivityType.note:
        return const Color(0xFF10B981);
      case ActivityType.bookmark:
        return const Color(0xFF8B5CF6);
      case ActivityType.reminder:
        return const Color(0xFFF59E0B);
      case ActivityType.todo:
        return const Color(0xFF6366F1);
    }
  }

  String _relativeTime(DateTime dt) {
    final diff = DateTime.now().difference(dt);
    if (diff.inMinutes < 1) return 'just now';
    if (diff.inMinutes < 60) return '${diff.inMinutes}m ago';
    if (diff.inHours < 24) return '${diff.inHours}h ago';
    if (diff.inDays < 7) return '${diff.inDays}d ago';
    return '${dt.year}-${dt.month.toString().padLeft(2, '0')}-${dt.day.toString().padLeft(2, '0')}';
  }

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final theme = Theme.of(context);

    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        SizedBox(
          width: 36,
          child: Column(
            children: [
              Container(
                width: 32,
                height: 32,
                decoration: BoxDecoration(
                  color: _color.withValues(alpha: 0.12),
                  shape: BoxShape.circle,
                ),
                child: Center(child: HeroIcon(_icon, size: 20, color: _color)),
              ),
              if (!isLast)
                Container(
                  width: 1,
                  height: 36,
                  color: cs.outlineVariant.withValues(alpha: 0.4),
                ),
            ],
          ),
        ),
        const SizedBox(width: 14),
        Expanded(
          child: Padding(
            padding: EdgeInsets.only(bottom: isLast ? 0 : 20),
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        item.title,
                        style: theme.textTheme.bodyMedium?.copyWith(fontWeight: FontWeight.w600),
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                      ),
                      const SizedBox(height: 2),
                      Text(
                        item.subtitle,
                        style: theme.textTheme.bodySmall?.copyWith(color: cs.onSurfaceVariant),
                      ),
                    ],
                  ),
                ),
                if (item.time != null)
                  Text(
                    _relativeTime(item.time!),
                    style: theme.textTheme.labelSmall?.copyWith(color: cs.onSurfaceVariant),
                  ),
              ],
            ),
          ),
        ),
      ],
    );
  }
}
