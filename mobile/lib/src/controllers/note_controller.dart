import 'dart:convert';

import 'package:flutter/foundation.dart';

import '../models/note_model.dart';
import '../rust/api/notes.dart';

class NoteController extends ChangeNotifier {
  List<Note> _notes = [];
  bool loading = true;
  String search = '';
  String? _workspaceId;

  List<Note> get notes => List.unmodifiable(_notes);

  List<Note> get filtered {
    if (search.trim().isEmpty) return _notes;
    final q = search.toLowerCase();
    return _notes
        .where(
          (n) => n.title.toLowerCase().contains(q) || n.content.toLowerCase().contains(q),
        )
        .toList();
  }

  static List<Note> _parse(String raw) {
    final list = (jsonDecode(raw) as List).cast<Map<String, dynamic>>();
    return list.map(Note.fromJson).toList();
  }

  Future<void> load(String workspaceId) async {
    _workspaceId = workspaceId;
    try {
      final raw = await getAllNotes(metaWorkspaceId: workspaceId);
      _notes = await compute(_parse, raw);
    } catch (e) {
      debugPrint('NoteController.load error: $e');
    }
    loading = false;
    notifyListeners();
  }

  Future<Note?> create() async {
    if (_workspaceId == null) return null;
    try {
      final raw = await createNote(
        title: 'Untitled',
        content: '',
        metaWorkspaceId: _workspaceId,
      );
      final json = jsonDecode(raw) as Map<String, dynamic>;
      final note = Note.fromJson(json);
      _notes.insert(0, note);
      notifyListeners();
      return note;
    } catch (e) {
      debugPrint('NoteController.create error: $e');
      return null;
    }
  }

  Future<void> update(Note note, String title, String content) async {
    if (_workspaceId == null) return;
    try {
      await updateNote(
        identifier: note.id,
        title: title,
        content: content,
        metaWorkspaceId: _workspaceId,
      );
      note.title = title;
      note.content = content;
      note.updatedAt = DateTime.now();
      notifyListeners();
    } catch (e) {
      debugPrint('NoteController.update error: $e');
    }
  }

  Future<void> delete(Note note) async {
    if (_workspaceId == null) return;
    try {
      await deleteNote(identifier: note.id, metaWorkspaceId: _workspaceId);
      _notes.removeWhere((n) => n.id == note.id);
      notifyListeners();
    } catch (e) {
      debugPrint('NoteController.delete error: $e');
    }
  }

  void setSearch(String s) {
    search = s;
    notifyListeners();
  }
}
