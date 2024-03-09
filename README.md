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
time: 871.9694ms
speed: 1770.7043389366645 (bytes/s)
1.7292034559928364 (KB/s)
=== nbt v2 ===
time: 36.4µs
speed: 42417582.41758242 (bytes/s)
41423.420329670334 (KB/s)
40.452558915693686 (MB/s)
=== nbt v3 ===
time: 25.8µs
speed: 59844961.24031008 (bytes/s)
58442.34496124031 (KB/s)
57.07260250121124 (MB/s)
=== nbt v4 ===
time: 26.4µs
speed: 58484848.484848484 (bytes/s)
57114.10984848485 (KB/s)
55.775497898910984 (MB/s)
=== nbt v5 ===
time: 24.7µs
speed: 62510121.45748988 (bytes/s)
61045.04048582996 (KB/s)
59.61429734944332 (MB/s)
=== fastnbt ===
time: 38.9µs
speed: 39691516.70951157 (bytes/s)
38761.24678663239 (KB/s)
37.8527800650707 (MB/s)
============ cli test ============
=== shen nbt 5 ===
time: 2.3202808s
speed: 2483288579.985664 (bytes/s)
2425086.50389225 (KB/s)
2368.2485389572753 (MB/s)
2.312742713825464 (GB/s)
```
