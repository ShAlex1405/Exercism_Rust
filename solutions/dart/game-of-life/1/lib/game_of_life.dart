class GameOfLife {
  List<List<int>> input;
  List<List<int>> resultMatrix = [];
  GameOfLife(this.input);

  void tick() {
    if (input.isEmpty) return;
    // Проходим по каждой линии в матрице
    for (var idx = 0; idx < input.length; idx++) {
      // Добавляем в результурующую матрицу пустую линию
      resultMatrix.add([]);
      // Получаем текущую линию в матрице
      var line = input[idx];
      // Получаем ближайшие соседние линии
      var [orNull, topOrBottom] = idx == 0
          ? [null, input[idx + 1]]
          : idx == input.length - 1
          ? [null, input[input.length - 2]]
          : [input[idx - 1], input[idx + 1]];

      // Идём по каждому элементу линии line
      for (var point = 0; point < line.length; point++) {
        var lifeCount = 0; // Иницилзируем счётчих живых соседних клеток
        var isDead = line[point] == 0
            ? true
            : false; // Устанавливаем первоначальное состояние текущей клетки

        // Считаем живых соседей в зависимости от места нахождения текущей клетки и наличия двух соседних линий
        if (point > 0 && point < line.length - 1) {
          lifeCount +=
              line[point + 1] +
              line[point - 1] +
              topOrBottom![point] +
              topOrBottom[point + 1] +
              topOrBottom[point - 1];
          lifeCount += orNull == null
              ? 0
              : orNull[point] + orNull[point + 1] + orNull[point - 1];
        } else {
          lifeCount += point == 0
              ? line[point + 1] + topOrBottom![point] + topOrBottom[point + 1]
              : line[point - 1] + topOrBottom![point] + topOrBottom[point - 1];
          lifeCount += orNull == null
              ? 0
              : point == 0
              ? orNull[point] + orNull[point + 1]
              : orNull[point] + orNull[point - 1];
        }

        // Устанавливаем значение текущей клетки в зависимости от количества живих соседей
        switch (lifeCount) {
          case 3:
            resultMatrix[idx].add(1);
          case 2:
            resultMatrix[idx].add(isDead ? 0 : 1);
          default:
            resultMatrix[idx].add(0);
        }
      }
    }
    input = resultMatrix;
  }

  List<List<int>> matrix() => input;
}