# yolov8s 在rust上的推理
### 从源码构建
`torch-rs`打包时依赖于`libtorch`，如果你想自己构建程序，要先添加以下两个环境变量
```
LIBTORCH="\path\to\libtorch"
PATH="\path\to\libtorch\lib"
```
然后直接按照`cargo build`或者`cargo run`正常运行即可
### 使用`exe`文件
如果你只是想要使用该程序，你需要按以下方式组织文件：
```
path:/
    run.bat
    yolo_binding.exe
    yolo_shell.exe
```
在当前目录启动命令行并执行以下命令：
```
.\run path/to/model.jit path/to/picture path/to/types.json
```
```
PS：
为了实现LIBTORCH环境变量的注入，我们通过yolo_shell执行yolo_binding主程序，yolo_shell会对所有输入参数进行转发，并将返回的字节流转发到主程序的std上
```
暂时没有将整个`dll`库打包进主程序的方法，因此编写了`yolo_shell`对主程序进行环境配置和注入，如果你知道怎么把`dll`附加进主程序，一定要给我pr！！！万分感谢！
