import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:heroicons/heroicons.dart';

import 'pages/home_page.dart';
import 'pages/todo_page.dart';
import 'pages/alarms_page.dart';
import 'pages/bookmarks_page.dart';
import 'pages/settings_page.dart';
import 'pages/notifications_page.dart';
import 'theme_notifier.dart';

class AppShell extends StatefulWidget {
  const AppShell({super.key});

  @override
  State<AppShell> createState() => _AppShellState();
}

class _AppShellState extends State<AppShell> {
  int _currentIndex = 0;
  final _searchController = SearchController();

  static const _searchableItems = [
    _SearchItem(title: 'Buy groceries', section: 'Todo', sectionIndex: 1, icon: HeroIcons.checkCircle),
    _SearchItem(title: 'Review pull request', section: 'Todo', sectionIndex: 1, icon: HeroIcons.checkCircle),
    _SearchItem(title: 'Read Flutter docs', section: 'Todo', sectionIndex: 1, icon: HeroIcons.checkCircle),
    _SearchItem(title: 'Morning standup', section: 'Alarms', sectionIndex: 2, icon: HeroIcons.clock),
    _SearchItem(title: 'Lunch break', section: 'Alarms', sectionIndex: 2, icon: HeroIcons.clock),
    _SearchItem(title: 'Evening workout', section: 'Alarms', sectionIndex: 2, icon: HeroIcons.clock),
    _SearchItem(title: 'Flutter Documentation', section: 'Bookmarks', sectionIndex: 3, icon: HeroIcons.bookmark),
    _SearchItem(title: 'Dart Language Tour', section: 'Bookmarks', sectionIndex: 3, icon: HeroIcons.bookmark),
    _SearchItem(title: 'Material Design 3', section: 'Bookmarks', sectionIndex: 3, icon: HeroIcons.bookmark),
    _SearchItem(title: 'pub.dev packages', section: 'Bookmarks', sectionIndex: 3, icon: HeroIcons.bookmark),
  ];

  final List<_NavItem> _navItems = const [
    _NavItem(icon: HeroIcons.home, label: 'Home'),
    _NavItem(icon: HeroIcons.checkCircle, label: 'Todo'),
    _NavItem(icon: HeroIcons.clock, label: 'Alarms'),
    _NavItem(icon: HeroIcons.bookmark, label: 'Bookmarks'),
    _NavItem(icon: HeroIcons.cog6Tooth, label: 'Settings'),
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
            leading: HeroIcon(item.icon),
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
    final colorScheme = Theme.of(context).colorScheme;
    final isDark = Theme.of(context).brightness == Brightness.dark;

    return AnnotatedRegion<SystemUiOverlayStyle>(
      value: SystemUiOverlayStyle(
        statusBarColor: Colors.transparent,
        statusBarIconBrightness: isDark ? Brightness.light : Brightness.dark,
        statusBarBrightness: isDark ? Brightness.dark : Brightness.light,
        systemNavigationBarColor: Colors.transparent,
      ),
      child: Scaffold(
        drawer: _AppDrawer(
          currentIndex: _currentIndex,
          onNavigate: (index) => setState(() => _currentIndex = index),
        ),
        body: Column(
          children: [
            SafeArea(
              bottom: false,
              child: Padding(
                padding: const EdgeInsets.fromLTRB(4, 8, 8, 8),
                child: Row(
                  children: [
                    Builder(
                      builder: (ctx) => IconButton(
                        icon: const HeroIcon(HeroIcons.bars3, size: 20),
                        onPressed: () => Scaffold.of(ctx).openDrawer(),
                      ),
                    ),
                    Expanded(
                      child: SearchAnchor.bar(
                        searchController: _searchController,
                        barHintText: 'Search todos, alarms, bookmarks…',
                        barLeading: const HeroIcon(HeroIcons.magnifyingGlass, size: 20),
                        suggestionsBuilder: _buildSuggestions,
                      ),
                    ),
                    IconButton(
                      icon: const HeroIcon(HeroIcons.bell, size: 20),
                      tooltip: 'Notifications',
                      onPressed: () => Navigator.push(
                        context,
                        MaterialPageRoute(builder: (_) => const NotificationsPage()),
                      ),
                    ),
                    Padding(
                      padding: const EdgeInsets.only(right: 4),
                      child: GestureDetector(
                        onTap: () => setState(() => _currentIndex = 4),
                        child: CircleAvatar(
                          radius: 18,
                          backgroundColor: colorScheme.primaryContainer,
                          child: Text(
                            'A',
                            style: TextStyle(
                              color: colorScheme.onPrimaryContainer,
                              fontWeight: FontWeight.bold,
                              fontSize: 14,
                            ),
                          ),
                        ),
                      ),
                    ),
                  ],
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
                  icon: HeroIcon(item.icon, style: HeroIconStyle.outline),
                  selectedIcon: HeroIcon(item.icon, style: HeroIconStyle.solid),
                  label: item.label,
                ),
              )
              .toList(),
        ),
      ),
    );
  }
}

// ── Drawer ────────────────────────────────────────────────────────────────────

class _AppDrawer extends StatelessWidget {
  final int currentIndex;
  final ValueChanged<int> onNavigate;

  const _AppDrawer({required this.currentIndex, required this.onNavigate});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    void go(int index) {
      Navigator.pop(context);
      onNavigate(index);
    }

    return Drawer(
      child: SafeArea(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            // ── User header ────────────────────────────────────────────────
            Padding(
              padding: const EdgeInsets.fromLTRB(20, 20, 20, 16),
              child: Row(
                children: [
                  CircleAvatar(
                    radius: 24,
                    backgroundColor: colorScheme.primary,
                    child: Text(
                      'A',
                      style: TextStyle(
                        color: colorScheme.onPrimary,
                        fontWeight: FontWeight.bold,
                        fontSize: 18,
                      ),
                    ),
                  ),
                  const SizedBox(width: 12),
                  Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        'Adeoye',
                        style: theme.textTheme.titleMedium?.copyWith(fontWeight: FontWeight.bold),
                      ),
                      Text('adeoye@example.com', style: theme.textTheme.bodySmall),
                    ],
                  ),
                ],
              ),
            ),

            // ── Workspaces ─────────────────────────────────────────────────
            Padding(
              padding: const EdgeInsets.fromLTRB(20, 0, 20, 6),
              child: Text(
                'WORKSPACES',
                style: theme.textTheme.labelSmall?.copyWith(
                  color: colorScheme.onSurfaceVariant,
                  letterSpacing: 1.1,
                  fontWeight: FontWeight.w600,
                ),
              ),
            ),
            ValueListenableBuilder<String>(
              valueListenable: activeWorkspaceNotifier,
              builder: (context, activeId, _) => Column(
                children: workspaces
                    .map(
                      (ws) => _DrawerItem(
                        icon: ws.icon,
                        label: ws.name,
                        selected: ws.id == activeId,
                        onTap: () => activeWorkspaceNotifier.value = ws.id,
                      ),
                    )
                    .toList(),
              ),
            ),

            const Padding(
              padding: EdgeInsets.symmetric(horizontal: 16, vertical: 8),
              child: Divider(height: 1),
            ),

            // ── Nav items ──────────────────────────────────────────────────
            _DrawerItem(icon: HeroIcons.home, label: 'Home', selected: currentIndex == 0, onTap: () => go(0)),
            _DrawerItem(icon: HeroIcons.checkCircle, label: 'Todos', selected: currentIndex == 1, onTap: () => go(1)),
            _DrawerItem(icon: HeroIcons.clock, label: 'Alarms', selected: currentIndex == 2, onTap: () => go(2)),
            _DrawerItem(icon: HeroIcons.bookmark, label: 'Bookmarks', selected: currentIndex == 3, onTap: () => go(3)),
            _DrawerItem(
              icon: HeroIcons.bell,
              label: 'Notifications',
              selected: false,
              onTap: () {
                Navigator.pop(context);
                Navigator.push(context, MaterialPageRoute(builder: (_) => const NotificationsPage()));
              },
            ),

            const Spacer(),
            const Padding(
              padding: EdgeInsets.symmetric(horizontal: 16),
              child: Divider(height: 1),
            ),

            // ── Color mode toggle ──────────────────────────────────────────
            ValueListenableBuilder<ThemeMode>(
              valueListenable: themeModeNotifier,
              builder: (context, mode, _) => Padding(
                padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
                child: Row(
                  children: [
                    HeroIcon(HeroIcons.swatch, size: 18, color: colorScheme.onSurfaceVariant),
                    const SizedBox(width: 10),
                    Text('Appearance', style: theme.textTheme.bodySmall),
                    const Spacer(),
                    _ThemeModeToggle(current: mode),
                  ],
                ),
              ),
            ),

            // ── Settings ───────────────────────────────────────────────────
            _DrawerItem(
              icon: HeroIcons.cog6Tooth,
              label: 'Settings',
              selected: currentIndex == 4,
              onTap: () => go(4),
            ),
            const SizedBox(height: 8),
          ],
        ),
      ),
    );
  }
}

