<img src="https://capsule-render.vercel.app/api?type=waving&color=auto&height=300&section=header&text=datto_gis&fontSize=90&animation=fadeIn&fontAlignY=38&descAlignY=51&descAlign=62"/>

## GISとは
- GIS（Geographic Information System）は、地理情報を地図と結びつけて管理・分析・可視化する仕組みです。
- 位置情報、道路、建物、地形、各種レイヤーを組み合わせて、空間的な意思決定を支援します。

## datto_gis
- datto_gisは、Rustで作る軽量なGISアプリケーションを目指したプロジェクトです。
- 由来は「脱兎の如く」から来ており、スピーディーで素早い動作感を重視しています。
- 目標としては、既存のGISソフトに対抗できるような、速くて直感的な地図操作を実現することです。
- ロゴはウサギをモチーフにしており、RustのRとrabbitのRをかけています。

## 実行方法
Rustの環境が整っていれば、プロジェクト直下でそのまま実行できます。

```bash
cargo run
```

必要に応じて、プロジェクトルートに移動してから実行します。

```bash
cd DattoGIS
cargo run
```

## プロジェクトの考え方
このアプリは、UIの描画と地図ロジックを分離し、責務ごとにモジュール化して構成しています。

- `apps` には画面とUIコンポーネントを配置
- `domain` には設定値・型・トレイト定義を配置
- `infrastructure` には外部ライブラリやHTTP、JSON、座標変換などの具体実装を配置
- `main.rs` でアプリ起動とUIの初期化を行う

こうすることで、地図描画の状態管理、API通信、UIの配置、設定値管理を整理しやすくしています。

## ディレクトリ構成

```text
DattoGIS/
├── assets/                     # アセット画像や地図関連の静的ファイル
│   ├── activity_bar/
│   ├── components/
│   ├── config/
│   ├── geo/
│   ├── main_logo/
│   ├── map/
│   └── window/
├── src/                        # ソースコード
│   ├── main.rs                 # アプリケーションのエントリーポイント
│   ├── apps/                   # アプリケーションUIの構成
│   │   ├── app.rs              # ルートアプリ本体
│   │   ├── mod.rs
│   │   ├── organisms/          # 画面の機能単位コンポーネント
│   │   │   ├── common/         # 共有コンポーネント・ユーティリティ
│   │   │   │   ├── components/
│   │   │   │   │   ├── atoms/
│   │   │   │   │   └── molecules/
│   │   │   │   └── services/
│   │   │   │       └── map/
│   │   │   └── main_window/    # メイン画面の構成要素
│   │   │       ├── activity_bar_app.rs
│   │   │       ├── footer_app.rs
│   │   │       ├── layer_controller_app.rs
│   │   │       ├── search_app.rs
│   │   │       ├── map/
│   │   │       │   ├── map_app.rs
│   │   │       │   ├── map_content_app.rs
│   │   │       │   ├── map_viewport_app.rs
│   │   │       │   ├── raster_tile_layer_app.rs
│   │   │       │   ├── vector_layer_app.rs
│   │   │       │   └── mod.rs
│   │   │       └── mod.rs
│   │   └── templates/
│   │       ├── main_window.rs
│   │       └── mod.rs
│   ├── domain/                 # ドメイン定義
│   │   ├── mod.rs
│   │   ├── params/             # 設定値や定数
│   │   │   ├── api_config.rs
│   │   │   ├── app_config.rs
│   │   │   ├── design_token_config.rs
│   │   │   ├── map_config.rs
│   │   │   ├── mod.rs
│   │   │   └── text_config.rs
│   │   ├── traits/             # 実装に依存しない振る舞い定義
│   │   │   ├── coordinate_transformer_trait.rs
│   │   │   ├── geocoding_trait.rs
│   │   │   ├── load_raster_tile_json_trait.rs
│   │   │   ├── load_vector_json_trait.rs
│   │   │   ├── map_area_trait.rs
│   │   │   ├── map_event_trait.rs
│   │   │   ├── map_tile_trait.rs
│   │   │   ├── mod.rs
│   │   │   ├── vector_layer_service_trait.rs
│   │   │   └── vector_repository_trait.rs
│   │   └── types/              # ドメインの型定義
│   │       ├── geocoding_type.rs
│   │       ├── map_coordinate_type.rs
│   │       ├── map_layer_type.rs
│   │       └── mod.rs
│   └── infrastructure/         # 外部依存の具体実装
│       ├── client/
│       │   └── index.rs
│       ├── coordinate/
│       ├── duckdb/
│       ├── geocoding/
│       ├── json/
│       ├── mod.rs
│       └── ...
├── Cargo.toml                  # Cargo設定
├── Cargo.lock
├── README.md
└── target/                     # ビルド成果物
```

