# yolov8s 在rust上的推理

### 从源码构建

`torch-rs`打包时依赖于`libtorch`，如果你想自己构建程序，要先添加以下两个环境变量
```
LIBTORCH="\path\to\libtorch"
PATH="\path\to\libtorch\lib"
```
然后直接`cargo build`或者`cargo run`正常运行即可

从源码构建推荐仅打包`yolo_cli`

### 使用`exe`文件(新版)
如果你只是想要使用该程序，你需要按以下方式组织文件：
```
path:/
│  run.bat
│
└─core
       yolo_binding.exe
       yolo_shell.exe
```
输入`run`会有相关提示。


### ChangeLog v2.0.0
在最新的版本，我们支持了批量推理，同时将原来的`binding`模块进行拆分，将`binding`模块发布在了[crates.io](https://crates.io/crates/yolo_binding)上

在这个仓库中，我们仅仅提供命令行实现，以实现模块化调用

>PS：
>
>为了实现LIBTORCH环境变量的注入，我们通过yolo_shell执行yolo_cli主程序，yolo_shell会对所有输入参数进行转发，并将返回的字节流转发到主程序的std上
>
>一般来说，我有依赖强迫症，会更新tch-rs库，所以记得选择合适的libtorch版本
>
>我暂时不知道将整个`dll`库打包进主程序的方法，因此编写了`yolo_shell`对主程序进行环境配置和注入，如果你知道怎么把`dll`附加进主程序，一定要给我pr！！！万分感谢！
