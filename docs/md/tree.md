2026/06/09 計画段階 part2

```
.
├── main.rs
├── components/
│   ├── atoms/
│   │   └── button.rs
│   └── molecules/
│       └── search.rs
├── apps/
│   ├── MapApp/
│   │   ├── core/
│   │   │   └── useMapApp.rs
│   │   └── ui/
│   │       └── mapApp.rs
│   └── dialogs/
│       └── SettingDialog/
│           ├── core
│           └── ui
├── infrastructure/
│   └── stores
└── domain/
    ├── traits
    └── params
```

2026/06/09 計画段階

```
.
├── main.rs
├── presentation/
│   ├── molecules
│   └── organisms/
│       └── [機能名]App/
│           ├── core/
│           │   └── use[機能名]App
│           └── ui/
│               └── [機能名]App
├── infrastructure/
│   └── [ライブラリの役割]
└── domain/
    ├── ports
    ├── params
    └── types
```
