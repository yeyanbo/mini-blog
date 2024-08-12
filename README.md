<!-- title: README -->
<!-- date: 2023-09-13  -->
<!-- category: KB  -->
<!-- tag: Readme  -->

# Introduction

A very simple blog. Read the Markdown files in the specified directory as article list, and display it as an HTML page.

You can add a new Markdown file to the ``/blog`` directory to add a new blog.
No need to restart the server, refresh the page to view the newly added blog.

[中文版本](README.zh-CN.md)

# 开发计划
- [ ] Add Code Copy for Code Block  [add 0.4.7]
- [ ] Add Line Number for Code Block [add 0.4.7]
- [ ] Add Toc for Blog Article  [add 0.4.7]
- [ ] Add Section Number for Heading  [add 0.4.7]
- [x] Integrate `highlightjs` code block syntax highlighting. [add 0.4.4]
- [x] Display author information after the blog article, which can be configured in the configuration file. [add 0.4.0]
- [x] A blog home page by read markdown files in the specified directory 
- [x] Read markdown files and display them as Html pages
- [x] Tag cloud and support filtering by specified tags
- [x] Category and support filtering by specified categories
- [x] Blogs on the blog list page are sorted by time
- [x] When a new markdown file is added, it will trigger a refresh of the overall article metadata. The system determines whether the article metadata needs to be reloaded based on the file's created attribute.

# Usage

## Download

You can download the latest version of this program from Github. After unzip, enter the decompression directory. The file structure is as follows:

```text
\mini-blog
|--\blog             //Directory where storage Markdown files
|--\static           //Directory where static files are stored, such as css and js.
|  |--\css           //Style files
|  |--\js            //Script files
|  |--\templates     //Templates
|  |--\images        //Images in the article are stored here
|  |--avator.png     //Author avator image
|--mini-blog         //Execute file
|--config.yml        //Configuration file
```

## Configuration

```yml
title: Blog Title
organization: Organization 2023-2024
record-number: Record Number
author:
  title: Aout Me
  description:
    - Description 1
    - Description 2
    - Description 3

app-port: 3000
log-level: info
```

## Run

Enter the decompressed directory and execute the following command:

```bash
nohup ./mini-blog > mini-blog.log &
```
By default, it is accessed through [http://localhost:3000](http://localhost:3000) in the browser.

After redeployment, execute again:

```bash
// Find the process with port number 3000
lsof -i:3000
// Kill the found thread by process pid
kill -9 [pid]
// Restart
nohup ./mini-blog > mini-blog.log &

```

## Markdown Metadata

> Plan to refactor blog metadata parsing

When writing a blog, Markdown syntax is used. This program uses Markdown's annotation syntax to store the blog's metadata.

The metadata annotation shown below is written at the beginning of the file:

```md
<!-- title: README -->
<!-- date: 2023-09-13  -->
<!-- category: KB  -->
<!-- tag: Readme  -->
```

* title: the title of the blog.
* date: the date the blog was written.
* category: category. Multiple categories are allowed, separated by commas (,).
* tag: tag. Multiple categories are allowed, separated by commas (,).