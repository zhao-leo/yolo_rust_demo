@echo off

REM 检查参数数量
if "%~1"=="" (
    echo Usage: %0 number optional[output_file]
    exit /b 1
) 

REM 从命令行参数读取数字和文件名
set num=%1
if "%~2"=="" (
    set file="Null"
) else (
    set file=%2
)

REM 循环从1到输入的数字
for /L %%i in (1,1,%num%) do (
    if %file%=="Null" (
        call run.bat waste_detection.jit images/%%i.png types.json -o result/output%%i.jpg -c 0.7
    ) else (
        call run.bat waste_detection.jit images/%%i.png types.json -o result/output%%i.jpg -c 0.7 >> %file%
        type %file%
    )
)