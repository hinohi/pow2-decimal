# pow2-decimal

## 元ネタ

https://twitter.com/Science_Release/status/1440659660961247242

## やってみると

2の1458642乗まで計算した。

* その中で「2の累乗数の中で、十進法表記で全ての数字が含まていない数字」は92個
* 最大は確かに 2¹⁶⁸=374144419156711147060143317175368453031918731001856
* 次に小さいのは 2¹⁵³=11417981541647679048466287755595961091061972992
* 一番最初に10個全てが現れるのは 2⁶⁸=295147905179352825856

0乗から199乗までの「もっとも出現しない数値の個数」のデータが [0-199.txt](./0-199.txt)。

## ベンチマーク

内部を十進数で表現。

```
$ time ./target/release/pow2-decimal > /dev/null
      239.33 real       238.42 user         0.60 sys
```

内部を百進数で表現、変わらない。

```
$ /usr/bin/time ./target/release/pow2-decimal > /dev/null
      225.09 real       223.88 user         0.61 sys
```
