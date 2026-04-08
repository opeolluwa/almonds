import 'dart:convert';

import 'package:flutter/foundation.dart';

import '../models/bookmark_model.dart';
import '../rust/api/bookmarks.dart';

class BookmarkController extends ChangeNotifier {
  List<Bookmark> _bookmarks = [];
  bool loading = true;
  String? activeTag;
  String? _workspaceId;

  List<Bookmark> get bookmarks => List.unmodifiable(_bookmarks);

  List<String> get tags {
    final tags = _bookmarks.map((b) => b.tag).whereType<String>().toSet().toList();
    tags.sort();
    return tags;
  }

  List<Bookmark> get filtered {
    if (activeTag == null) return _bookmarks;
    return _bookmarks.where((b) => b.tag == activeTag).toList();
  }

  static List<Bookmark> _parse(String raw) {
    final list = (jsonDecode(raw) as List).cast<Map<String, dynamic>>();
    return list.map(Bookmark.fromJson).toList();
  }

  Future<void> load(String workspaceId) async {
    _workspaceId = workspaceId;
    try {
      final raw = await getAllBookmarks(metaWorkspaceId: workspaceId);
      _bookmarks = await compute(_parse, raw);
    } catch (e) {
      debugPrint('BookmarkController.load error: $e');
    }
    loading = false;
    notifyListeners();
  }

  Future<void> create(String title, String url, String tag) async {
    if (_workspaceId == null) return;
    try {
      final raw = await createBookmark(
        title: title,
        url: url,
        tag: tag,
        metaWorkspaceId: _workspaceId,
      );
      final json = jsonDecode(raw) as Map<String, dynamic>;
      _bookmarks.insert(0, Bookmark.fromJson(json));
      notifyListeners();
    } catch (e) {
      debugPrint('BookmarkController.create error: $e');
    }
  }

  Future<void> delete(Bookmark bookmark) async {
    if (_workspaceId == null) return;
    try {
      await deleteBookmark(identifier: bookmark.id, metaWorkspaceId: _workspaceId);
      _bookmarks.removeWhere((b) => b.id == bookmark.id);
      notifyListeners();
    } catch (e) {
      debugPrint('BookmarkController.delete error: $e');
    }
  }

  void setActiveTag(String? tag) {
    activeTag = activeTag == tag ? null : tag;
    notifyListeners();
  }
}
