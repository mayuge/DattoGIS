```
docs #ドキュメント
proto #プロトタイプ
source #本番候補のコード
```

```
.
├── main.rs #エントリポイント
├── components/
│   ├── atoms/
│   │   └── button.rs
│   └── molecules
├── apps/
│   ├── app.rs #アプリケーションのエントリポイント
│   ├── organisms/
│   │   └── main
│   └── templates/
│       └── main_window.rs
├── assets
├── domain/
│   ├── app_config.rs #アプリ初期設定
│   ├── design_token_config.rs #style初期設定
│   └── map_config.rs　#地図系初期設定
└── services/
    └── map/
        ├── use_map_history.rs　#戻る進むボタンアーキテクチャ構想
        └── use_map_instance.rs #地図ライブラリ構築部分
```