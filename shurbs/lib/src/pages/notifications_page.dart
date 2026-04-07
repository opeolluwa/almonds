import 'package:flutter/material.dart';
import 'package:heroicons/heroicons.dart';

class NotificationsPage extends StatelessWidget {
  const NotificationsPage({super.key});

  static const _notifications = [
    _NotifItem(
      icon: HeroIcons.checkCircle,
      title: 'Todo completed',
      body: 'You completed "Buy groceries"',
      time: '2h ago',
      color: Colors.green,
    ),
    _NotifItem(
      icon: HeroIcons.clock,
      title: 'Reminder triggered',
      body: 'Morning standup reminder went off',
      time: '5h ago',
      color: Colors.orange,
    ),
    _NotifItem(
      icon: HeroIcons.bookmark,
      title: 'New bookmark',
      body: 'Flutter docs added to bookmarks',
      time: 'Yesterday',
      color: Colors.purple,
    ),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Notifications'),
        centerTitle: false,
      ),
      body: _notifications.isEmpty
          ? Center(
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  HeroIcon(
                    HeroIcons.bellSlash,
                    size: 64,
                    color: Theme.of(context).colorScheme.outlineVariant,
                  ),
                  const SizedBox(height: 12),
                  Text(
                    'No notifications',
                    style: Theme.of(context).textTheme.bodyLarge,
                  ),
                ],
              ),
            )
          : ListView.separated(
              padding: const EdgeInsets.symmetric(vertical: 8),
              itemCount: _notifications.length,
              separatorBuilder: (_, __) => const Divider(height: 1),
              itemBuilder: (ctx, i) => _NotifTile(item: _notifications[i]),
            ),
    );
  }
}

class _NotifItem {
  final HeroIcons icon;
  final String title;
  final String body;
  final String time;
  final Color color;

  const _NotifItem({
    required this.icon,
    required this.title,
    required this.body,
    required this.time,
    required this.color,
  });
}

class _NotifTile extends StatelessWidget {
  final _NotifItem item;

  const _NotifTile({required this.item});

  @override
  Widget build(BuildContext context) {
    return ListTile(
      contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 4),
      leading: CircleAvatar(
        backgroundColor: item.color.withValues(alpha: 0.12),
        child: HeroIcon(item.icon, size: 20, color: item.color),
      ),
      title: Text(item.title, style: const TextStyle(fontWeight: FontWeight.w600)),
      subtitle: Text(item.body),
      trailing: Text(
        item.time,
        style: Theme.of(context).textTheme.bodySmall,
      ),
    );
  }
}
