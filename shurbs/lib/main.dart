import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:path_provider/path_provider.dart';
// import 'src/rust/kernel.dart';
import 'src/app.dart';
import 'src/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await SystemChrome.setEnabledSystemUIMode(SystemUiMode.edgeToEdge);

  await RustLib.init();

  final dir = await getApplicationDocumentsDirectory();
  final dbPath = '${dir.path}${Platform.pathSeparator}shurbs.db';
  // await initKernel(databaseUrl: 'sqlite://$dbPath?mode=rwc');

  runApp(const App());
}
