import 'dart:io';
import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:path_provider/path_provider.dart';

import 'setup_page.dart';

// ── First-launch guard ────────────────────────────────────────────────────────

Future<bool> hasSeenOnboarding() async {
  final dir = await getApplicationDocumentsDirectory();
  return File('${dir.path}/.onboarding_done').existsSync();
}

Future<void> markOnboardingSeen() async {
  final dir = await getApplicationDocumentsDirectory();
  await File('${dir.path}/.onboarding_done').create();
}

// ── Slide model ───────────────────────────────────────────────────────────────

class _Slide {
  final String title;
  final String subtitle;
  final Widget Function(Color accent) illustration;

  const _Slide({
    required this.title,
    required this.subtitle,
    required this.illustration,
  });
}

// ── Page ──────────────────────────────────────────────────────────────────────

class OnboardingPage extends StatefulWidget {
  const OnboardingPage({super.key});

  @override
  State<OnboardingPage> createState() => _OnboardingPageState();
}

class _OnboardingPageState extends State<OnboardingPage> {
  final _controller = PageController();
  int _page = 0;

  final List<_Slide> _slides = [
    _Slide(
      title: 'Stay on top\nof your tasks',
      subtitle:
          'Create todos with priorities, track what matters and get things done — one tap at a time.',
      illustration: (color) => _TodoIllustration(color: color),
    ),
    _Slide(
      title: 'Never miss\na reminder',
      subtitle:
          'Set one-off or recurring reminders that fire even when the app is closed, just like an alarm.',
      illustration: (color) => _ReminderIllustration(color: color),
    ),
    _Slide(
      title: 'Capture\neverything',
      subtitle:
          'Write notes, save bookmarks, and keep a calendar — all your thoughts in one calm place.',
      illustration: (color) => _NotesIllustration(color: color),
    ),
  ];

  void _next() {
    if (_page < _slides.length - 1) {
      _controller.nextPage(
        duration: const Duration(milliseconds: 380),
        curve: Curves.easeInOut,
      );
    } else {
      _finish();
    }
  }

