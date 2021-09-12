# yew-timer
yewでタイマーを作ってみる

## コマンドのメモ

### プロジェクト作成 
```sh
$ cargo new --lib yew-timer && cd yew-timer
``` 

### wasmビルド
```sh
$ wasm-pack build --target web --out-name wasm --out-dir ./static/wasm
```

### ローカルサーバ起動
```sh
$ miniserve ./static --index index.html
```

### toolchainでnightlyを使えているかどうかの確認
```sh
$ rustup show
```

## Tips
### wasmでブラウザのコンソールでランタイムエラーになったとき
ランタイムエラーは表示されるが、詳細からどこに問題が起きているか追えない。
[console_error_panic_hook](https://github.com/rustwasm/console_error_panic_hook) を使うと詳細が見えるようになる

### chronoを使うとwasmで現在時刻表示でランタイムエラーになる
以下にもそれっぽいこと書いてある。
https://stackoverflow.com/questions/63210984/chrono-kills-my-rust-webassembly-function

chronoを使うのをやめて、[js-sys](https://crates.io/crates/js-sys) で現在時刻を取得するようにして回避した

### Stringを&'static strに変換する
以下を見て解決。  
いっつも引っかかってる気がする。
https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str
