# yew-timer
yewでタイマーを作ってみる

##　コマンドのメモ

### プロジェクト作成 
```sh
$ cargo new --lib yew-timer && cd yew-timer
``` 

### wasmビルド
```sh
$ wasm-pack build --target web --out-name wasm --out-dir ./static
```

### ローカルサーバ起動
```sh
$ miniserve ./static --index index.html
```

### toolchainでnightlyを使えているかどうかの確認
```sh
$ rustup show
```
