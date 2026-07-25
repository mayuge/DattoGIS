```
cargo doc --open
```

## 開発画面を起動する際の手順
```
cargo run
```


## ビルドする際の手順

- https://github.com/zed-industries/zed/issues/46263

```
$env:GPUI_FXC_PATH="C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64\fxc.exe"
>> 
>> cargo build --release
```