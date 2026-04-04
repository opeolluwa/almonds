import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:heroicons/heroicons.dart';

import 'pages/home_page.dart';
import 'pages/todo_page.dart';
import 'pages/alarms_page.dart';
import 'pages/bookmarks_page.dart';
import 'pages/notes_page.dart';
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
    void go(int index) {
      Navigator.pop(context);
      onNavigate(index);
    }

    return Drawer(
      child: Column(
        children: [
          // ── Gradient header ────────────────────────────────────────────
          _DrawerHeader(),

          // ── Scrollable body ────────────────────────────────────────────
          Expanded(
            child: ListView(
              padding: const EdgeInsets.fromLTRB(12, 12, 12, 0),
              children: [
                _NavTile(icon: HeroIcons.home, label: 'Home', selected: currentIndex == 0, onTap: () => go(0)),
                _NavTile(icon: HeroIcons.checkCircle, label: 'Todos', selected: currentIndex == 1, onTap: () => go(1)),
                _NavTile(icon: HeroIcons.clock, label: 'Alarms', selected: currentIndex == 2, onTap: () => go(2)),
                _NavTile(icon: HeroIcons.bookmark, label: 'Bookmarks', selected: currentIndex == 3, onTap: () => go(3)),
                _NavTile(
                  icon: HeroIcons.documentText,
                  label: 'Notes',
                  selected: false,
                  onTap: () {
                    Navigator.pop(context);
                    Navigator.push(context, MaterialPageRoute(builder: (_) => const NotesPage()));
                  },
                ),
                _NavTile(
                  icon: HeroIcons.bell,
                  label: 'Notifications',
                  selected: false,
                  onTap: () {
                    Navigator.pop(context);
                    Navigator.push(context, MaterialPageRoute(builder: (_) => const NotificationsPage()));
                  },
                ),

                const SizedBox(height: 20),

                // ── Workspaces ─────────────────────────────────────────
                _WorkspacesSection(),
              ],
            ),
          ),

          // ── Footer ────────────────────────────────────────────────────
          _DrawerFooter(currentIndex: currentIndex, onNavigate: onNavigate),
        ],
      ),
    );
  }
}

// ── Drawer header ─────────────────────────────────────────────────────────────

class _DrawerHeader extends StatelessWidget {
  const _DrawerHeader();

