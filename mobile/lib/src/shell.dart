import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:heroicons/heroicons.dart';

import 'controllers/bookmark_controller.dart';
import 'controllers/calendar_controller.dart';
import 'controllers/home_controller.dart';
import 'controllers/note_controller.dart';
import 'controllers/reminder_controller.dart';
import 'controllers/todo_controller.dart';
import 'controllers/workspace_controller.dart';
import 'pages/home_page.dart';
import 'pages/todo_page.dart';
import 'pages/reminders_page.dart';
import 'pages/calendar_page.dart';
import 'pages/bookmarks_page.dart';
import 'pages/notes_page.dart';
import 'pages/settings_page.dart';
import 'pages/notifications_page.dart';
import 'theme_notifier.dart';
import 'profile_notifier.dart';

class AppShell extends StatefulWidget {
  const AppShell({super.key});

  @override
  State<AppShell> createState() => _AppShellState();
}

class _AppShellState extends State<AppShell> {
  int _currentIndex = 0;
  final _searchController = SearchController();

  late final WorkspaceController _workspaceController;
  late final TodoController _todoController;
  late final NoteController _noteController;
  late final BookmarkController _bookmarkController;
  late final ReminderController _reminderController;
  late final HomeController _homeController;
  late final CalendarController _calendarController;

  @override
  void initState() {
    super.initState();
    _workspaceController = WorkspaceController();
    _todoController = TodoController();
    _noteController = NoteController();
    _bookmarkController = BookmarkController();
    _reminderController = ReminderController();
    _homeController = HomeController(
      todoController: _todoController,
      noteController: _noteController,
      bookmarkController: _bookmarkController,
      reminderController: _reminderController,
    );
    _calendarController = CalendarController(
      todoController: _todoController,
      reminderController: _reminderController,
    );

    _loadAll();
  }

  Future<void> _loadAll() async {
    await _workspaceController.load();
    final wsId = _workspaceController.activeWorkspaceId;
    if (wsId == null) return;
    await Future.wait([
      ProfileNotifier.instance.load(wsId),
      _todoController.load(wsId),
      _noteController.load(wsId),
      _bookmarkController.load(wsId),
      _reminderController.load(wsId),
    ]);
  }

  @override
  void dispose() {
    _searchController.dispose();
    _homeController.dispose();
    _calendarController.dispose();
    _workspaceController.dispose();
    _todoController.dispose();
    _noteController.dispose();
    _bookmarkController.dispose();
    _reminderController.dispose();
    super.dispose();
  }

  // Nav indices: 0=Home 1=Todo 2=Reminders 3=Bookmarks 4=Settings
  final List<_NavItem> _navItems = const [
    _NavItem(icon: HeroIcons.home, label: 'Home'),
    _NavItem(icon: HeroIcons.checkCircle, label: 'Todo'),
    _NavItem(icon: HeroIcons.clock, label: 'Reminders'),
    _NavItem(icon: HeroIcons.bookmark, label: 'Bookmarks'),
    _NavItem(icon: HeroIcons.cog6Tooth, label: 'Settings'),
  ];

  Iterable<Widget> _buildSuggestions(BuildContext context, SearchController controller) {
    final query = controller.text.toLowerCase();
    if (query.isEmpty) return [];

    final results = <_SearchResult>[];

    for (final todo in _todoController.todos) {
      if (todo.title.toLowerCase().contains(query)) {
        results.add(_SearchResult(title: todo.title, section: 'Todo', sectionIndex: 1, icon: HeroIcons.checkCircle));
      }
    }
    for (final note in _noteController.notes) {
      if (note.title.toLowerCase().contains(query)) {
        results.add(_SearchResult(title: note.title.isEmpty ? 'Untitled' : note.title, section: 'Notes', sectionIndex: -1, icon: HeroIcons.documentText));
      }
    }
    for (final bookmark in _bookmarkController.bookmarks) {
      if (bookmark.title.toLowerCase().contains(query)) {
        results.add(_SearchResult(title: bookmark.title, section: 'Bookmarks', sectionIndex: 3, icon: HeroIcons.bookmark));
      }
    }
    for (final reminder in _reminderController.reminders) {
      if (reminder.title.toLowerCase().contains(query)) {
        results.add(_SearchResult(title: reminder.title, section: 'Reminders', sectionIndex: 2, icon: HeroIcons.clock));
      }
    }

    return results.take(10).map(
          (item) => ListTile(
            leading: HeroIcon(item.icon),
            title: Text(item.title),
            subtitle: Text(item.section),
            onTap: () {
              controller.closeView(item.title);
              if (item.sectionIndex >= 0) {
                setState(() => _currentIndex = item.sectionIndex);
              }
            },
          ),
        );
  }

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    final isDark = Theme.of(context).brightness == Brightness.dark;

    final pages = [
      HomePage(controller: _homeController),
      TodoPage(controller: _todoController),
      RemindersPage(controller: _reminderController),
      BookmarksPage(controller: _bookmarkController),
      const SettingsPage(),
    ];

