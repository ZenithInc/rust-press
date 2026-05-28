---
title: Crates
layout: doc
sidebar: true
search: true
access: public
---

# Crates

ワークスペースは小さな crate に分割されています。

## rustpress-cli

コマンドライン引数を解析し、コマンドをディスパッチします。

## rustpress-core

設定を読み込み、ソース Markdown をスキャンし、レンダリングを調整し、public アセットをコピーし、検索アセットを書き込みます。

## rustpress-md

frontmatter を解析し、Markdown をレンダリングし、見出しアンカーを生成し、検索テキストを抽出します。

## rustpress-theme

デフォルト HTML をレンダリングし、CSS と JavaScript runtime アセットを書き込みます。

## rustpress-search

ローカル検索インデックスを構築し、tokenization helpers を公開します。

## rustpress-dev

静的ファイルを配信し、ソースファイルを監視し、変更時に再ビルドし、開発モードで更新スクリプトを注入します。