  void _finish() async {
    await markOnboardingSeen();
    if (!mounted) return;
    Navigator.of(context).pushReplacement(
      MaterialPageRoute(builder: (_) => const SetupPage()),
    );
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final isLast = _page == _slides.length - 1;
    final colorScheme = Theme.of(context).colorScheme;
    final accent = colorScheme.primary;
    final isDark = Theme.of(context).brightness == Brightness.dark;

    final statusBarHeight = MediaQuery.of(context).padding.top;

    return AnnotatedRegion<SystemUiOverlayStyle>(
      value: SystemUiOverlayStyle(
        statusBarColor: accent,
        statusBarIconBrightness: isDark ? Brightness.light : Brightness.light,
        statusBarBrightness: isDark ? Brightness.dark : Brightness.dark,
      ),
      child: Scaffold(
      backgroundColor: colorScheme.surface,
      body: Stack(
        children: [
          // ── Slides ─────────────────────────────────────────────────────────
          PageView.builder(
            controller: _controller,
            itemCount: _slides.length,
            onPageChanged: (i) => setState(() => _page = i),
            itemBuilder: (_, i) => _SlideView(slide: _slides[i]),
          ),

          // ── Accent status bar fill ─────────────────────────────────────────
          Positioned(
            top: 0,
            left: 0,
            right: 0,
            child: AnimatedContainer(
              duration: const Duration(milliseconds: 300),
              height: statusBarHeight,
              color: accent,
            ),
          ),

          // ── Dot indicators (top center) ────────────────────────────────────
          Positioned(
            top: statusBarHeight + 20,
            left: 0,
            right: 0,
            child: Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: List.generate(_slides.length, (i) {
                final active = i == _page;
                return AnimatedContainer(
                  duration: const Duration(milliseconds: 280),
                  margin: const EdgeInsets.symmetric(horizontal: 4),
                  width: active ? 24 : 8,
                  height: 8,
                  decoration: BoxDecoration(
                    color: active
                        ? accent
                        : accent.withValues(alpha: 0.2),
                    borderRadius: BorderRadius.circular(4),
                  ),
                );
              }),
            ),
          ),

          // ── Bottom nav ─────────────────────────────────────────────────────
          Positioned(
            bottom: MediaQuery.of(context).padding.bottom + 32,
            left: 32,
            right: 32,
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                // Skip
                TextButton(
                  onPressed: _finish,
                  style: TextButton.styleFrom(
                    foregroundColor: accent.withValues(alpha: 0.6),
                    padding: EdgeInsets.zero,
                    minimumSize: const Size(48, 48),
                  ),
                  child: const Text(
                    'Skip',
                    style: TextStyle(fontSize: 15, fontWeight: FontWeight.w500),
                  ),
                ),

                // Next / Get started
                GestureDetector(
                  onTap: _next,
                  child: AnimatedContainer(
                    duration: const Duration(milliseconds: 280),
                    width: 56,
                    height: 56,
                    decoration: BoxDecoration(
                      color: accent,
                      borderRadius: BorderRadius.circular(28),
                    ),
                    child: Center(
                      child: AnimatedSwitcher(
                        duration: const Duration(milliseconds: 200),
                        child: isLast
                            ? const Icon(Icons.check_rounded,
                                color: Colors.white, size: 24, key: ValueKey('check'))
                            : const Icon(Icons.arrow_forward_rounded,
                                color: Colors.white, size: 24, key: ValueKey('arrow')),
                      ),
                    ),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    ));
  }
}

// ── Slide view ────────────────────────────────────────────────────────────────

class _SlideView extends StatelessWidget {
  final _Slide slide;
  const _SlideView({required this.slide});

  @override
  Widget build(BuildContext context) {
    final size = MediaQuery.of(context).size;
    final accent = Theme.of(context).colorScheme.primary;
    final cardBg = Color.alphaBlend(accent.withValues(alpha: 0.10), Colors.white);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Illustration card
        Container(
          width: double.infinity,
          height: size.height * 0.52,
          color: cardBg,
          child: Center(child: slide.illustration(accent)),
        ),

        // Text content
        Padding(
          padding: const EdgeInsets.fromLTRB(32, 36, 32, 0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                slide.title,
                style: const TextStyle(
                  fontSize: 30,
                  fontWeight: FontWeight.w700,
                  color: Color(0xFF1a1a2e),
                  height: 1.2,
                  letterSpacing: -0.5,
                ),
              ),
              const SizedBox(height: 16),
              Text(
                slide.subtitle,
                style: TextStyle(
                  fontSize: 15,
                  fontWeight: FontWeight.w400,
                  color: const Color(0xFF1a1a2e).withValues(alpha: 0.5),
                  height: 1.6,
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }
}

// ── Illustrations ─────────────────────────────────────────────────────────────

class _TodoIllustration extends StatelessWidget {
  final Color color;
  const _TodoIllustration({required this.color});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 220,
      height: 220,
      child: CustomPaint(painter: _TodoPainter(color)),
    );
  }
}

class _TodoPainter extends CustomPainter {
  final Color color;
  const _TodoPainter(this.color);

  @override
  void paint(Canvas canvas, Size size) {
    final cx = size.width / 2;
    final cy = size.height / 2;

    // Card shadow
    final shadowPaint = Paint()
      ..color = Colors.black.withOpacity(0.07)
      ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 12);
    canvas.drawRRect(
      RRect.fromRectAndRadius(
        Rect.fromCenter(center: Offset(cx, cy + 4), width: 160, height: 190),
        const Radius.circular(20),
      ),
      shadowPaint,
    );

    // Card body
    canvas.drawRRect(
      RRect.fromRectAndRadius(
        Rect.fromCenter(center: Offset(cx, cy), width: 160, height: 190),
        const Radius.circular(20),
      ),
      Paint()..color = Colors.white,
    );

    // Header bar
    canvas.drawRRect(
      RRect.fromRectAndRadius(
        Rect.fromLTWH(cx - 80, cy - 95, 160, 48),
        const Radius.circular(20),
      ),
      Paint()..color = color,
    );

    // Header text lines
    final linePaint = Paint()
      ..color = Colors.white.withOpacity(0.6)
      ..strokeCap = StrokeCap.round
      ..strokeWidth = 4;
    canvas.drawLine(Offset(cx - 52, cy - 75), Offset(cx + 10, cy - 75), linePaint);
    canvas.drawLine(Offset(cx - 52, cy - 60), Offset(cx - 10, cy - 60), linePaint);

    // Todo rows
    void drawRow(double y, bool checked, Color dotColor) {
      final circlePaint = Paint()
        ..color = checked ? dotColor : Colors.transparent
        ..style = PaintingStyle.fill;
      final circleBorder = Paint()
        ..color = dotColor
        ..style = PaintingStyle.stroke
        ..strokeWidth = 2;
      canvas.drawCircle(Offset(cx - 55, y), 9, circlePaint);
      canvas.drawCircle(Offset(cx - 55, y), 9, circleBorder);

      if (checked) {
        final checkPaint = Paint()
          ..color = Colors.white
          ..style = PaintingStyle.stroke
          ..strokeWidth = 2
          ..strokeCap = StrokeCap.round;
        final path = Path()
          ..moveTo(cx - 60, y)
          ..lineTo(cx - 56, y + 4)
          ..lineTo(cx - 49, y - 4);
        canvas.drawPath(path, checkPaint);
      }

      final lp = Paint()
        ..color = checked
            ? const Color(0xFF1a1a2e).withOpacity(0.15)
            : const Color(0xFF1a1a2e).withOpacity(0.3)
        ..strokeCap = StrokeCap.round
        ..strokeWidth = 3;
      canvas.drawLine(Offset(cx - 40, y), Offset(cx + 58, y), lp);
    }

    drawRow(cy - 20, true, color);
    drawRow(cy + 10, true, color);
    drawRow(cy + 40, false, const Color(0xFFD1D5DB));
    drawRow(cy + 70, false, const Color(0xFFD1D5DB));

    // Floating + badge
    canvas.drawCircle(Offset(cx + 66, cy - 76), 18, Paint()..color = color);
    final plusPaint = Paint()
      ..color = Colors.white
      ..strokeWidth = 2.5
      ..strokeCap = StrokeCap.round;
    canvas.drawLine(Offset(cx + 66, cy - 83), Offset(cx + 66, cy - 69), plusPaint);
    canvas.drawLine(Offset(cx + 59, cy - 76), Offset(cx + 73, cy - 76), plusPaint);
  }

  @override
  bool shouldRepaint(_TodoPainter old) => old.color != color;
}

class _ReminderIllustration extends StatelessWidget {
  final Color color;
  const _ReminderIllustration({required this.color});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 220,
      height: 220,
      child: CustomPaint(painter: _ReminderPainter(color)),
    );
  }
}

class _ReminderPainter extends CustomPainter {
  final Color color;
  const _ReminderPainter(this.color);

  @override
  void paint(Canvas canvas, Size size) {
    final cx = size.width / 2;
    final cy = size.height / 2;

    // Dashed orbit circle
    final orbitPaint = Paint()
      ..color = color.withOpacity(0.3)
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2;
    const dashCount = 20;
    const r = 88.0;
    for (int i = 0; i < dashCount; i++) {
      final startAngle = (i * 2 * math.pi / dashCount);
      final endAngle = startAngle + (math.pi / dashCount) * 0.6;
      canvas.drawArc(
        Rect.fromCenter(center: Offset(cx, cy), width: r * 2, height: r * 2),
        startAngle,
        endAngle,
        false,
        orbitPaint,
      );
    }

    // Clock shadow
    final shadowPaint = Paint()
      ..color = Colors.black.withOpacity(0.08)
      ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 10);
    canvas.drawCircle(Offset(cx, cy + 4), 62, shadowPaint);

    // Clock face
    canvas.drawCircle(Offset(cx, cy), 62, Paint()..color = Colors.white);

    // Clock rim
    canvas.drawCircle(
      Offset(cx, cy),
      62,
      Paint()
        ..color = color
        ..style = PaintingStyle.stroke
        ..strokeWidth = 3,
    );

    // Hour marks
    final markPaint = Paint()
      ..color = const Color(0xFF1a1a2e).withOpacity(0.15)
      ..strokeWidth = 2
      ..strokeCap = StrokeCap.round;
    for (int i = 0; i < 12; i++) {
      final angle = i * math.pi / 6 - math.pi / 2;
      final outer = Offset(cx + 54 * math.cos(angle), cy + 54 * math.sin(angle));
      final inner = Offset(cx + 46 * math.cos(angle), cy + 46 * math.sin(angle));
      canvas.drawLine(inner, outer, markPaint);
    }

    // Hour hand
    final hourAngle = 8 * math.pi / 6 - math.pi / 2;
    canvas.drawLine(
      Offset(cx, cy),
      Offset(cx + 30 * math.cos(hourAngle), cy + 30 * math.sin(hourAngle)),
      Paint()
        ..color = const Color(0xFF1a1a2e)
        ..strokeWidth = 4
        ..strokeCap = StrokeCap.round,
    );

    // Minute hand
    final minAngle = -math.pi / 2;
    canvas.drawLine(
      Offset(cx, cy),
      Offset(cx + 44 * math.cos(minAngle), cy + 44 * math.sin(minAngle)),
      Paint()
        ..color = color
        ..strokeWidth = 3
        ..strokeCap = StrokeCap.round,
    );

    // Center dot
    canvas.drawCircle(Offset(cx, cy), 5, Paint()..color = const Color(0xFF1a1a2e));

    // Bell badge
    canvas.drawCircle(Offset(cx, cy - 82), 16, Paint()..color = color);
    canvas.drawCircle(
      Offset(cx, cy - 82),
      16,
      Paint()
        ..color = Colors.white
        ..style = PaintingStyle.stroke
        ..strokeWidth = 2,
    );
    final bLine = Paint()
      ..color = Colors.white
      ..strokeWidth = 2
      ..strokeCap = StrokeCap.round;
    canvas.drawLine(Offset(cx - 6, cy - 84), Offset(cx - 6, cy - 76), bLine);
    canvas.drawLine(Offset(cx + 6, cy - 84), Offset(cx + 6, cy - 76), bLine);
    canvas.drawLine(Offset(cx - 6, cy - 76), Offset(cx + 6, cy - 76), bLine);

    // Floating dots on orbit
    void dot(double angle, Color c) {
      canvas.drawCircle(
        Offset(cx + r * math.cos(angle), cy + r * math.sin(angle)),
        8,
        Paint()..color = c,
      );
    }

    dot(-math.pi / 4, color);
    dot(math.pi * 0.75, color.withOpacity(0.4));
  }

  @override
  bool shouldRepaint(_ReminderPainter old) => old.color != color;
}

class _NotesIllustration extends StatelessWidget {
  final Color color;
  const _NotesIllustration({required this.color});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 220,
      height: 220,
      child: CustomPaint(painter: _NotesPainter(color)),
    );
  }
}

