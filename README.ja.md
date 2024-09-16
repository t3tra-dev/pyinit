[[英語/English](README.md)]

# pyinit

`pyinit`は、Pythonライブラリプロジェクトの雛形を作成するためのコマンドラインツールです。新しいPythonライブラリを素早くセットアップするための基本的なファイルと構造化されたディレクトリレイアウトを提供します。

## インストール方法

### macOS、Linux、およびWindows

リポジトリの[リリースページ](https://github.com/t3tra-dev/pyinit/releases)から、macOS、Linux、Windows向けのビルド済みバイナリをダウンロードできます。

1. **該当するバイナリをダウンロード**します。
2. **バイナリを解凍**します（圧縮形式の場合）。
3. **バイナリを移動**します。macOSとLinuxでは`/usr/local/bin`、Windowsでは`C:\Program Files`などのPATHに含まれるディレクトリに移動します。

### ローカルビルド

ソースから`pyinit`をローカルでビルドするには、以下の手順に従ってください。

1. **リポジトリをクローン**します:

    ```bash
    git clone https://github.com/t3tra-dev/pyinit.git
    cd pyinit
    ```

2. **Rustツールチェインをインストール**します。未インストールの場合は、[https://rustup.rs](https://rustup.rs) からRustをインストールしてください。

3. **バイナリをビルド**します:

    - macOSとLinux:

        ```bash
        cargo build --release
        ```

    - Windows:

        ```bash
        cargo build --release --target x86_64-pc-windows-msvc
        ```

4. **バイナリを探します**。`target/release`ディレクトリ（Windowsの場合は`target\x86_64-pc-windows-msvc\release`）内にあります。

5. **バイナリを移動**します。システムのPATHに含まれるディレクトリに移動します。

## 使い方

`pyinit`を使用するには、以下のコマンドを実行します:

```bash
pyinit
```

### コマンドライン引数

- `--name`, `-n`: Pythonライブラリの名前。（対話的に指定しない場合は必須）
- `--description`, `-d`: ライブラリの簡単な説明。（オプション、空白でも可）
- `--author`, `-a`: 作者の名前。（対話的に指定しない場合は必須）
- `--license`, `-l`: ライセンスの種類。（対話的に指定しない場合は必須。`templates/licenses/`内のオプションが使用できます）

### 例

1. **対話モード:**

    単に`pyinit`を実行し、プロンプトに従って対話的に新しいPythonライブラリプロジェクトをセットアップします。

2. **対話的でないモード（すべての必須引数を指定する場合）:**

    ```bash
    pyinit --name my_library --author JohnDoe --license MIT
    ```

3. **対話的でないモード（オプションの引数も指定する場合）:**

    ```bash
    pyinit --name my_library --description "サンプルPythonライブラリ" --author JohnDoe --license MIT
    ```

## ライセンス

`pyinit`はMITライセンスの条件の下でライセンスされています。詳細については、[LICENSE](LICENSE)ファイルをご覧ください。

## 貢献

貢献は歓迎します！提案や改善がある場合は、[GitHub](https://github.com/t3tra-dev/pyinit/issues)で問題を開くか、プルリクエストを送信してください。

## サポート

サポートが必要な場合は、[GitHub](https://github.com/t3tra-dev/pyinit/issues)で問題を開いてください。
