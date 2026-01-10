import 'dart:math';
class ArmstrongNumbers {
  ArmstrongNumbers();

  bool isArmstrongNumber(String inputNumber) {
    switch (inputNumber.length) {
      case 1:
        return true;
      case < 19:
        return inputNumber
                .split('')
                .fold(
                  0,
                  (sum, el) =>
                      pow(int.parse(el), inputNumber.length).toInt() + sum,
                )
                .toString() ==
            inputNumber;
      case >= 19:
        var sum = BigInt.from(0);
        for (var item in inputNumber.split('')) {
          sum += BigInt.from(int.parse(item)).pow(inputNumber.length);
        }
        return sum.toString() == inputNumber;
      default:
        return false;
    }
  }
}
