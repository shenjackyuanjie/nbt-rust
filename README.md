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

Hello, nbt!
============ small test ============
=== nbt v1 ===
time: 872.8818ms
speed: 1768.853469049303 (bytes/s)
1.72739596586846 (KB/s)
=== nbt v2 ===
time: 33.9µs
speed: 45545722.71386431 (bytes/s)
44478.24483775812 (KB/s)
43.43578597437316 (MB/s)
=== nbt v3 ===
time: 54.7µs
speed: 28226691.04204753 (bytes/s)
27565.12797074954 (KB/s)
26.9190702839351 (MB/s)
=== nbt v4 ===
time: 24.3µs
speed: 63539094.65020576 (bytes/s)
62049.897119341564 (KB/s)
60.595602655606996 (MB/s)
=== nbt v5 ===
time: 23.7µs
speed: 65147679.32489452 (bytes/s)
63620.7805907173 (KB/s)
62.129668545622366 (MB/s)
=== fastnbt ===
time: 28.9µs
speed: 53425605.536332175 (bytes/s)
52173.44290657439 (KB/s)
50.95062783845155 (MB/s)
============ cli test ============
=== shen nbt 5 ===
time: 2.2855815sspeed: 2520989434.4174557 (bytes/s)
2461903.7445482966 (KB/s)
2404.202875535446 (MB/s)
2.347854370640084 (GB/s)
```
