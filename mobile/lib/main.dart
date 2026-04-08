import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:path_provider/path_provider.dart';
import 'src/app.dart';
import 'src/rust/api/kernel.dart';
import 'src/rust/frb_generated.dart';
import 'src/services/notification_service.dart';
import 'src/pages/onboarding_page.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await SystemChrome.setEnabledSystemUIMode(SystemUiMode.edgeToEdge);

  await RustLib.init();

  final dir = await getApplicationDocumentsDirectory();
  final dbPath = '${dir.path}${Platform.pathSeparator}shurbs.db';
  await initKernel(databaseUrl: 'sqlite://$dbPath?mode=rwc');

  await NotificationService.instance.init();

  final seenOnboarding = await hasSeenOnboarding();

  runApp(App(showOnboarding: !seenOnboarding));
}
