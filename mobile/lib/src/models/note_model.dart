class Note {
  final String id;
  String title;
  String content;
  DateTime updatedAt;

  Note({
    required this.id,
    required this.title,
    required this.content,
    required this.updatedAt,
  });

  factory Note.fromJson(Map<String, dynamic> j) => Note(
        id: j['identifier'] as String,
        title: j['title'] as String,
        content: j['content'] as String,
        updatedAt: DateTime.parse(j['updatedAt'] as String),
      );
}
