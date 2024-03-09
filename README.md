# nbt-rust

nbt解析器 by shenjack

writen in rust!

感谢 @神楽坂柚咲/伊欧/langyo

在编写过程中的帮助（

## 概况

- `shen-nbt1`
  - 几周的技术积累
  - 100 mb/s

- `shen-nbt2`
  - 2个月的技术积累
  - 500 mb/s

- `shen-nbt3/4`
  - 半年的技术积累
  - v3 有单一依赖库
  - v4 无依赖库
  - 2000 mb/s

- `shen-nbt5` (编写中)
  - 一年左右的技术积累
  - 4000 mb/s ?
  - 支持 `serde` 序列化/反序列化

## 测试数据

解压 `test-data.zip` 到 `test-data` 文件夹

```text
❯ cargo run --release -- .\test-data\test-zip

============ small test ============
=== nbt v1 ===
time: 1.2222941s  speed: 1263.1984397208494 (bytes/sec)
1.233592226289892 (kb/sec)
0.0012046799084862226 (mb/sec)
1.1764452231310768e-6 (gb/sec)
10 5
Level
=== nbt v2 ===
time: 269.1µs  speed: 5737643.9985135645 (bytes/sec)
5603.167967298403 (kb/sec)
5.4718437180648465 (mb/sec)
0.005343597380922702 (gb/sec)
=== nbt v3 ===
time: 58µs  speed: 26620689.655172415 (bytes/sec)
25996.76724137931 (kb/sec)
25.387468009159484 (mb/sec)
0.02479244922769481 (gb/sec)
10 5
Level
=== nbt v4 ===
time: 211.7µs  speed: 7293339.631554086 (bytes/sec)
7122.401983939537 (kb/sec)
6.955470687440954 (mb/sec)
0.006792451843204057 (gb/sec)
=== nbt v5 ===
time: 34.6µs  speed: 44624277.456647396 (bytes/sec)
43578.39595375722 (kb/sec)
42.55702729859104 (mb/sec)
0.04155959697128031 (gb/sec)
=== fastnbt ===
time: 37.7µs  speed: 40954907.161803715 (bytes/sec)
39995.02652519894 (kb/sec)
39.05764309101459 (mb/sec)
0.038142229581068936 (gb/sec)
============ cli test ============
=== shen nbt 5 ===
time: 2.2960054s  speed: 2509544103.424147 (bytes/sec)
2450726.6635001437 (kb/sec)
2393.287757324359 (mb/sec)
2.3371950755120694 (gb/sec)
```
