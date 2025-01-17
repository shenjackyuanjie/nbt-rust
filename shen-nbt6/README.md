# shen-nbt6

TODO!

这次我会尽量不适用库, 但是像 mutf8 解码 还是会使用库的

- [] 读取
  - [x] Java
    - [ ] borrow -> owned
  - [x] JavaNetAfter1_20_2
    - [ ] borrow -> owned
  - [ ] BedrockDisk
    - [ ] borrow -> owned
  - [ ] BedrockNetVarInt
    - [ ] borrow -> owned
  - [ ] SNBT
- [ ] 写入
  - [ ] Java
  - [ ] JavaNetAfter1_20_2
  - [ ] BedrockDisk
  - [ ] BedrockNetVarInt
  - [x] SNBT ( `impl Display for NbtValue` )

- [ ] `Serde` 支持 (等待 PR, 我不会写了)
  - [ ] `Serialize`
  - [ ] `Deserialize`
  - [ ] `from_value`
  - [ ] `to_value`
