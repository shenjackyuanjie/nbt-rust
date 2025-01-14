# shen-nbt5

一个 "全功能" 的 "快速" NBT 解析器

目前进度

- [x] 读取
  - [x] Java
  - [x] JavaNetAfter1_20_2
  - [x] BedrockDisk
  - [x] BedrockNetVarInt
- [x] 写入
  - [x] Java
  - [x] JavaNetAfter1_20_2
  - [x] BedrockDisk
  - [x] BedrockNetVarInt

- [ ] `Serde` 支持 (等待 PR, 我不会写了)
  - [ ] `Serialize`
  - [ ] `Deserialize`
  - [ ] `from_value`
  - [ ] `to_value`

支持

- `Java`
  - 也就是除了 1.20.2+ (协议号 >= 764) 之后的 网络传输用 NBT 格式
  - 数据都是大端序
  - 根节点必须有名称
  - 根节点必须是一个 `NbtCompound(10)` 类型
- `JavaNetAfter1_20_2`
  - Java 版在 1.20.2 之后的网络传输用 NBT 格式
  - 数据都是大端序
  - 根节点没有名称
  - 根节点必须是一个 `NbtCompound(10)` 类型
- `BedrockDisk`
  - 基岩版存在硬盘里的 NBT 格式
  - 数据都是小端序
  - 根节点必须有名称
  - 根节点是 `NbtCompound(10)` 或者 `NbtList(9)` 类型

  - ```text
    与Java版本使用的big-endian格式相同，但所有数字都以little-endian编码。
    这包括标记名称和TAG_String值之前的16位长度前缀，以及TAG_Float和TAG_Double值。

    ---- https://wiki.vg/NBT
    ```

- `BedrockNetVarInt`
  - 基岩版用于网络传输的 NBT 格式

  - ```text
    这种格式比其他格式稍微复杂一些。与Java版本的big-endian格式的区别如下：

    TAG_Short、TAG_Float和TAG_Double值被编码为其小端对应值
    TAG_Int值以及TAG_List、TAG_Byte_Array、TAG_Int_Array和TAG_Long_Array的长度前缀均编码为使用ZigZag编码的VarInt
    TAG_Long值使用ZigZag编码被编码为VarLong
    所有字符串（标记名称和TAG_String值）都以普通的VarInt作为长度前缀

    ---- https://wiki.vg/NBT
    ```

writen in rust!

## 感谢

感谢 [@langyo](https://github.com/langyo) 和 [@InfyniteHeap](https://github.com/InfyniteHeap)
在编写过程中的帮助（

感谢 [mat](https://github.com/mat-1) 的 simd-nbt 中 [`mutf8.rs`](https://github.com/azalea-rs/simdnbt/blob/master/simdnbt/benches/mutf8.rs) 的实现

感谢 [wiki.vg](https://wiki.vg/NBT) 存储的 NBT 格式的详细信息

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
    - 也就 2400 ms/s
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

shen-nbt5 通过了作者电脑上 所有 .nbt 格式的文件的读取测试

```text
total: 6063, open failed: 25, parse failed: 0, gzip parse: 6013, normal parse: 25
```
