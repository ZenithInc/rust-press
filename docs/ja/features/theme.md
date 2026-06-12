---
title: テーマ
layout: doc
sidebar: true
search: true
access: public
---

# テーマ

RustPress は組み込みテーマを提供します。HTML、CSS、小さな JavaScript 実行時スクリプトだけで動作し、フロントエンドのビルドは不要です。

## レイアウト

- sticky トップバー
- トップドロップダウン
- 左サイドバー
- 本文
- H2/H3 目次
- 言語切替
- 検索ダイアログ
- Light/Dark 切替
- GitHub リンク
- Markdown コピー menu

## ナビゲーション

`top_nav` はトップバー、`sidebars` は左サイドバーを制御します。`sidebar = "guide"` は関連付けだけで、`top_nav.items` をサイドバーにコピーしません。

## 色モード

```toml
[theme]
skin = "light"
allow_switch = true
```

`light` と `dark` をサポートします。切替結果は `localStorage` に保存されます。

## GitHub リンク

```toml
[theme]
github_url = "https://github.com/your-org/your-repo"
```

空文字ならアイコンは表示されません。

## 検索とコピー

検索が有効な場合、トップバーに検索ボタンが表示されます。`Shift` を 2 回押しても開けます。コードコピーと Markdown コピーもテーマが提供します。

## アクセスマスク

`access: masked` ページにはフロントエンドのマスクパネルが表示されます。これは閲覧用の遮蔽であり、セキュリティではありません。
