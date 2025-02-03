# scenario-text-length-checker

## 概要

scenario-text-length-checkerは、ティラノスクリプトのシナリオテキストの長さをチェックするためのツールです。指定された行数を超えるテキストを探し、ファイル名・行数とともに表示します。

## 特徴

- ファイルを置いて実行するだけで簡単に動作します。
- `setting.json`を変更することで1行ごとの文字数と行数を指定できます。

## インストール

1. [https://github.com/YoujinTea/scenario-text-length-checker/releases](https://github.com/YoujinTea/scenario-text-length-checker/releases)から最新版のzipファイルをダウンロードします。
2. ダウンロードしたzipファイルを展開し、展開したフォルダをティラノスクリプトで作成されたプロジェクトの`data/others`フォルダ内に配置します。

## 使い方

1. `scenario-text-length-checker.exe`を起動します。
2. `data/scenario`ファイル内のksファイルを全て読み、指定された行数を超えていないかを確認します。

## `setting.json`の各パラメータについて

```json
{
  "linefeed_tag": [
    "[r]"
  ],
  "page_break_tag": [
    "[p]"
  ],
  "line_count": 2,
  "max_row_length": 30
}
```

- `linefeed_tag`: 改行タグを指定します。複数指定する場合は以下のように変更してください。

    ```json
    {
      "linefeed_tag": [
        "[r]",
        "[lr]"
      ]
    }
    ```

- `page_break_tag`: 改ページタグを指定します。こちらも複数指定可能です。
- `line_count`: 1行あたりの最大文字数を指定します。
- `max_row_length`: 1つのテキストあたりの最大行数を指定します。

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。詳細については、`LICENSE`ファイルを参照してください。

## 貢献

バグ報告や機能提案は、[Issues](https://github.com/YoujinTea/scenario-text-length-checker/issues)ページで受け付けています。プルリクエストも歓迎します。
