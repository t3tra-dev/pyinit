[[英語/English](README.md)]

# pyinit

`pyinit`は、Pythonライブラリプロジェクトを迅速にセットアップするためのコマンドラインツールです。`README.md`、`LICENSE`、`setup.py`など、ライブラリ開発に必要なファイルを自動で生成し、開発者がプロジェクト構築にかかる手間を省きます。

## 特徴

- Pythonライブラリプロジェクトのディレクトリ構造を自動生成
- `README.md`、`LICENSE`、`__init__.py`、`setup.py`、`requirements.txt`などの重要ファイルを作成
- 柔軟なカスタムテンプレートのサポート
- Linux、macOS、Windowsをサポートするクロスプラットフォーム対応
- インタラクティブなプロンプトとコマンドライン引数での高速セットアップを提供

## インストール方法

### Linux

`pyinit`をLinuxにインストールするには、[リリースページ](https://github.com/t3tra-dev/pyinit/releases)から最新バイナリをダウンロードし、バイナリを`PATH`にあるディレクトリに移動します。

```bash
wget https://github.com/t3tra-dev/pyinit/releases/download/v0.1.0/pyinit-linux-latest-v0.1.0.zip
unzip pyinit-linux-latest-v0.1.0.zip
chmod +x pyinit
sudo mv pyinit /usr/local/bin/
```

`pyinit`を実行して使い始めます:

```bash
pyinit --help
```

### macOS

macOS向けに`pyinit`をインストールするには、[リリースページ](https://github.com/t3tra-dev/pyinit/releases)から最新バイナリをダウンロードし、バイナリを`PATH`にあるディレクトリに移動します。

```bash
curl -L -O https://github.com/t3tra-dev/pyinit/releases/download/v0.1.0/pyinit-macos-latest-v0.1.0.zip
unzip pyinit-macos-latest-v0.1.0.zip
chmod +x pyinit
sudo mv pyinit /usr/local/bin/
```

`pyinit`を実行して使い始めます:

```bash
pyinit --help
```

### Windows

Windowsで`pyinit`を使用するには、[リリースページ](https://github.com/t3tra-dev/pyinit/releases)から最新バイナリをダウンロードして解凍します。

```bash
Invoke-WebRequest -Uri https://github.com/t3tra-dev/pyinit/releases/download/v0.1.0/pyinit-windows-latest-v0.1.0.zip -OutFile pyinit.zip
Expand-Archive -Path pyinit.zip -DestinationPath .
```

以下の手順で`pyinit`を`PATH`に追加します:

1. 「このPC」を右クリックし、「プロパティ」を選択
2. 「システムの詳細設定」をクリックし、「環境変数」を選択
3. 「システム環境変数」の中から`Path`を探し、選択後「編集」をクリック
4. 「新規」をクリックし、`pyinit.exe`があるパスを追加
5. すべてのウィンドウで「OK」をクリックして閉じる

これでコマンドラインから`pyinit`が実行できるようになります:

```bash
pyinit --help
```

## ローカルでのビルド方法

`pyinit`をソースコードからローカル環境でビルドするには、Rustがインストールされている必要があります。

### 前提条件

- [Rust](https://www.rust-lang.org/tools/install)がシステムにインストールされていること

### ビルド・インストール手順

リポジトリをクローンして `cargo install --path .` を実行してください:

```bash
git clone https://github.com/t3tra-dev/pyinit.git
cd pyinit
cargo install --path .
```

これで、`pyinit`が実行できるようになります:

```bash
pyinit --help
```

## 使用方法

`pyinit`は、対話形式およびコマンドライン引数を使用して非対話形式で実行できます。

### コマンドライン引数

```bash
pyinit [OPTIONS]
```

#### オプション

- `--name`, `-n`: ライブラリの名前を指定
- `--description`, `-d`: ライブラリの説明を指定
- `--author`, `-a`: 作者の名前を指定
- `--license`, `-l`: ライセンスの種類を指定(MIT、GPL、Apache-2.0、など)
- `--help`, `-h`: CLIのヘルプ情報を表示(`-h`はサマリー)
- `--version`: CLIのバージョンを表示

#### 使用例

インタラクティブにPythonライブラリプロジェクトを作成するには、以下のコマンドを実行します:

```bash
pyinit
```

プロジェクト名や説明、作者、ライセンスを尋ねられるので、入力して進みます。非対話形式でプロジェクトを作成する場合は、コマンドライン引数を使用します:

```bash
pyinit --name mylib --description "A Python library for awesome features" --author "John Doe" --license MIT
```

これにより、以下のディレクトリ構造が生成されます:

```
mylib/
├── LICENSE
├── README.md
├── mylib/
│   └── __init__.py
├── requirements.txt
├── setup.py
└── tests/
```

### ライセンスの選択

ライセンスに`custom`を選んだ場合、カスタムライセンステキストを入力するか、空白のままにすることができます。それ以外の場合は、選択したライセンスに基づいて`LICENSE`ファイルが自動的に生成されます。

## コントリビューティングとサポート

`pyinit`への貢献は大歓迎です！以下の手順に従って貢献できます:

1. リポジトリをフォークします: [https://github.com/t3tra-dev/pyinit](https://github.com/t3tra-dev/pyinit)
2. フィーチャーブランチを作成します: `git checkout -b feature/your-feature`
3. 変更をコミットします: `git commit -m 'Add a new feature'`
4. ブランチをプッシュします: `git push origin feature/your-feature`
5. プルリクエストを送信します。

サポートや質問については、リポジトリの[Issuesセクション](https://github.com/t3tra-dev/pyinit/issues)に問題を報告してください。

全てのコントリビューターに感謝しています！

![pyinit Contributors](https://contrib.rocks/image?repo=t3tra-dev/pyinit)

---

`pyinit`はMITライセンスの下で提供されています。詳細については[LICENSE](https://github.com/kanarus/pyinit/blob/main/LICENSE)ファイルをご覧ください。
