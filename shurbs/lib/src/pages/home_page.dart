import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SafeArea(
        child: CustomScrollView(
          slivers: [
            SliverPadding(
              padding: const EdgeInsets.symmetric(horizontal: 20),
              sliver: SliverList(
                delegate: SliverChildListDelegate([
                  const SizedBox(height: 8),
                  const _HeroBanner(),
                  const SizedBox(height: 28),
                  const _SectionLabel(text: 'At a Glance'),
                  const SizedBox(height: 12),
                  const _StatsRow(),
                  const SizedBox(height: 28),
                  const _SectionLabel(text: "Today's Focus"),
                  const SizedBox(height: 12),
                  const _FocusCard(),
                  const SizedBox(height: 28),
                  const _SectionLabel(text: 'Recent Activity'),
                  const SizedBox(height: 12),
                  const _ActivityTimeline(),
                  const SizedBox(height: 40),
                ]),
              ),
            ),
          ],
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
        Container(width: 3, height: 14, decoration: BoxDecoration(color: cs.primary, borderRadius: BorderRadius.circular(2))),
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
  const _HeroBanner();

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
          // Decorative circles
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
          // Content
          Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                _date,
                style: Theme.of(context).textTheme.bodySmall?.copyWith(
                      color: isDark ? cs.onSurface.withValues(alpha: 0.5) : Colors.white.withValues(alpha: 0.75),
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
                      '3 tasks pending today',
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
  const _StatsRow();

  @override
  Widget build(BuildContext context) {
    final stats = [
      _Stat('Todos', '3', HeroIcons.checkCircle, const Color(0xFF6366F1)),
      _Stat('Alarms', '1', HeroIcons.clock, const Color(0xFFF59E0B)),
      _Stat('Bookmarks', '12', HeroIcons.bookmark, const Color(0xFF8B5CF6)),
      _Stat('Notes', '5', HeroIcons.documentText, const Color(0xFF10B981)),
    ];

    return Row(
      children: stats
          .map((s) => Expanded(child: Padding(
                padding: EdgeInsets.only(right: s == stats.last ? 0 : 10),
                child: _StatCard(stat: s),
              )))
          .toList(),
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
            style: Theme.of(context).textTheme.labelSmall?.copyWith(
                  color: cs.onSurfaceVariant,
                ),
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

// ─── Today's focus card ───────────────────────────────────────────────────────

class _FocusCard extends StatelessWidget {
  const _FocusCard();

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final theme = Theme.of(context);

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
                    const HeroIcon(HeroIcons.checkCircle, size: 20, color: Color(0xFF6366F1)),
                    const SizedBox(width: 4),
                    Text('Top todo', style: theme.textTheme.labelSmall?.copyWith(color: const Color(0xFF6366F1), fontWeight: FontWeight.w600)),
                  ],
                ),
              ),
              const Spacer(),
              Text('High priority', style: theme.textTheme.labelSmall?.copyWith(color: Colors.red.shade400)),
            ],
          ),
          const SizedBox(height: 14),
          Text(
            'Review pull request',
            style: theme.textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w600, letterSpacing: -0.3),
          ),
          const SizedBox(height: 6),
          Text(
            'Due today · Assigned to you',
            style: theme.textTheme.bodySmall?.copyWith(color: cs.onSurfaceVariant),
          ),
          const SizedBox(height: 16),
          Row(
            children: [
              Expanded(
                child: OutlinedButton(
                  onPressed: () {},
                  style: OutlinedButton.styleFrom(
                    padding: const EdgeInsets.symmetric(vertical: 10),
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                  ),
                  child: const Text('Dismiss'),
                ),
              ),
              const SizedBox(width: 10),
              Expanded(
                child: FilledButton(
                  onPressed: () {},
                  style: FilledButton.styleFrom(
                    padding: const EdgeInsets.symmetric(vertical: 10),
                    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                  ),
                  child: const Text('Mark Done'),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}

// ─── Activity timeline ────────────────────────────────────────────────────────

class _ActivityTimeline extends StatelessWidget {
  const _ActivityTimeline();

  @override
  Widget build(BuildContext context) {
    final items = [
      _Activity('Buy groceries', 'Todo completed', '2h ago', HeroIcons.checkCircle, const Color(0xFF10B981)),
      _Activity('Morning standup', 'Alarm triggered', '5h ago', HeroIcons.clock, const Color(0xFFF59E0B)),
      _Activity('Flutter docs', 'Bookmark added', 'Yesterday', HeroIcons.bookmark, const Color(0xFF8B5CF6)),
    ];

    return Column(
      children: List.generate(items.length, (i) => _TimelineTile(item: items[i], isLast: i == items.length - 1)),
    );
  }
}

class _Activity {
  final String title;
  final String subtitle;
  final String time;
  final HeroIcons icon;
  final Color color;
  const _Activity(this.title, this.subtitle, this.time, this.icon, this.color);
}

class _TimelineTile extends StatelessWidget {
  final _Activity item;
  final bool isLast;

  const _TimelineTile({required this.item, required this.isLast});

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final theme = Theme.of(context);

    return IntrinsicHeight(
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // Timeline line + dot
          SizedBox(
            width: 36,
            child: Column(
              children: [
                Container(
                  width: 32,
                  height: 32,
                  decoration: BoxDecoration(
                    color: item.color.withValues(alpha: 0.12),
                    shape: BoxShape.circle,
                  ),
                  child: Center(child: HeroIcon(item.icon, size: 20, color: item.color)),
                ),
                if (!isLast)
                  Expanded(
                    child: Center(
                      child: Container(
                        width: 1,
                        color: cs.outlineVariant.withValues(alpha: 0.4),
                      ),
                    ),
                  ),
              ],
            ),
          ),
          const SizedBox(width: 14),
          // Content
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
                        Text(item.title, style: theme.textTheme.bodyMedium?.copyWith(fontWeight: FontWeight.w600)),
                        const SizedBox(height: 2),
                        Text(item.subtitle, style: theme.textTheme.bodySmall?.copyWith(color: cs.onSurfaceVariant)),
                      ],
                    ),
                  ),
                  Text(item.time, style: theme.textTheme.labelSmall?.copyWith(color: cs.onSurfaceVariant)),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
