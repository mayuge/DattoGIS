
# Rustの`&str`と`String`の違い

## `&str`とは？

&strは「文字列スライス」と呼ばれ、自分では文字列の実体を持たず、メモリ上の他の場所にある文字列データを参照するだけの軽量な型です。

```rust
let greeting: &str = "hello";
```
  この場合、"hello"という文字列はプログラムのバイナリ（静的領域）に埋め込まれており、&strはそれをポインタと長さで参照しています。

- 所有権を持たない
- 不変（immutable）
- 軽量
- よく関数の引数などに使われる

## `String`とは？

Stringはヒープ上に確保された文字列の所有者です。可変で、長さを変更したり、新しい文字列を構築したりできます。

```rust
let mut name: String = String::from("Alice");
name.push_str(" Smith");
```

- 所有権を持つ
- 可変（mutable）
- ヒープ上にデータを持つ
- to_string() で簡単に作れる