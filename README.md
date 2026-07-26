<img src="https://capsule-render.vercel.app/api?type=waving&color=auto&height=300&section=header&text=datto_gis&fontSize=90&animation=fadeIn&fontAlignY=38&descAlignY=51&descAlign=62"/>

## gisとは
- GIS（Geographic Information System：地理情報システム）とは、位置や空間に関する情報（空間データ）をコンピュータ上のデジタル地図と結びつけ、データの管理、分析、視覚化を行うシステム

## datto_gis
- datto_gisの由来
  - 「脱兎の如く」の慣用句のように、速く、きびきびと動作すること
  -  他のGISソフトを打倒するようなものになってほしい
  -  ロゴはウサギをかたどったもの。rust製なのrustのRとrabbitのR同じだよね!

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
### source/datto_gisのディレクトリ構成
```
.
├── main.rs #エントリポイント
├── components #共通UIコンポーネント
│   ├── atoms
│   │   └── header.rs #共通ヘッダー
│   └── molecules
├── apps 
│   ├── app.rs #アプリケーションのエントリポイント
│   ├── templates #すべての部品を組み合わせる場所
│   │   └── main_window #画面名
│   └── organisms #機能ごとに部品を分けてコードを整理する（見た目のみ）
│       └── main_window
│           ├── activity_bar_app #個別機能呼び出し アクティビティバー　左アイコン群
│           ├── layer_controller_app #レイヤー透明度・表示切替　レイヤーコントローラ　左レイヤーパネル
│           └── map
│               ├── map_app.rs #地図描画部分 イベント、状態管理、ラスタータイル、ベクターレイヤー配置
│               ├── raster_tile_layer_app.rs #ラスタータイル描画機能
│               └── vector_layer_app #ベクター描画機能
├── domain #定義
│   ├── params #パラメータ定義
│   │   ├── design_token_config.rs #デザイントークン定義
│   │   ├── map_config.rs #地図関連定義
│   │   └── app_config.rs #基本要素定義
│   ├── traits #serviceやinfrastructureの関数定義
│   └── types #型定義
├── infrastructure #ライブラリを使った機能を集約
└── services #独自に作成した計算・純粋なロジックを集約
    └── map
        ├── use_map_area.rs #地図描画エリア計算
        ├── use_map_event.rs #イベント受け取り後の動作計算
        ├── use_map_instance.rs #座標・ズームレベル管理
        ├── use_map_tile.rs #地図タイルのURL割り出し
        └── use_map_world_pixel #座標をピクセル単位に変換するロジック
```

