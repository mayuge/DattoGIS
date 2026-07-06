## dattogis

## 実行方法
- 1.Rustの実行環境を整える
  - https://qiita.com/y-428/items/4fcad7e73b061d9154f8
- 2.source/datto_gisに移動した後、cargo run

```
cd source/datto_gis
```
```
cargo run
```

## ディレクトリ構成
```
docs #ドキュメント
proto #プロトタイプ
source #本番候補のコード
```

```
.
├── main.rs #エントリポイント
├── components
│   ├── atoms
│   │   └── button.rs
│   └── molecules
├── apps
│   ├── app.rs #アプリケーションのエントリポイント
│   ├── organisms
│   │   └── main
│   └── templates
│       └── main_window.rs
├── assets
├── domain
│   ├── app_config.rs #アプリ初期設定
│   ├── design_token_config.rs #style初期設定
│   └── map_config.rs #地図系初期設定
├── services
│   └── map
│       ├── use_map_history.rs
│       ├── use_map_instance.rs
│       ├── use_map_tile.rs
│       └── use_map_world_pixel
└── infrastructure
    └── http
        └── http_request_client
```

