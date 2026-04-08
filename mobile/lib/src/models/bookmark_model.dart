class Bookmark {
  final String id;
  final String title;
  final String url;
  final String? tag;

  const Bookmark({
    required this.id,
    required this.title,
    required this.url,
    this.tag,
  });

  factory Bookmark.fromJson(Map<String, dynamic> j) => Bookmark(
        id: j['identifier'] as String,
        title: j['title'] as String,
        url: j['url'] as String,
        tag: j['tag'] as String?,
      );
}
