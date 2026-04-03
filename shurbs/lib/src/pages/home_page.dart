import 'package:flutter/material.dart';

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Scaffold(
      body: SafeArea(
        child: CustomScrollView(
          slivers: [
            const SliverAppBar.large(title: Text('Wild Almonds')),
            SliverPadding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              sliver: SliverList(
                delegate: SliverChildListDelegate([
                  _GreetingCard(colorScheme: colorScheme),
                  const SizedBox(height: 24),
                  Text('Overview', style: theme.textTheme.titleMedium?.copyWith(fontWeight: FontWeight.bold)),
                  const SizedBox(height: 12),
                  _StatsGrid(colorScheme: colorScheme),
                  const SizedBox(height: 24),
                  Text('Recent Activity', style: theme.textTheme.titleMedium?.copyWith(fontWeight: FontWeight.bold)),
                  const SizedBox(height: 12),
                  _RecentActivityList(),
                  const SizedBox(height: 32),
                ]),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _GreetingCard extends StatelessWidget {
  final ColorScheme colorScheme;

  const _GreetingCard({required this.colorScheme});

  String get _greeting {
    final hour = DateTime.now().hour;
    if (hour < 12) return 'Good morning';
    if (hour < 17) return 'Good afternoon';
    return 'Good evening';
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(20),
      decoration: BoxDecoration(
        color: colorScheme.primaryContainer,
        borderRadius: BorderRadius.circular(16),
      ),
      child: Row(
        children: [
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  _greeting,
                  style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                    color: colorScheme.onPrimaryContainer.withValues(alpha: 0.7),
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  'Welcome back',
                  style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                    color: colorScheme.onPrimaryContainer,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const SizedBox(height: 8),
                Text(
                  'You have 3 pending todos today.',
                  style: Theme.of(context).textTheme.bodySmall?.copyWith(
                    color: colorScheme.onPrimaryContainer.withValues(alpha: 0.8),
                  ),
                ),
              ],
            ),
          ),
          Icon(Icons.wb_sunny_outlined, size: 48, color: colorScheme.onPrimaryContainer.withValues(alpha: 0.5)),
        ],
      ),
    );
  }
}

class _StatsGrid extends StatelessWidget {
  final ColorScheme colorScheme;

  const _StatsGrid({required this.colorScheme});

  @override
  Widget build(BuildContext context) {
    final stats = [
      _StatItem(label: 'Todos', count: 3, icon: Icons.check_box_outlined, color: Colors.blue),
      _StatItem(label: 'Alarms', count: 1, icon: Icons.alarm_outlined, color: Colors.orange),
      _StatItem(label: 'Bookmarks', count: 12, icon: Icons.bookmark_outline, color: Colors.purple),
      _StatItem(label: 'Notes', count: 5, icon: Icons.note_outlined, color: Colors.green),
    ];

    return GridView.count(
      crossAxisCount: 2,
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      mainAxisSpacing: 12,
      crossAxisSpacing: 12,
      childAspectRatio: 1.6,
      children: stats.map((stat) => _StatCard(stat: stat)).toList(),
    );
  }
}

class _StatItem {
  final String label;
  final int count;
  final IconData icon;
  final Color color;

  const _StatItem({
    required this.label,
    required this.count,
    required this.icon,
    required this.color,
  });
}

class _StatCard extends StatelessWidget {
  final _StatItem stat;

  const _StatCard({required this.stat});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: stat.color.withValues(alpha: 0.1),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: stat.color.withValues(alpha: 0.2)),
      ),
      child: Row(
        children: [
          Icon(stat.icon, color: stat.color, size: 28),
          const SizedBox(width: 12),
          Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text(
                '${stat.count}',
                style: theme.textTheme.headlineSmall?.copyWith(
                  fontWeight: FontWeight.bold,
                  color: stat.color,
                ),
              ),
              Text(stat.label, style: theme.textTheme.bodySmall),
            ],
          ),
        ],
      ),
    );
  }
}

class _RecentActivityList extends StatelessWidget {
  const _RecentActivityList();

  @override
  Widget build(BuildContext context) {
    final items = [
      _ActivityItem(title: 'Buy groceries', type: 'Todo', time: '2h ago', icon: Icons.check_box_outlined),
      _ActivityItem(title: 'Morning standup', type: 'Alarm', time: '5h ago', icon: Icons.alarm),
      _ActivityItem(title: 'Flutter docs', type: 'Bookmark', time: 'Yesterday', icon: Icons.bookmark_outline),
    ];

    return Column(
      children: items
          .map(
            (item) => ListTile(
              contentPadding: EdgeInsets.zero,
              leading: CircleAvatar(
                backgroundColor: Theme.of(context).colorScheme.secondaryContainer,
                child: Icon(item.icon, size: 20, color: Theme.of(context).colorScheme.onSecondaryContainer),
              ),
              title: Text(item.title),
              subtitle: Text(item.type),
              trailing: Text(item.time, style: Theme.of(context).textTheme.bodySmall),
            ),
          )
          .toList(),
    );
  }
}

class _ActivityItem {
  final String title;
  final String type;
  final String time;
  final IconData icon;

  const _ActivityItem({
    required this.title,
    required this.type,
    required this.time,
    required this.icon,
  });
}
