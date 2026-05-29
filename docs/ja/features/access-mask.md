---
title: アクセスマスク
layout: doc
sidebar: true
search: true
access: masked
---

# アクセスマスク

このページには `access: masked` が設定されています。

オーバーレイはフロントエンドの表示マスクにすぎません。コンテンツを暗号化せず、静的 HTML からコンテンツを削除せず、サーバーサイド認証も提供しません。

## 動作確認

このサンプルサイトでは `[access].password` にデモ用パスワードを設定しています。正しいパスワードを入力すると、このブラウザーセッションでオーバーレイが非表示になります。

```toml
[access]
enabled = true
mode = "mask"
password = "rustpress"
password_hint = "Enter password"
```

`password` が設定されていない場合、`access: masked` ページにはオーバーレイは表示されません。

## 注意

アクセスマスクはフロントエンドのオーバーレイであり、セキュリティ保護ではありません。生成された HTML とフロントエンドスクリプトは直接閲覧できます。
