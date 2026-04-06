import 'package:flutter/material.dart';

import 'shell.dart';
import 'theme.dart';
import 'theme_notifier.dart';

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) {
    return ValueListenableBuilder<ThemeMode>(
      valueListenable: themeModeNotifier,
      builder: (context, mode, _) => ValueListenableBuilder<AccentSwatch>(
        valueListenable: accentColorNotifier,
        builder: (context, accent, _) => MaterialApp(
          debugShowCheckedModeBanner: false,
          themeMode: mode,
          theme: lightTheme(accent),
          darkTheme: darkTheme(accent),
          home: const AppShell(),
        ),
      ),
    );
  }
}
