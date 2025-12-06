# Tauri Plugin bliet

this is the structure:

├── android
│   ├── build.gradle.kts
│   ├── proguard-rules.pro
│   ├── settings.gradle
│   └── src
│       ├── androidTest
│       │   └── java
│       │       └── ExampleInstrumentedTest.kt
│       ├── main
│       │   ├── AndroidManifest.xml
│       │   └── java
│       │       ├── Example.kt
│       │       └── ExamplePlugin.kt
│       └── test
│           └── java
│               └── ExampleUnitTest.kt
├── build.rs
├── Cargo.toml
├── permissions
│   └── default.toml
├── README.md
└── src
    ├── commands.rs
    ├── desktop.rs
    ├── error.rs
    ├── lib.rs
    ├── mobile.rs
    └── models.rs