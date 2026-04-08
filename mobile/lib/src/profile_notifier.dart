import 'dart:convert';

import 'package:flutter/foundation.dart';

import 'rust/api/user_preference.dart';

class ProfileNotifier extends ChangeNotifier {
  ProfileNotifier._();
  static final instance = ProfileNotifier._();

  String _id = '';
  String _workspaceId = '';
  String firstName = '';
  String lastName = '';
  String email = '';

  String get displayName => firstName.isNotEmpty ? firstName : 'User';
  String get fullName => [firstName, lastName].where((s) => s.isNotEmpty).join(' ');
  String get initials => firstName.isNotEmpty ? firstName[0].toUpperCase() : 'U';

  Future<void> load(String workspaceId) async {
    _workspaceId = workspaceId;
    try {
      final raw = await getUserPreference(metaWorkspaceId: workspaceId);
      if (raw != null && raw.isNotEmpty) {
        final j = jsonDecode(raw) as Map<String, dynamic>;
        _id = j['identifier'] as String? ?? '';
        firstName = j['firstName'] as String? ?? '';
        lastName = j['lastName'] as String? ?? '';
        email = j['email'] as String? ?? '';
        notifyListeners();
      }
    } catch (e) {
      debugPrint('ProfileNotifier.load error: $e');
    }
  }

  Future<void> save({
    required String newFirstName,
    required String newLastName,
    required String newEmail,
    String? workspaceId,
  }) async {
    if (workspaceId != null) _workspaceId = workspaceId;
    try {
      if (_id.isEmpty) {
        final raw = await createUserPreference(
          firstName: newFirstName,
          lastName: newLastName,
          email: newEmail,
          metaWorkspaceId: _workspaceId.isNotEmpty ? _workspaceId : null,
        );
        final j = jsonDecode(raw) as Map<String, dynamic>;
        _id = j['identifier'] as String? ?? '';
      } else {
        await updateUserPreference(
          identifier: _id,
          firstName: newFirstName,
          lastName: newLastName,
          email: newEmail,
          metaWorkspaceId: _workspaceId.isNotEmpty ? _workspaceId : null,
        );
      }
      firstName = newFirstName;
      lastName = newLastName;
      email = newEmail;
      notifyListeners();
    } catch (e) {
      debugPrint('ProfileNotifier.save error: $e');
    }
  }
}
