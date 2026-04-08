import 'package:flutter/material.dart';

/// Gradient header background with a subtle dot-grid pattern overlay.
/// Drop-in replacement for the plain gradient [Container] used in settings
/// [FlexibleSpaceBar.background].
class SettingsHeaderBackground extends StatelessWidget {
  const SettingsHeaderBackground({
    super.key,
    required this.colors,
    required this.child,
  });

  final List<Color> colors;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        gradient: LinearGradient(
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
          colors: colors,
        ),
      ),
      child: Stack(
        children: [
          Positioned.fill(child: CustomPaint(painter: _DotPatternPainter())),
          child,
        ],
      ),
    );
  }
}

class _DotPatternPainter extends CustomPainter {
  const _DotPatternPainter();

  @override
  void paint(Canvas canvas, Size size) {
    final paint = Paint()
      ..color = Colors.white.withValues(alpha: 0.07)
      ..style = PaintingStyle.fill;

    const spacing = 22.0;
    const radius = 2.2;

    for (double row = 0; row * spacing < size.height + spacing; row++) {
      final offsetX = (row % 2 == 0) ? 0.0 : spacing / 2;
      for (double col = 0; col * spacing - offsetX < size.width + spacing; col++) {
        canvas.drawCircle(
          Offset(col * spacing - offsetX + spacing / 2, row * spacing + spacing / 2),
          radius,
          paint,
        );
      }
    }
  }

  @override
  bool shouldRepaint(_DotPatternPainter _) => false;
}
