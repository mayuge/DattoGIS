# GPUI における Atomic Design 設計ガイド

## 概要

Atomic Design は UI を以下の 5 層に分割して管理する設計手法です。

1. Atoms（原子）
2. Molecules（分子）
3. Organisms（生物）
4. Templates（テンプレート）
5. Pages（ページ）

GPUI は React のような Component ベースの UI フレームワークであるため、Atomic Design と非常に相性が良いです。

---

# ディレクトリ構成例

```text
src/
├── components/
│   ├── atoms/
│   │   ├── button.rs
│   │   ├── label.rs
│   │   ├── icon.rs
│   │   └── input.rs
│   │
│   ├── molecules/
│   │   ├── search_box.rs
│   │   ├── coordinate_input.rs
│   │   └── toolbar_button.rs
│   │
│   ├── organisms/
│   │   ├── sidebar.rs
│   │   ├── map_toolbar.rs
│   │   ├── layer_panel.rs
│   │   └── status_bar.rs
│   │
│   ├── templates/
│   │   ├── desktop_layout.rs
│   │   └── map_layout.rs
│   │
│   └── pages/
│       ├── map_page.rs
│       ├── layer_page.rs
│       └── settings_page.rs
│
├── views/
│   └── main_window.rs
│
├── state/
│   ├── map_state.rs
│   ├── layer_state.rs
│   └── app_state.rs
│
└── main.rs
```

---

# Atoms

最小単位の UI。

単独で意味を持たず、再利用性を重視する。

## Button

```rust
pub struct PrimaryButton {
    text: SharedString,
}
```

用途

- 保存
- 開く
- 削除
- OK

など

---

## Label

```rust
pub struct Label {
    text: SharedString,
}
```

用途

- 見出し
- 説明文
- 座標表示

---

## Icon

```rust
pub struct Icon {
    icon_name: SharedString,
}
```

用途

- 拡大
- 縮小
- 移動

---

# Molecules

Atoms を組み合わせる。

1つの機能を持つ。

---

## SearchBox

```text
┌─────────────┐
│ 検索        │
└─────────────┘ 🔍
```

構成

- Input
- Button
- Icon

```rust
pub struct SearchBox {
    query: SharedString,
}
```

---

## CoordinateInput

```text
緯度 [35.68]
経度 [139.76]
```

構成

- Label
- Input

---

# Organisms

Molecules を組み合わせた大きな UI。

GIS アプリの主要機能になる。

---

## LayerPanel

```text
+-------------------+
| レイヤ一覧        |
+-------------------+
| ☑ 建物            |
| ☑ 道路            |
| ☑ 河川            |
+-------------------+
```

構成

- Label
- Button
- List

---

## Sidebar

```text
+-------------------+
| ツール            |
|-------------------|
| 選択              |
| 編集              |
| 計測              |
+-------------------+
```

構成

- ToolButton
- SearchBox
- LayerPanel

---

## MapToolbar

```text
+-------------------------------+
| + | - | Home | Rotate | GPS |
+-------------------------------+
```

構成

- IconButton
- Separator

---

# Templates

画面レイアウトのみを管理する。

データを持たない。

---

## MapLayout

```text
+------------------------------------+
| Toolbar                            |
+------+-----------------------------+
|Side  |                             |
|Bar   |           Map               |
|      |                             |
+------+-----------------------------+
| Status Bar                         |
+------------------------------------+
```

構成

- Sidebar
- Toolbar
- StatusBar

---

## DesktopLayout

```text
+------------------------------------+
| Header                             |
+------------------------------------+
|                                    |
|          Content Area              |
|                                    |
+------------------------------------+
```

---

# Pages

ユーザーが実際に表示する画面。

状態管理を持つ。

---

## MapPage

```rust
pub struct MapPage {
    map_state: Entity<MapState>,
}
```

構成

- MapLayout
- LayerPanel
- MapView

---

## SettingsPage

```rust
pub struct SettingsPage {
    settings_state: Entity<SettingsState>,
}
```

構成

- 設定フォーム
- テーマ変更
- 地図設定

---

# 状態管理との分離

Atomic Design では UI と状態を分離する。

推奨構成

```text
UI
 ↓
Page
 ↓
State
 ↓
Domain
```

例

```rust
MapPage
 ↓
MapState
 ↓
MapService
 ↓
MapRepository
```

---

# GISアプリ（DattoGIS）での適用例

## Atoms

- Button
- Label
- Icon
- TextInput

---

## Molecules

- SearchBox
- CoordinateInput
- LayerItem

---

## Organisms

- LayerPanel
- ToolPanel
- StatusBar
- AttributeTable

---

## Templates

- MapLayout
- DesktopLayout

---

## Pages

- MapPage
- LayerPage
- SettingsPage

---

# GPUIで重要な考え方

GPUIでは

```rust
impl Render for MapToolbar
```

のように Component ごとに Render を実装する。

Atomic Design の各階層をそのまま Rust の構造体に対応させると管理しやすい。

```text
Atom
 ↓
Molecule
 ↓
Organism
 ↓
Template
 ↓
Page
```

各階層は下位階層のみ参照する。

```text
Page
 └─ Organism
      └─ Molecule
            └─ Atom
```

逆方向の依存は禁止する。

```text
Atom → Organism
```

のような参照は作らない。

---

# DattoGIS 推奨構成

```text
src/
├── components/
│   ├── atoms/
│   ├── molecules/
│   ├── organisms/
│   ├── templates/
│   └── pages/
│
├── state/
│
├── domain/
│
├── infrastructure/
│
└── main.rs
```

Atomic Design と DDD を組み合わせることで、

- UI変更に強い
- GIS機能追加が容易
- テストしやすい
- 長期運用しやすい

という構成になる。
