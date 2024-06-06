﻿| 条目                               | 惯例                                                         |
| ---------------------------------- | ------------------------------------------------------------ |
| 包 Crates                          | [unclear](https://github.com/rust-lang/api-guidelines/issues/29) |
| 模块 Modules                       | `snake_case`                                                 |
| 类型 Types                         | `UpperCamelCase`                                             |
| 特征 Traits                        | `UpperCamelCase`                                             |
| 枚举 Enumerations                  | `UpperCamelCase`                                             |
| 结构体 Structs                     | `UpperCamelCase`                                             |
| 函数 Functions                     | `snake_case`                                                 |
| 方法 Methods                       | `snake_case`                                                 |
| 通用构造器 General constructors    | `new` or `with_more_details`                                 |
| 转换构造器 Conversion constructors | `from_some_other_type`                                       |
| 宏 Macros                          | `snake_case!`                                                |
| 局部变量 Local variables           | `snake_case`                                                 |
| 静态类型 Statics                   | `SCREAMING_SNAKE_CASE`                                       |
| 常量 Constants                     | `SCREAMING_SNAKE_CASE`                                       |
| 类型参数 Type parameters           | `UpperCamelCase`，通常使用一个大写字母: `T`                  |
| 生命周期 Lifetimes                 | 通常使用小写字母: `'a`，`'de`，`'src`                        |
| Features                           | [unclear](https://github.com/rust-lang/api-guidelines/issues/101) but see [C-FEATURE](https://course.rs/practice/naming.html#c-feature) |