## UIと地図の責務分離
現在の構造では、アプリの画面は主に `apps` 配下にあります。

### 1. `apps/app.rs`
- アプリのルートを作成する
- GPUIのウィンドウ生成と初期設定を行う
- `MainWindow` を生成して、画面全体の描画に接続する

### 2. `apps/templates/main_window.rs`
- アプリの画面全体のレイアウトを組み立てる
- `MapApp`、`SearchApp`、`FooterApp`、`LayerControllerApp` などを配置する
- 地図画面の中心にクロスヘアなどの装飾も描画する

### 3. `apps/organisms/main_window/*`
- 役割ごとに分割された機能群
- 例:
  - `activity_bar_app.rs`: 左側の機能バー
  - `search_app.rs`: 検索入力
  - `layer_controller_app.rs`: レイヤー表示/透過度の制御
  - `footer_app.rs`: ステータスや補助情報
  - `map/`: 地図の描画・操作実装

### 4. `apps/organisms/common`
- 画面全体で再利用する共通部品が置かれている場所
- `components/atoms` と `components/molecules` にはUI部品があり、
  `services/map` には地図計算やイベント処理ロジックが配置されている

## ドメイン層の役割
`src/domain` は、アプリの中核となる定義をまとめた層です。

- `params/`: 色、サイズ、地図の設定値、テキスト定数を管理
- `traits/`: 地図エリア計算、座標変換、タイル読込、ベクターレイヤー処理などのインターフェースを定義
- `types/`: 座標、レイヤー、ジオコーディング関連の型を定義

この層は、UIや外部ライブラリの実装詳細に依存しないように設計されています。

## インフラ層の役割
`src/infrastructure` は、外部依存や具体実装を閉じ込める層です。

- `client`: HTTP通信クライアント
- `geocoding`: 住所や座標の変換処理
- `coordinate`: 座標変換や投影計算
- `duckdb`: SQLite的な分析用データベースアクセス
- `json`: JSONの読み込みやデータ変換

これにより、UIやドメイン層からは「何をしたいか」に集中でき、具体的な実装はインフラ層に隠す設計になっています。

## 今の開発の特徴
現時点の構成では、次のような設計意図が見えてきます。

- 画面ごとに `organisms` に分割して、機能が追いやすい
- 地図の計算ロジックを `services/map` に集約している
- 設定値を `domain/params` に切り出して、UIとロジックの結合を弱くしている
- `infrastructure` に具体実装を寄せて、将来的なテストや置き換えをしやすくしている

## 今後の方向性
この構成は、さらに次のような方向に発展させやすいです。

- 地図の状態管理を専用のStore/Controllerに切り出す
- 地図データソースを増やしてレイヤー管理を強化する
- `infrastructure` の責務をもっと明確に分割する
- UIとロジックの境界をさらに整理し、テストしやすい構造にする

## まとめ
DattoGISは、単なる地図表示アプリではなく、GISの基本要素であるレイヤー管理、座標計算、地図描画、検索、設定管理を整理して構築しているプロジェクトです。

今のディレクトリ構造は、以下の流れに沿って整理されています。

```text
main.rs
  -> apps/app.rs
      -> templates/main_window.rs
          -> organisms/main_window/*
              -> common/components
              -> common/services/map

domain: 設定・型・契約
infrastructure: 外部実装
```

この構造により、地図アプリとして必要なUI・ロジック・データアクセスを自然に分離しながら開発を進められるようになっています。

