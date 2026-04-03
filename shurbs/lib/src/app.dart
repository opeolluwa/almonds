import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:google_fonts/google_fonts.dart';

import 'shell.dart';
import 'theme_notifier.dart';

// Onyx dark palette
const _onyxBackground = Color(0xFF0d1220);
const _onyxSurface = Color(0xFF0a0f1c);
const _onyxSurfaceVariant = Color(0xFF070b14);
const _onyxOutline = Color(0xFF5e7aab);

// Rose accent palette
const _rosePrimary = Color(0xFFd02752);
const _rosePrimaryContainer = Color(0xFFa11d3f);
const _roseOnPrimaryContainer = Color(0xFFf9d0d9);
const _roseSecondary = Color(0xFFed7694);

class ShurbsApp extends StatelessWidget {
  const ShurbsApp({super.key});

  @override
  Widget build(BuildContext context) {
    return AnnotatedRegion<SystemUiOverlayStyle>(
      value: SystemUiOverlayStyle.light.copyWith(
        statusBarColor: Colors.transparent,
        systemNavigationBarColor: Colors.transparent,
      ),
      child: ValueListenableBuilder<ThemeMode>(
        valueListenable: themeModeNotifier,
        builder: (context, mode, child) => MaterialApp(
          title: 'Wild Almonds',
          debugShowCheckedModeBanner: false,
          themeMode: mode,
          theme: _lightTheme(),
          darkTheme: _darkTheme(),
          builder: (context, child) => MediaQuery(
            data: MediaQuery.of(context).copyWith(
              textScaler: const TextScaler.linear(0.9),
            ),
            child: child!,
          ),
          home: const AppShell(),
        ),
      ),
    );
  }
}

ThemeData _lightTheme() {
  return ThemeData(
    colorScheme: ColorScheme.fromSeed(
      seedColor: _rosePrimary,
      brightness: Brightness.light,
    ),
    textTheme: GoogleFonts.poppinsTextTheme(),
    useMaterial3: true,
  );
}

ThemeData _darkTheme() {
  final colorScheme = ColorScheme(
    brightness: Brightness.dark,
    primary: _rosePrimary,
    onPrimary: Colors.white,
    primaryContainer: _rosePrimaryContainer,
    onPrimaryContainer: _roseOnPrimaryContainer,
    secondary: _roseSecondary,
    onSecondary: Colors.white,
    secondaryContainer: const Color(0xFF5e7aab),
    onSecondaryContainer: const Color(0xFFe0e6f2),
    tertiary: const Color(0xFF375990),
    onTertiary: Colors.white,
    tertiaryContainer: const Color(0xFF070b14),
    onTertiaryContainer: const Color(0xFFb8c4e0),
    error: const Color(0xFFcf6679),
    onError: Colors.white,
    errorContainer: const Color(0xFF8c1d2a),
    onErrorContainer: const Color(0xFFf9dedc),
    surface: _onyxBackground,
    onSurface: const Color(0xFFe0e6f2),
    surfaceContainerHighest: _onyxSurfaceVariant,
    onSurfaceVariant: const Color(0xFFb8c4e0),
    outline: _onyxOutline,
    outlineVariant: const Color(0xFF375990),
    shadow: Colors.black,
    scrim: Colors.black,
    inverseSurface: const Color(0xFFe0e6f2),
    onInverseSurface: _onyxBackground,
    inversePrimary: _rosePrimary,
  );

  return ThemeData(
    colorScheme: colorScheme,
    textTheme: GoogleFonts.poppinsTextTheme(ThemeData.dark().textTheme),
    useMaterial3: true,
    scaffoldBackgroundColor: _onyxBackground,
    cardTheme: CardThemeData(
      color: _onyxSurface,
      elevation: 0,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
        side: const BorderSide(color: Color(0xFF1a2540), width: 1),
      ),
    ),
    navigationBarTheme: NavigationBarThemeData(
      backgroundColor: _onyxSurface,
      indicatorColor: _rosePrimary.withValues(alpha: 0.2),
      iconTheme: WidgetStateProperty.resolveWith((states) {
        if (states.contains(WidgetState.selected)) {
          return const IconThemeData(color: _rosePrimary);
        }
        return const IconThemeData(color: Color(0xFF8a9dc6));
      }),
      labelTextStyle: WidgetStateProperty.resolveWith((states) {
        if (states.contains(WidgetState.selected)) {
          return const TextStyle(color: _rosePrimary, fontSize: 12, fontWeight: FontWeight.w600);
        }
        return const TextStyle(color: Color(0xFF8a9dc6), fontSize: 12);
      }),
    ),
    appBarTheme: const AppBarTheme(
      backgroundColor: _onyxBackground,
      foregroundColor: Color(0xFFe0e6f2),
      elevation: 0,
      surfaceTintColor: Colors.transparent,
      systemOverlayStyle: SystemUiOverlayStyle(
        statusBarColor: Colors.transparent,
        statusBarIconBrightness: Brightness.light,
      ),
    ),
    dividerTheme: const DividerThemeData(
      color: Color(0xFF1a2540),
    ),
    inputDecorationTheme: InputDecorationTheme(
      filled: true,
      fillColor: _onyxSurfaceVariant,
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(10),
        borderSide: const BorderSide(color: Color(0xFF1a2540)),
      ),
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(10),
        borderSide: const BorderSide(color: Color(0xFF1a2540)),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(10),
        borderSide: const BorderSide(color: _rosePrimary, width: 1.5),
      ),
    ),
    filledButtonTheme: FilledButtonThemeData(
      style: FilledButton.styleFrom(
        backgroundColor: _rosePrimary,
        foregroundColor: Colors.white,
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
      ),
    ),
    chipTheme: ChipThemeData(
      backgroundColor: _onyxSurfaceVariant,
      selectedColor: _rosePrimary.withValues(alpha: 0.25),
      side: const BorderSide(color: Color(0xFF1a2540)),
      labelStyle: const TextStyle(color: Color(0xFFb8c4e0), fontSize: 12),
    ),
    segmentedButtonTheme: SegmentedButtonThemeData(
      style: SegmentedButton.styleFrom(
        backgroundColor: _onyxSurfaceVariant,
        selectedBackgroundColor: _rosePrimary.withValues(alpha: 0.2),
        foregroundColor: const Color(0xFF8a9dc6),
        selectedForegroundColor: _rosePrimary,
        side: const BorderSide(color: Color(0xFF1a2540)),
      ),
    ),
    searchBarTheme: SearchBarThemeData(
      backgroundColor: WidgetStatePropertyAll(_onyxSurface),
      side: const WidgetStatePropertyAll(BorderSide(color: Color(0xFF1a2540))),
      elevation: const WidgetStatePropertyAll(0),
    ),
    switchTheme: SwitchThemeData(
      thumbColor: WidgetStateProperty.resolveWith((states) =>
          states.contains(WidgetState.selected) ? _rosePrimary : const Color(0xFF5e7aab)),
      trackColor: WidgetStateProperty.resolveWith((states) =>
          states.contains(WidgetState.selected)
              ? _rosePrimary.withValues(alpha: 0.3)
              : const Color(0xFF1a2540)),
    ),
  );
}
