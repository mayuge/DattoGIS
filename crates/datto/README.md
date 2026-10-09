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

## ベクターレイヤー
ベクターレイヤーは `assets/config/vector_config.json` で定義し、GeoJSONファイルを読み込んで地図上に表示します。現在、Point、LineString、Polygon、MultiPolygonに対応しています。Polygonの内側リングは穴として塗り抜かれます。

動作確認用のダミーデータは次の場所にあります。

- `assets/geo/polygon.geojson`: 通常のPolygon
- `assets/geo/multipolygon.geojson`: 穴を含むPolygonを含むMultiPolygon

レイヤーごとの `epsg` を使ってWeb Mercatorへ座標変換し、`opacity` と `style` を描画に適用します。

## ディレクトリ構成

```text
DattoGIS/
├── assets/                         # UI素材、設定、地理データ
│   ├── activity_bar/               # サイドバーのアイコン
│   ├── components/                 # チェックボックスなどの部品画像
│   ├── config/                     # ラスター/ベクターレイヤーの設定JSON
│   │   ├── raster_tile_config.json
│   │   └── vector_config.json
│   ├── geo/                        # GeoJSONのサンプル・表示データ
│   │   ├── airport.geojson
│   │   ├── multipolygon.geojson
│   │   ├── polygon.geojson
│   │   └── railway.geojson
│   ├── main_logo/                  # アプリのロゴ
│   ├── map/                        # 地図上の表示素材
│   └── window/                     # ウィンドウ用素材
├── src/
│   ├── main.rs                     # アプリケーションの起動
│   ├── apps/                       # GPUIによる画面・操作
│   │   ├── app.rs                  # ルート画面とウィンドウ初期化
│   │   ├── templates/
│   │   │   ├── main_window.rs      # 各画面機能をまとめるメイン画面
│   │   │   └── mod.rs
│   │   └── organisms/
│   │       ├── common/
│   │       │   ├── components/
│   │       │   │   ├── atoms/      # 単独で使えるUI部品
│   │       │   │   │   ├── checkbox.rs     # 表示状態の切り替え
│   │       │   │   │   ├── footer.rs       # フッター部品
│   │       │   │   │   ├── header.rs       # ヘッダー部品
│   │       │   │   │   ├── search_input.rs # 検索入力
│   │       │   │   │   └── slider.rs       # 不透明度入力
│   │       │   │   └── molecules/
│   │       │   │       └── layer_item.rs   # チェックとスライダーを持つレイヤー行
│   │       │   └── services/map/   # 表示部品から分離した地図計算
│   │       │       ├── map_render_service.rs # 表示タイル・形状とキャッシュキー
│   │       │       ├── map_state_service.rs  # 地図状態とレイヤー再読込
│   │       │       ├── use_map_area.rs       # 地図表示領域の計算
│   │       │       ├── use_map_bbox.rs       # 表示範囲の地理座標化
│   │       │       ├── use_map_event.rs      # ズーム・パン操作
│   │       │       ├── use_map_instance.rs   # MapInstanceの公開窓口
│   │       │       ├── use_map_tile.rs       # XYZタイルの座標とURL
│   │       │       ├── use_map_world_pixel.rs # 座標とワールドピクセルの変換
│   │       │       └── vector_layer_service.rs # WKB形状の投影
│   │       └── main_window/
│   │           ├── activity_bar_app.rs       # 左側の機能バー
│   │           ├── footer_app.rs             # 座標・ズーム等の表示
│   │           ├── layer_controller_app.rs   # レイヤー設定の編集・保存
│   │           ├── search_app.rs             # 地名・住所検索
│   │           └── map/
│   │               ├── map_app.rs          # 地図状態と操作の接続
│   │               ├── raster_tile_layer_app.rs # ラスタータイル描画
│   │               └── vector_layer_app.rs  # ベクター形状描画
│   ├── domain/                     # UI/外部実装に依存しない型と契約
│   │   ├── params/                 # アプリ・地図・デザイン等の定数
│   │   │   ├── api_config.rs       # API接続設定
│   │   │   ├── app_config.rs       # アプリ名・ウィンドウ設定
│   │   │   ├── design_token_config.rs # 色・余白・サイズ
│   │   │   ├── map_config.rs       # 投影・ズーム・タイル設定
│   │   │   └── text_config.rs      # UI表示文字列
│   │   ├── traits/                 # 外部実装に依存しない操作契約
│   │   │   ├── coordinate_transformer_trait.rs
│   │   │   ├── geocoding_trait.rs
│   │   │   ├── load_raster_tile_json_trait.rs
│   │   │   ├── load_vector_json_trait.rs
│   │   │   ├── map_area_trait.rs
│   │   │   ├── map_event_trait.rs
│   │   │   ├── map_tile_trait.rs
│   │   │   ├── vector_layer_service_trait.rs
│   │   │   ├── vector_repository_trait.rs
│   │   │   └── world_pixel_trait.rs
│   │   └── types/                  # 座標、地図状態、レイヤー、検索結果
│   │       ├── geocoding_type.rs
│   │       ├── geometry_cache_key_type.rs
│   │       ├── map_coordinate_type.rs
│   │       ├── map_instance_type.rs
│   │       └── map_layer_type.rs
│   └── infrastructure/             # 外部ライブラリを使う具体処理
│       ├── client/index.rs         # HTTPクライアント実装
│       ├── coordinate/
│       │   └── proj_core_coordinate_transformer.rs # PROJ座標変換
│       ├── duckdb/
│       │   └── vector_repository.rs # GeoJSON/WKB保存とBBox検索
│       ├── geocoding/index.rs      # ジオコーディングAPI呼び出し
│       └── json/
│           ├── load_raster_tile_json.rs # ラスター設定の読込・保存
│           └── load_vector_json.rs      # ベクター設定の読込・保存
├── Cargo.toml                      # Rust依存関係とパッケージ設定
├── Cargo.lock                      # 依存関係の固定バージョン
└── README.md
```

### 主な処理の流れ
1. `layer_controller_app.rs` がレイヤー設定を表示し、変更を設定JSONへ保存します。
2. `map_state_service.rs` と `vector_repository.rs` がGeoJSONを読み、WKBと空間範囲に変換してDuckDBへ格納します。
3. `vector_layer_service.rs` がWKBの形状を読み取り、EPSGから画面座標へ投影します。
4. `vector_layer_app.rs` が点・線・ポリゴンを描画します。ポリゴンはEvenOdd塗り規則で穴を表現します。
5. ラスタータイルは `map_render_service.rs` で可視範囲を計算し、`raster_tile_layer_app.rs` が設定順に描画します。

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

## ベクター形状の処理
GeoJSONの座標配列は `vector_repository.rs` でWKBへ変換し、型番号と座標数を保ったままDuckDBへ保存します。Polygonは外周と内側リングを別々に保持し、MultiPolygonは複数のPolygonへ分解して画面形状にします。

画面座標への変換では、各頂点を設定されたEPSGからWeb Mercatorへ変換した後、地図中心とズームに基づくピクセル座標へ変換します。穴はリングごとのサブパスをEvenOdd規則で塗ることで、リングの向きに依存せず塗り抜きます。

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

