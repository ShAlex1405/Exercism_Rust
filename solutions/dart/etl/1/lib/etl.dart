class Etl {
  Etl();

  Map<String, int> transform(Map<String, List<String>> groups) => {
    for (var e in groups.entries)
      for (var listElement in e.value)
        listElement.toLowerCase(): int.parse(e.key),
  };
}