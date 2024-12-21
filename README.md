# atcoder-rust

VSCodeを使用してRustでAtCoderの問題を解くための設定ファイルとテンプレート

## Requirement

- Visual Studio Code
  - Dev Containers
- Docker

## Usage

### 問題の準備

1. `/templates/a.rs`を`/src/bin/`にコピーする
2. 必要に応じてリネーム
3. コピーしたファイルの`case1()`にある`input`と`expected_output`にテストケースを書き込む
4. 同様に他のテストケースも作成する

### 解答の作成と実行

- `main()`で問題を解く
- VSCodeの「実行とデバッグ」から`Debug unit tests in active file`でテストできる
- `/input.txt`に入力を書き込んだあと、VSCodeの「実行とデバッグ」から`Debug executable active file with input.txt`でデバッグできる
