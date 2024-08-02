<!-- title: 使用手册 -->
<!-- date: 2023-09-13  -->
<!-- category: KB  -->
<!-- tag: Readme  -->

# 简介

一个非常简单的博客。读取指定目录下的Markdown文件，形成目录，并展示为HTML页面。

你可以向``/blog``目录下添加新Markdown文件来新增博客。
无需重启服务，刷新页面即可查看新增的博客。

# 开发计划

- [x] 在博客文章后面增加了作者信息，可在配置文件中录入作者信息。 [add 0.4.0]
- [x] 读取目录下的文件，形成目录页
- [x] 读取Markdown文件，展示为Html页面
- [x] 标签云（Tag）,同时支持按照指定标签进行过滤
- [x] 分类（Category）,同时支持按照指定分类进行过滤
- [x] 博客列表页面的博客按照时间进行了排序
- [x] 当新增Markdown文件时，会触发整体文章元数据刷新。系统是通过文件的created属性来判定是否需要重新加载文章元数据。

# 使用帮助

## 下载

从Github上可以下载本程序的最新版本。解压后进入到解压目录，文件结构如下：

```text
\mini-blog
|--\blog             //存储Markdown文件的目录
|--\static           //存储静态文件的目录，例如：css、js、templates等
|  |--\css           //样式文件
|  |--\templates     //模板
|  |--\images        //图像，文章中的图像存储在此
|  |--avator.png     //图像，作者的大头像
|--mini-blog         //可执行的程序
|--config.yml        //配置文件
```

## 配置信息

```yml
title: 博客标题
organization: 组织名称 2023-2024
record-number: 网站备案号
author:
  title: Aout Me
  description:
    - Description 1
    - Description 2
    - Description 3

app-port: 3000
log-level: info
```

## 执行

进入到解压目录中，执行如下命令：

```bash
./mini-blog
```

默认情况下，在浏览器中通过地址[http://localhost:3000](http://localhost:3000)访问。


## Markdown特殊格式

在编写博客时，使用的是Markdown语法。本程序利用Markdown的注解语法来存储博客的元数据。

如下所示元数据注解，写在文件的起始位置：

```md
<!-- title: 使用手册 -->
<!-- date: 2023-09-13  -->
<!-- category: KB  -->
<!-- tag: Readme  -->
```
* title: 博客的标题。
* date: 博客的编写日期。
* category: 分类。可以多个，使用逗号(,)分隔。
* tag: 标签。可以多个，使用逗号(,)分隔。