class _NotesPainter extends CustomPainter {
  final Color color;
  const _NotesPainter(this.color);

  @override
  void paint(Canvas canvas, Size size) {
    final cx = size.width / 2;
    final cy = size.height / 2;

    void drawCard(Offset center, double w, double h, double angle, Color cardColor) {
      canvas.save();
      canvas.translate(center.dx, center.dy);
      canvas.rotate(angle);
      final shadowPaint = Paint()
        ..color = Colors.black.withOpacity(0.06)
        ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 8);
      canvas.drawRRect(
        RRect.fromRectAndRadius(
          Rect.fromCenter(center: const Offset(0, 4), width: w, height: h),
          const Radius.circular(14),
        ),
        shadowPaint,
      );
      canvas.drawRRect(
        RRect.fromRectAndRadius(
          Rect.fromCenter(center: Offset.zero, width: w, height: h),
          const Radius.circular(14),
        ),
        Paint()..color = cardColor,
      );
      final lp = Paint()
        ..color = Colors.black.withOpacity(0.12)
        ..strokeWidth = 3
        ..strokeCap = StrokeCap.round;
      for (int i = 0; i < 4; i++) {
        final y = -h / 2 + 28 + i * 16.0;
        canvas.drawLine(Offset(-w / 2 + 16, y), Offset(w / 2 - 16, y), lp);
      }
      canvas.restore();
    }

