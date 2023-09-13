<!-- title: 使用手册 -->
<!-- date: 2023-09-13  -->
<!-- category: KB  -->
<!-- tag: Readme  -->

# mini-blog

一个非常简单的博客。

读取指定目录下的Markdown文件，形成目录，并展示为HTML页面。

- [x] 读取目录下的文件，形成目录页
- [x] 读取Markdown文件，展示为Html页面
- [ ] 标签云（Tag）
- [ ] 分类（Category）

## Markdown特殊格式

使用Markdown的注解，形成文件的元数据。

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

## 下载

从Github上可以下载本程序的最新版本。

## 执行

进入到解压目录中，执行如下命令：

```bash
./mini-blog
```

默认情况下，在3000端口下执行。