// ── Theme mode toggle (sun / auto / moon) ─────────────────────────────────────

class _ThemeModeToggle extends StatelessWidget {
  final ThemeMode current;
  const _ThemeModeToggle({required this.current});

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;

    Widget btn(HeroIcons icon, ThemeMode mode) {
      final active = current == mode;
      return GestureDetector(
        onTap: () => themeModeNotifier.value = mode,
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 180),
          curve: Curves.easeInOut,
          width: 32,
          height: 28,
          decoration: BoxDecoration(
            color: active ? colorScheme.primary.withValues(alpha: 0.15) : Colors.transparent,
            borderRadius: BorderRadius.circular(7),
            border: Border.all(
              color: active ? colorScheme.primary : Colors.transparent,
              width: 1,
            ),
          ),
          child: Center(
            child: HeroIcon(
              icon,
              size: 15,
              color: active ? colorScheme.primary : colorScheme.onSurfaceVariant,
            ),
          ),
        ),
      );
    }

    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        btn(HeroIcons.sun, ThemeMode.light),
        const SizedBox(width: 4),
        btn(HeroIcons.computerDesktop, ThemeMode.system),
        const SizedBox(width: 4),
        btn(HeroIcons.moon, ThemeMode.dark),
      ],
    );
  }
}

// ── Drawer item ───────────────────────────────────────────────────────────────

class _DrawerItem extends StatelessWidget {
  final HeroIcons icon;
  final String label;
  final bool selected;
  final VoidCallback onTap;

  const _DrawerItem({required this.icon, required this.label, required this.selected, required this.onTap});

  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: HeroIcon(icon, size: 20),
      title: Text(label),
      onTap: onTap,
      selected: selected,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
      horizontalTitleGap: 8,
      contentPadding: const EdgeInsets.symmetric(horizontal: 16),
    );
  }
}

// ── Models ────────────────────────────────────────────────────────────────────

class _SearchItem {
  final String title;
  final String section;
  final int sectionIndex;
  final HeroIcons icon;

  const _SearchItem({
    required this.title,
    required this.section,
    required this.sectionIndex,
    required this.icon,
  });
}

class _NavItem {
  final HeroIcons icon;
  final String label;

  const _NavItem({required this.icon, required this.label});
}
