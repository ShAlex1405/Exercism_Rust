class InvalidNucleotideException implements Exception {}

class NucleotideCount {
  NucleotideCount();

  Map<String, int> count(String chain) {
    var nucleotides = {"A": 0, "C": 0, "G": 0, "T": 0};
    for (var el in chain.split('')) {
      if (nucleotides.containsKey(el)) {
        nucleotides[el] = nucleotides[el]! + 1;
      } else {
        throw InvalidNucleotideException();
      }
    }
    return nucleotides;
  }
}