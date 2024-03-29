---
title: Compile Command
---

# The `compile` sub-command

Compile your Pascal program to Javascript code which can be executed in web browsers.

## Arguments

|  Name  |  Type  |  Default Value  |                        Description                        |
| :----: | :----: | :-------------: | :-------------------------------------------------------: |
| input  | string | ./sanpasjs.pas  |          Pascal file path needed to be compiled           |
| output | string | ./dist/index.js | Javascript file path where compiled file should be placed |

## Sample

```shell
sanpasjs new --input <PASCAL_INPUT_FILE_PATH> --output <JS_OUTPUT_FILE_PATH>
```

## Description

The following code will help to transpile your Pascal program to a Javascript program.
Currently, this command is configured for ease of use of [`sanpasjs new`](/guides/commands/new.md) command.
For example, if you created the project using [`sanpasjs new`](/guides/commands/new.md) command you just need to run it (no need to specify other options).

```shell
sanpasjs compile
```

Before running the above code you need to make sure that you are in the correct folder.
![File explorer](../../public/CommandUsage/Sanjaiyan_Copy_File_Path_Compile.png)
![File explorer](../../public/CommandUsage/Sanjaiyan_Compile_Command_CMD.png)

If you are not using [`sanpasjs new`](/guides/commands/new.md) command, you need to specify file paths to run the command [as shown in the sample](#sample).

## Example

```shell
sanpasjs new --input input_pascal_file.pas --output output_js_file.js
```

<style>
    * {
        scroll-behavior: smooth;
    }
</style>
