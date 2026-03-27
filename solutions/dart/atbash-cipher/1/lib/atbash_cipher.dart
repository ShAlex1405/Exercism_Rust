class AtbashCipher {
  final String plain = 'abcdefghijklmnopqrstuvwxyz';
  final String cipher = 'zyxwvutsrqponmlkjihgfedcba';

  AtbashCipher();

  String encode(String word) {
    final encodedWord = word
        .toLowerCase()
        .replaceAll(RegExp(r'[^a-zA-Zа-яА-ЯёЁ0-9]'), '')
        .split('')
        .map((l) => !plain.contains(l) ? l : cipher[plain.indexOf(l)])
        .join();

    return [
      for (var i = 0; i < encodedWord.length; i++)
        if (i > 0 && i % 5 == 0) ' ${encodedWord[i]}' else encodedWord[i],
    ].join();
  }

  String decode(String word) => word
      .toLowerCase()
      .replaceAll(RegExp(r'[^a-zA-Zа-яА-ЯёЁ0-9]'), '')
      .split('')
      .map((l) => !cipher.contains(l) ? l : plain[cipher.indexOf(l)])
      .join();
}