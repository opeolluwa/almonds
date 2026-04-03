import 'package:flutter/material.dart';

import 'pages/home_page.dart';
import 'pages/todo_page.dart';
import 'pages/alarms_page.dart';
import 'pages/bookmarks_page.dart';
import 'pages/settings_page.dart';

class AppShell extends StatefulWidget {
  const AppShell({super.key});

  @override
  State<AppShell> createState() => _AppShellState();
}

class _AppShellState extends State<AppShell> {
  int _currentIndex = 0;
  final _searchController = SearchController();

  static const _searchableItems = [
    _SearchItem(title: 'Buy groceries', section: 'Todo', sectionIndex: 1, icon: Icons.check_box_outlined),
    _SearchItem(title: 'Review pull request', section: 'Todo', sectionIndex: 1, icon: Icons.check_box_outlined),
    _SearchItem(title: 'Read Flutter docs', section: 'Todo', sectionIndex: 1, icon: Icons.check_box_outlined),
    _SearchItem(title: 'Morning standup', section: 'Alarms', sectionIndex: 2, icon: Icons.alarm_outlined),
    _SearchItem(title: 'Lunch break', section: 'Alarms', sectionIndex: 2, icon: Icons.alarm_outlined),
    _SearchItem(title: 'Evening workout', section: 'Alarms', sectionIndex: 2, icon: Icons.alarm_outlined),
    _SearchItem(title: 'Flutter Documentation', section: 'Bookmarks', sectionIndex: 3, icon: Icons.bookmark_outline),
    _SearchItem(title: 'Dart Language Tour', section: 'Bookmarks', sectionIndex: 3, icon: Icons.bookmark_outline),
    _SearchItem(title: 'Material Design 3', section: 'Bookmarks', sectionIndex: 3, icon: Icons.bookmark_outline),
    _SearchItem(title: 'pub.dev packages', section: 'Bookmarks', sectionIndex: 3, icon: Icons.bookmark_outline),
  ];

  final List<_NavItem> _navItems = const [
    _NavItem(icon: Icons.home_outlined, activeIcon: Icons.home, label: 'Home'),
    _NavItem(icon: Icons.check_box_outlined, activeIcon: Icons.check_box, label: 'Todo'),
    _NavItem(icon: Icons.alarm_outlined, activeIcon: Icons.alarm, label: 'Alarms'),
    _NavItem(icon: Icons.bookmark_outline, activeIcon: Icons.bookmark, label: 'Bookmarks'),
    _NavItem(icon: Icons.settings_outlined, activeIcon: Icons.settings, label: 'Settings'),
  ];

  final List<Widget> _pages = const [
    HomePage(),
    TodoPage(),
    AlarmsPage(),
    BookmarksPage(),
    SettingsPage(),
  ];

  @override
  void dispose() {
    _searchController.dispose();
    super.dispose();
  }

  Iterable<Widget> _buildSuggestions(BuildContext context, SearchController controller) {
    final query = controller.text.toLowerCase();
    if (query.isEmpty) return [];

    return _searchableItems
        .where((item) => item.title.toLowerCase().contains(query))
        .map(
          (item) => ListTile(
            leading: Icon(item.icon),
            title: Text(item.title),
            subtitle: Text(item.section),
            onTap: () {
              controller.closeView(item.title);
              setState(() => _currentIndex = item.sectionIndex);
            },
          ),
        );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Column(
        children: [
          SafeArea(
            bottom: false,
            child: Padding(
              padding: const EdgeInsets.fromLTRB(16, 8, 16, 8),
              child: SearchAnchor.bar(
                searchController: _searchController,
                barHintText: 'Search todos, alarms, bookmarks…',
                barLeading: const Icon(Icons.search, size: 20),
                suggestionsBuilder: _buildSuggestions,
              ),
            ),
          ),
          Expanded(
            child: MediaQuery(
              data: MediaQuery.of(context).copyWith(
                padding: MediaQuery.of(context).padding.copyWith(top: 0),
              ),
              child: IndexedStack(
                index: _currentIndex,
                children: _pages,
              ),
            ),
          ),
        ],
      ),
      bottomNavigationBar: NavigationBar(
        selectedIndex: _currentIndex,
        onDestinationSelected: (index) => setState(() => _currentIndex = index),
        destinations: _navItems
            .map(
              (item) => NavigationDestination(
                icon: Icon(item.icon),
                selectedIcon: Icon(item.activeIcon),
                label: item.label,
              ),
            )
            .toList(),
      ),
    );
  }
}

class _SearchItem {
  final String title;
  final String section;
  final int sectionIndex;
  final IconData icon;

  const _SearchItem({
    required this.title,
    required this.section,
    required this.sectionIndex,
    required this.icon,
  });
}

class _NavItem {
  final IconData icon;
  final IconData activeIcon;
  final String label;

  const _NavItem({
    required this.icon,
    required this.activeIcon,
    required this.label,
  });
}
