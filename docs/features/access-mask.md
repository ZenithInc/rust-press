---
title: 访问遮罩
layout: doc
sidebar: true
search: true
access: masked
---

# 访问遮罩

此页面使用 `access: masked` 标记。

遮罩只是前端查看遮挡。它不会加密内容，不会从静态 HTML 中移除内容，也不提供服务端身份认证。

## 测试行为

在密码字段输入任意文本，即可在当前浏览器会话中隐藏遮罩。

## 说明

访问遮罩只是前端界面遮挡，不是安全保护。构建产物中的 HTML 仍然可以直接查看。
