class EggCounter {
  EggCounter();

  int count(int decEggs) => decEggs
      .toRadixString(2)
      .split('')
      .map((n) => int.parse(n))
      .fold(0, (prev, next) => prev + next);
}