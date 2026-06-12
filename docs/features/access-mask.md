---
title: 访问遮罩
layout: doc
sidebar: true
search: true
access: masked
---

# 访问遮罩

这个页面使用了 `access: masked`，所以会显示 RustPress 的前端访问遮罩。

## 它能做什么

访问遮罩适合演示、轻量内部预览或避免页面被随手浏览。用户输入配置中的密码后，当前浏览器会话会记住解锁状态。

```toml
[access]
enabled = true
mode = "mask"
password = "rustpress"
password_hint = "Enter password"
```

页面 frontmatter：

```yaml
---
title: Private Preview
access: masked
---
```

## 它不能做什么

访问遮罩不是认证系统：

- 不加密 HTML。
- 不从 `dist/` 删除页面内容。
- 不阻止别人查看源文件或网络响应。
- 不替代服务端登录、VPN、反向代理认证或对象存储权限。

如果文档包含真正敏感内容，请使用带权限控制的托管方案。

## 启用条件

遮罩只有在这些条件同时满足时才显示：

1. `[access].enabled = true`
2. `[access].mode = "mask"`
3. `[access].password` 非空
4. 页面设置 `access: masked`

任何条件不满足，页面都会按普通公开页面渲染。

## 解锁行为

前端脚本会把当前路径的解锁状态保存到 `sessionStorage`。这意味着：

- 刷新同一页面仍保持解锁。
- 关闭浏览器会话后需要重新输入。
- 不同路径的页面分别记录。

## 搜索关系

`access: masked` 不会自动排除搜索。如果不希望遮罩页出现在搜索中，请同时设置：

```yaml
search: false
```