    // Back card — deeper tint
    drawCard(Offset(cx + 18, cy + 10), 130, 160, 0.15, color.withOpacity(0.35));
    // Middle card — lighter tint
    drawCard(Offset(cx - 12, cy - 8), 130, 160, -0.1, color.withOpacity(0.18));
    // Front card — white
    drawCard(Offset(cx + 2, cy), 130, 160, 0.0, Colors.white);

    // Bookmark ribbon
    final ribbonPath = Path()
      ..moveTo(cx + 52, cy - 80)
      ..lineTo(cx + 68, cy - 80)
      ..lineTo(cx + 68, cy - 44)
      ..lineTo(cx + 60, cy - 50)
      ..lineTo(cx + 52, cy - 44)
      ..close();
    canvas.drawPath(ribbonPath, Paint()..color = color);

    // Lines on front card
    final fp = Paint()
      ..color = const Color(0xFF1a1a2e).withOpacity(0.18)
      ..strokeWidth = 3
      ..strokeCap = StrokeCap.round;
    canvas.drawLine(Offset(cx - 46, cy - 42), Offset(cx + 30, cy - 42), fp);
    canvas.drawLine(Offset(cx - 46, cy - 26), Offset(cx + 46, cy - 26), fp);
    canvas.drawLine(Offset(cx - 46, cy - 10), Offset(cx + 46, cy - 10), fp);
    canvas.drawLine(Offset(cx - 46, cy + 6), Offset(cx + 20, cy + 6), fp);
  }

  @override
  bool shouldRepaint(_NotesPainter old) => old.color != color;
}
