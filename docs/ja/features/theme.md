---
title: テーマ
layout: doc
sidebar: true
search: true
access: public
---

# テーマ

デフォルトテーマは、静的 HTML、CSS、小さな JavaScript runtime で構成されています。

## カラーモード

テーマには 2 つの組み込みカラーモードがあります。

- `light`
- `dark`

`allow_switch = true` の場合、トップバーに Light/Dark 切り替えが表示され、選択したモードが `localStorage` に保存されます。

## GitHub リンク

`[theme]` に `github_url` を設定すると、トップバー右側に GitHub アイコンが表示されます。アイコンをクリックすると、設定したリポジトリを開きます。

```toml
[theme]
github_url = "https://github.com/your-org/your-repo"
```

## レイアウト

生成されるページには次のものが含まれます。

- sticky トップナビゲーション
- サイドバーナビゲーション
- レスポンシブなモバイルメニュー
- レベル 2 とレベル 3 の見出し用目次
- ローカル検索ダイアログ

テーマはフロントエンドマスクをセキュリティとして説明しません。
