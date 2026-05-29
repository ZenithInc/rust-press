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

当前示例站点在 `[access].password` 中配置了演示密码。输入正确密码后，遮罩会在当前浏览器会话中隐藏。

```toml
[access]
enabled = true
mode = "mask"
password = "rustpress"
password_hint = "Enter password"
```

如果没有配置 `password`，`access: masked` 页面不会显示遮罩。

## 说明

访问遮罩只是前端界面遮挡，不是安全保护。构建产物中的 HTML 和前端脚本仍然可以直接查看。
