# git_webhooks


基于github、bitbucket、coding、gitee的自动化集成部署已经有很多了，但是试用了好几个，对自己目前都需求来说还是有一点复杂。我的理解里面webhooks是一个非常简单的操作，就是一个push代码，然后对应版本，触发服务器的pull操作，然后执行一系列相关命令。

我想要的比较懒，首先一个服务器会部署很多的项目，

* 服务器上只需要运行一个服务
* 指定分支才会触发，或者不同的分支执行不同的命令
* 对于前端来说，自动pull了更新即可
* 对于后端，需要更新，然后执行额外的命令、重启服务等
* 有安全保证


配置文件:
```.language-json5
{
  secret: "xxxxxx", // 全局密钥
  command: "git pull", 
  inherit: true, // 是否让子项目继承全局属性, 如果为false,  则repos里面的项目就不会继承全局属性
  repos: {
    // repository name 作为key
    "hello_world": {
      cwd: "/home/hello_world",
      command: "/code/xxx", // 要执行的命令，可以是一个字符串，也可以是一个数组
      branch: "master", // 可以指定分支才pull，如果不设置，默认所有分支都会执行
      secret: "fjdksalfjdsalfjda"
    },
    "hahaha": {
      cwd: "/home/hello_world",
      command: [
        "git pull",
        "cargo build",
      ],
      branch: "*"
    }
  }
}
```

如果所有项目都使用一个私钥，那么就在github webhooks中都url中配置参数就可以了，例如，config.json5文件可以这么配置：
```.language-json5
{
  secret: "xxxxxx", // 全局密钥
  command: "git pull", 
  inherit: true, // 是否让子项目继承全局属性, 如果为false,  则repos里面的项目就不会继承全局属性
}
```

webhooks 请求url里面可以增加参数：

http://xxxx.com/webhooks/git?cwd=/www/respository_path&command=git+pull