  @override
  Widget build(BuildContext context) {
    return ValueListenableBuilder<AccentSwatch>(
      valueListenable: accentColorNotifier,
      builder: (_, accent, __) {
        return Container(
          width: double.infinity,
          decoration: BoxDecoration(
            gradient: LinearGradient(
              begin: Alignment.topLeft,
              end: Alignment.bottomRight,
              colors: [accent.primary, accent.primaryContainer],
            ),
          ),
          child: Stack(
            children: [
              // Dot pattern
              Positioned.fill(child: CustomPaint(painter: _DotPainter())),
              // Content
              SafeArea(
                bottom: false,
                child: Padding(
                  padding: const EdgeInsets.fromLTRB(20, 24, 20, 24),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      // App mark
                      Row(
                        children: [
                          Container(
                            width: 28,
                            height: 28,
                            decoration: BoxDecoration(
                              color: Colors.white.withValues(alpha: 0.2),
                              borderRadius: BorderRadius.circular(7),
                            ),
                            child: const Center(child: HeroIcon(HeroIcons.sparkles, size: 15, color: Colors.white)),
                          ),
                          const SizedBox(width: 8),
                          const Text(
                            'Shurbs',
                            style: TextStyle(
                              color: Colors.white,
                              fontWeight: FontWeight.w700,
                              fontSize: 14,
                              letterSpacing: 0.4,
                            ),
                          ),
                        ],
                      ),
                      const SizedBox(height: 20),
                      // Avatar + user
                      Row(
                        children: [
                          Container(
                            padding: const EdgeInsets.all(2.5),
                            decoration: BoxDecoration(
                              shape: BoxShape.circle,
                              color: Colors.white.withValues(alpha: 0.25),
                            ),
                            child: CircleAvatar(
                              radius: 22,
                              backgroundColor: accent.primaryContainer,
                              child: const Text(
                                'A',
                                style: TextStyle(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 18),
                              ),
                            ),
                          ),
                          const SizedBox(width: 12),
                          Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              const Text(
                                'Adeoye',
                                style: TextStyle(color: Colors.white, fontWeight: FontWeight.w700, fontSize: 16),
                              ),
                              Text(
                                'adeoye@example.com',
                                style: TextStyle(color: Colors.white.withValues(alpha: 0.72), fontSize: 12),
                              ),
                            ],
                          ),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}

class _DotPainter extends CustomPainter {
  @override
  void paint(Canvas canvas, Size size) {
    final p = Paint()..color = Colors.white.withValues(alpha: 0.07);
    const s = 20.0;
    for (double r = 0; r * s < size.height + s; r++) {
      final ox = (r % 2 == 0) ? 0.0 : s / 2;
      for (double c = 0; c * s - ox < size.width + s; c++) {
        canvas.drawCircle(Offset(c * s - ox + s / 2, r * s + s / 2), 2.0, p);
      }
    }
  }

  @override
  bool shouldRepaint(_DotPainter _) => false;
}

// ── Nav tile ──────────────────────────────────────────────────────────────────

class _NavTile extends StatelessWidget {
  final HeroIcons icon;
  final String label;
  final bool selected;
  final VoidCallback onTap;

  const _NavTile({required this.icon, required this.label, required this.selected, required this.onTap});

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    final theme = Theme.of(context);

    return ValueListenableBuilder<AccentSwatch>(
      valueListenable: accentColorNotifier,
      builder: (_, accent, __) {
        return Padding(
          padding: const EdgeInsets.only(bottom: 2),
          child: Material(
            color: selected ? accent.primary.withValues(alpha: 0.1) : Colors.transparent,
            borderRadius: BorderRadius.circular(10),
            child: InkWell(
              onTap: onTap,
              borderRadius: BorderRadius.circular(10),
              child: Padding(
                padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 9),
                child: Row(
                  children: [
                    // Icon badge
                    AnimatedContainer(
                      duration: const Duration(milliseconds: 200),
                      width: 34,
                      height: 34,
                      decoration: BoxDecoration(
                        color: selected ? accent.primary : colorScheme.surfaceContainerHighest,
                        borderRadius: BorderRadius.circular(9),
                      ),
                      child: Center(
                        child: HeroIcon(
                          icon,
                          size: 17,
                          style: selected ? HeroIconStyle.solid : HeroIconStyle.outline,
                          color: selected ? Colors.white : colorScheme.onSurfaceVariant,
                        ),
                      ),
                    ),
                    const SizedBox(width: 12),
                    Text(
                      label,
                      style: theme.textTheme.bodyMedium?.copyWith(
                        fontWeight: selected ? FontWeight.w700 : FontWeight.w500,
                        color: selected ? accent.primary : colorScheme.onSurface,
                      ),
                    ),
                    if (selected) ...[
                      const Spacer(),
                      Container(
                        width: 5,
                        height: 5,
                        decoration: BoxDecoration(color: accent.primary, shape: BoxShape.circle),
                      ),
                      const SizedBox(width: 2),
                    ],
                  ],
                ),
              ),
            ),
          ),
        );
      },
    );
  }
}

// ── Workspaces section ────────────────────────────────────────────────────────

class _WorkspacesSection extends StatelessWidget {
  const _WorkspacesSection();

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Padding(
          padding: const EdgeInsets.only(left: 10, bottom: 8),
          child: Text(
            'WORKSPACES',
            style: theme.textTheme.labelSmall?.copyWith(
              color: colorScheme.onSurfaceVariant.withValues(alpha: 0.5),
              letterSpacing: 1.2,
              fontSize: 10,
              fontWeight: FontWeight.w600,
            ),
          ),
        ),
        ValueListenableBuilder<AccentSwatch>(
          valueListenable: accentColorNotifier,
          builder: (_, accent, __) => ValueListenableBuilder<String>(
            valueListenable: activeWorkspaceNotifier,
            builder: (_, activeId, __) => SingleChildScrollView(
              scrollDirection: Axis.horizontal,
              child: Row(
                children: [
                  ...workspaces.map((ws) {
                    final active = ws.id == activeId;
                    return Padding(
                      padding: const EdgeInsets.only(right: 8),
                      child: GestureDetector(
                        onTap: () => activeWorkspaceNotifier.value = ws.id,
                        child: AnimatedContainer(
                          duration: const Duration(milliseconds: 180),
                          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 7),
                          decoration: BoxDecoration(
                            color: active ? accent.primary : colorScheme.surfaceContainerHighest,
                            borderRadius: BorderRadius.circular(20),
                          ),
                          child: Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              HeroIcon(
                                ws.icon,
                                size: 13,
                                style: active ? HeroIconStyle.solid : HeroIconStyle.outline,
                                color: active ? Colors.white : colorScheme.onSurfaceVariant,
                              ),
                              const SizedBox(width: 6),
                              Text(
                                ws.name,
                                style: theme.textTheme.labelSmall?.copyWith(
                                  color: active ? Colors.white : colorScheme.onSurfaceVariant,
                                  fontWeight: active ? FontWeight.w700 : FontWeight.w500,
                                ),
                              ),
                            ],
                          ),
                        ),
                      ),
                    );
                  }),
                  // New workspace chip
                  GestureDetector(
                    onTap: () {},
                    child: Container(
                      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 7),
                      decoration: BoxDecoration(
                        border: Border.all(
                          color: colorScheme.outlineVariant.withValues(alpha: 0.6),
                          width: 1,
                        ),
                        borderRadius: BorderRadius.circular(20),
                      ),
                      child: Row(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          HeroIcon(HeroIcons.plus, size: 12, color: colorScheme.onSurfaceVariant),
                          const SizedBox(width: 5),
                          Text(
                            'New',
                            style: theme.textTheme.labelSmall?.copyWith(color: colorScheme.onSurfaceVariant),
                          ),
                        ],
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ],
    );
  }
}

// ── Drawer footer ─────────────────────────────────────────────────────────────

class _DrawerFooter extends StatelessWidget {
  final int currentIndex;
  final ValueChanged<int> onNavigate;

  const _DrawerFooter({required this.currentIndex, required this.onNavigate});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Container(
      decoration: BoxDecoration(
        border: Border(top: BorderSide(color: colorScheme.outlineVariant.withValues(alpha: 0.4))),
      ),
      padding: const EdgeInsets.fromLTRB(16, 10, 12, 0),
      child: SafeArea(
        top: false,
        child: Row(
          children: [
            // Settings
            Expanded(
              child: InkWell(
                onTap: () {
                  Navigator.pop(context);
                  onNavigate(4);
                },
                borderRadius: BorderRadius.circular(8),
                child: Padding(
                  padding: const EdgeInsets.symmetric(vertical: 8),
                  child: Row(
                    children: [
                      HeroIcon(
                        HeroIcons.cog6Tooth,
                        size: 18,
                        style: currentIndex == 4 ? HeroIconStyle.solid : HeroIconStyle.outline,
                        color: colorScheme.onSurfaceVariant,
                      ),
                      const SizedBox(width: 8),
                      Text(
                        'Settings',
                        style: theme.textTheme.bodySmall?.copyWith(
                          color: colorScheme.onSurfaceVariant,
                          fontWeight: currentIndex == 4 ? FontWeight.w600 : FontWeight.normal,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ),
            // Theme toggle
            ValueListenableBuilder<ThemeMode>(
              valueListenable: themeModeNotifier,
              builder: (_, mode, __) => _ThemeModeToggle(current: mode),
            ),
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
