import 'package:flutter/material.dart';

import 'shell.dart';
import 'theme.dart';
import 'theme_notifier.dart';
import 'pages/onboarding_page.dart';

class App extends StatelessWidget {
  final bool showOnboarding;
  const App({super.key, this.showOnboarding = false});

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
          home: showOnboarding ? const OnboardingPage() : const AppShell(),
        ),
      ),
    );
  }
}
