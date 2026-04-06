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

TextTheme _poppins([TextTheme? base]) {
  return GoogleFonts.poppinsTextTheme(base).copyWith(
    bodyMedium: GoogleFonts.poppins(
      fontSize: 15,
      textStyle: base?.bodyMedium,
    ),
  );
}

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
          theme: _lightTheme(accent),
          darkTheme: _darkTheme(accent),
          home: const AppShell(),
        ),
      ),
    );
  }
}

ThemeData _lightTheme(AccentSwatch accent) {
  return ThemeData(
    colorScheme: ColorScheme.fromSeed(
      seedColor: accent.primary,
      brightness: Brightness.light,
    ),
    textTheme: _poppins(),
    useMaterial3: true,
    cardTheme: CardThemeData(
      elevation: 0,
      shadowColor: Colors.transparent,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
      ),
    ),
    appBarTheme: AppBarTheme(
      backgroundColor: Colors.transparent,
      foregroundColor: accent.primary,
      elevation: 0,
      scrolledUnderElevation: 0,
      surfaceTintColor: Colors.transparent,
      systemOverlayStyle: const SystemUiOverlayStyle(
        statusBarColor: Colors.transparent,
        statusBarIconBrightness: Brightness.dark,
        statusBarBrightness: Brightness.light,
      ),
    ),
    searchBarTheme: const SearchBarThemeData(
      elevation: WidgetStatePropertyAll(0),
      surfaceTintColor: WidgetStatePropertyAll(Colors.transparent),
    ),
  );
}

ThemeData _darkTheme(AccentSwatch accent) {
  final colorScheme = ColorScheme(
    brightness: Brightness.dark,
    primary: accent.primary,
    onPrimary: Colors.white,
    primaryContainer: accent.primaryContainer,
    onPrimaryContainer: accent.onPrimaryContainer,
    secondary: accent.secondary,
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
    inversePrimary: accent.primary,
  );

  return ThemeData(
    colorScheme: colorScheme,
    textTheme: _poppins(ThemeData.dark().textTheme),
    useMaterial3: true,
    scaffoldBackgroundColor: _onyxBackground,
    cardTheme: CardThemeData(
      color: _onyxSurface,
      elevation: 0,
      shadowColor: Colors.transparent,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
        side: const BorderSide(color: Color(0xFF1a2540), width: 1),
      ),
    ),
    navigationBarTheme: NavigationBarThemeData(
      backgroundColor: _onyxSurface,
      indicatorColor: accent.primary.withValues(alpha: 0.2),
      iconTheme: WidgetStateProperty.resolveWith((states) {
        if (states.contains(WidgetState.selected)) {
          return IconThemeData(color: accent.primary);
        }
        return const IconThemeData(color: Color(0xFF8a9dc6));
      }),
      labelTextStyle: WidgetStateProperty.resolveWith((states) {
        if (states.contains(WidgetState.selected)) {
          return TextStyle(color: accent.primary, fontSize: 12, fontWeight: FontWeight.w600);
        }
        return const TextStyle(color: Color(0xFF8a9dc6), fontSize: 12);
      }),
    ),
    appBarTheme: AppBarTheme(
      backgroundColor: accent.primary,
      foregroundColor: Colors.white,
      elevation: 0,
      surfaceTintColor: Colors.transparent,
      systemOverlayStyle: const SystemUiOverlayStyle(
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
        borderSide: BorderSide(color: accent.primary, width: 1.5),
      ),
    ),
    filledButtonTheme: FilledButtonThemeData(
      style: FilledButton.styleFrom(
        backgroundColor: accent.primary,
        foregroundColor: Colors.white,
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
      ),
    ),
    chipTheme: ChipThemeData(
      backgroundColor: _onyxSurfaceVariant,
      selectedColor: accent.primary.withValues(alpha: 0.25),
      side: const BorderSide(color: Color(0xFF1a2540)),
      labelStyle: const TextStyle(color: Color(0xFFb8c4e0), fontSize: 12),
    ),
    segmentedButtonTheme: SegmentedButtonThemeData(
      style: SegmentedButton.styleFrom(
        backgroundColor: _onyxSurfaceVariant,
        selectedBackgroundColor: accent.primary.withValues(alpha: 0.2),
        foregroundColor: const Color(0xFF8a9dc6),
        selectedForegroundColor: accent.primary,
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
          states.contains(WidgetState.selected) ? accent.primary : const Color(0xFF5e7aab)),
      trackColor: WidgetStateProperty.resolveWith((states) =>
          states.contains(WidgetState.selected)
              ? accent.primary.withValues(alpha: 0.3)
              : const Color(0xFF1a2540)),
    ),
  );
}
