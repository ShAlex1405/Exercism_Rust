class HighScores {
  final List<int> _inputList;
  HighScores(this._inputList);

  List<int> get scores => _inputList;

  List<int> personalTopThree() {
    List<int> sorted = List.from(_inputList)..sort((a, b) => b.compareTo(a));
    return sorted.length <= 3 ? sorted : sorted.sublist(0, 3);
  }

  int latest() => _inputList.last;

  int personalBest() => (List<int>.from(_inputList)..sort()).last;
}