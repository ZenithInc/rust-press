---
title: アクセスマスク
layout: doc
sidebar: true
search: true
access: masked
---

# アクセスマスク

このページは `access: masked` を使っているため、フロントエンドのアクセスマスクが表示されます。

## できること

デモ、軽量な内部プレビュー、気軽な閲覧の抑制に使えます。正しいパスワードを入力すると、現在のブラウザセッションで解除状態が記録されます。

```toml
[access]
enabled = true
mode = "mask"
password = "rustpress"
password_hint = "Enter password"
```

```yaml
---
title: Private Preview
access: masked
---
```

## できないこと

これは認証ではありません。

- HTML を暗号化しません。
- `dist/` から内容を削除しません。
- ソースやネットワーク response の閲覧を防ぎません。
- サーバー側ログインや VPN の代わりにはなりません。

機密情報にはホスティング側のアクセス制御を使ってください。

## 表示条件

1. `[access].enabled = true`
2. `[access].mode = "mask"`
3. `[access].password` が空でない
4. ページが `access: masked`

## 検索

`access: masked` は検索除外ではありません。必要なら `search: false` も指定します。
