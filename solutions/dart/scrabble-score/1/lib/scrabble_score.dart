int score(String word) {
  var res = 0;
  Map<List<String>, int> scoreMap = {
    ['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']: 1,
    ['D', 'G']: 2,
    ['B', 'M', 'C', 'P']: 3,
    ['F', 'H', 'V', 'W', 'Y']: 4,
    ['K']: 5,
    ['J', 'X']: 8,
    ['Q', 'Z']: 10,
  };

  for (var letter in word.split('')) {
    for (var key in scoreMap.keys) {
      if (key.contains(letter.toUpperCase())) {
        res += scoreMap[key] ?? 0;
        break;
      }
    }
  }

  return res;
}