    return AnnotatedRegion<SystemUiOverlayStyle>(
      value: SystemUiOverlayStyle(
        statusBarColor: Colors.transparent,
        statusBarIconBrightness: isDark ? Brightness.light : Brightness.dark,
        statusBarBrightness: isDark ? Brightness.dark : Brightness.light,
        systemNavigationBarColor: Colors.transparent,
      ),
      child: NotificationListener<NavigateToTabNotification>(
        onNotification: (n) {
          setState(() => _currentIndex = n.tabIndex);
          return true;
        },
        child: Scaffold(
        drawer: _AppDrawer(
          currentIndex: _currentIndex,
          onNavigate: (index) => setState(() => _currentIndex = index),
          noteController: _noteController,
          calendarController: _calendarController,
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
                        barHintText: 'Search todos, reminders, bookmarks…',
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
                        child: ListenableBuilder(
                          listenable: ProfileNotifier.instance,
                          builder: (_, __) => CircleAvatar(
                            radius: 18,
                            backgroundColor: colorScheme.primaryContainer,
                            child: Text(
                              ProfileNotifier.instance.initials,
                              style: TextStyle(
                                color: colorScheme.onPrimaryContainer,
                                fontWeight: FontWeight.bold,
                                fontSize: 14,
                              ),
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
                  children: pages,
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
      ),
    );
  }
}

// ── Drawer ────────────────────────────────────────────────────────────────────

class _AppDrawer extends StatelessWidget {
  final int currentIndex;
  final ValueChanged<int> onNavigate;
  final NoteController noteController;
  final CalendarController calendarController;

  const _AppDrawer({
    required this.currentIndex,
    required this.onNavigate,
    required this.noteController,
    required this.calendarController,
  });

  @override
  Widget build(BuildContext context) {
    void go(int index) {
      Navigator.pop(context);
      onNavigate(index);
    }

    return Drawer(
      child: Column(
        children: [
          _DrawerHeader(),
          Expanded(
            child: ListView(
              padding: const EdgeInsets.fromLTRB(12, 12, 12, 0),
              children: [
                _NavTile(icon: HeroIcons.home, label: 'Home', selected: currentIndex == 0, onTap: () => go(0)),
                _NavTile(icon: HeroIcons.checkCircle, label: 'Todos', selected: currentIndex == 1, onTap: () => go(1)),
                _NavTile(icon: HeroIcons.clock, label: 'Reminders', selected: currentIndex == 2, onTap: () => go(2)),
                _NavTile(icon: HeroIcons.bookmark, label: 'Bookmarks', selected: currentIndex == 3, onTap: () => go(3)),
                // _NavTile(
                //   icon: HeroIcons.calendarDays,
                //   label: 'Calendar',
                //   selected: false,
                //   muted: true,
                //   onTap: () {},
                // ),
                _NavTile(
                  icon: HeroIcons.documentText,
                  label: 'Notes',
                  selected: false,
                  onTap: () {
                    Navigator.pop(context);
                    Navigator.push(
                      context,
                      MaterialPageRoute(
                        builder: (_) => NotesPage(controller: noteController),
                      ),
                    );
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
                _WorkspacesSection(),
              ],
            ),
          ),
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
        return ListenableBuilder(
          listenable: ProfileNotifier.instance,
          builder: (_, __) {
            final profile = ProfileNotifier.instance;
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
              Positioned.fill(child: CustomPaint(painter: _DotPainter())),
              SafeArea(
                bottom: false,
                child: Padding(
                  padding: const EdgeInsets.fromLTRB(20, 24, 20, 24),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
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
                              child: Text(
                                profile.initials,
                                style: const TextStyle(color: Colors.white, fontWeight: FontWeight.bold, fontSize: 18),
                              ),
                            ),
                          ),
                          const SizedBox(width: 12),
                          Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                profile.displayName,
                                style: const TextStyle(color: Colors.white, fontWeight: FontWeight.w700, fontSize: 16),
                              ),
                              Text(
                                profile.email.isNotEmpty ? profile.email : 'Set up your profile',
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
  final bool muted;

  const _NavTile({required this.icon, required this.label, required this.selected, required this.onTap, this.muted = false});

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    final theme = Theme.of(context);

    return ValueListenableBuilder<AccentSwatch>(
      valueListenable: accentColorNotifier,
      builder: (_, accent, __) {
        return Opacity(
          opacity: muted ? 0.4 : 1.0,
          child: Padding(
          padding: const EdgeInsets.only(bottom: 2),
          child: Material(
            color: selected ? accent.primary.withValues(alpha: 0.1) : Colors.transparent,
            borderRadius: BorderRadius.circular(10),
            child: InkWell(
              onTap: muted ? null : onTap,
              borderRadius: BorderRadius.circular(10),
              child: Padding(
                padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 9),
                child: Row(
                  children: [
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
        ));
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


// ── Models ────────────────────────────────────────────────────────────────────

class _SearchResult {
  final String title;
  final String section;
  final int sectionIndex;
  final HeroIcons icon;

  const _SearchResult({
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
