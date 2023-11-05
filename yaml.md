# YAML语法介绍

## 一、为什么学习它？
在数据格式描述和较复杂数据内容展示方面的配置文件，JSON能够很好的支持，包括语法突出显示、自动格式化、验证工具等。然而缺乏注释，过于严格，长字符串转换会出现问题等等。对于运维人员，面对较复杂的数据结构来说，不得不寻找一个替代的方式。
YAML（YAML 不是标记语言）是一种非常灵活的格式，几乎是 JSON 的超集，已经被用在一些著名的项目中，如 Travis CI、Circle CI 和 AWS CloudFormation。YAML 的库几乎和 JSON 一样无处不在。除了支持注释、换行符分隔、多行字符串、裸字符串和更灵活的类型系统之外，YAML 也支持引用文件，以避免重复代码。

## 二、简介
`YAML`语言（发音 /ˈjæməl/）的设计目标，就是方便人类读写。它实质上是一种通用的数据串行化格式。
`YAML`有一个小的怪癖。所有的`YAML`文件开始行都应该是`---`。这是`YAML`格式的一部分, 表明一个文件的开始。

它的基本语法规则如下:
+ 大小写敏感
+ 使用缩进表示层级关系
+ 缩进时不允许使用Tab键，只允许使用空格。
+ 缩进的空格数目不重要，只要相同层级的元素左侧对齐即可

\# 表示注释，从这个字符一直到行尾，都会被解析器忽略。

`YAML`支持的数据结构有三种。
+ 对象：键值对的集合，又称为映射（mapping）/ 哈希（hashes） / 字典（dictionary）
+  数组：一组按次序排列的值，又称为序列（sequence） / 列表（list）
+ 纯量（scalars）：单个的、不可再分的值

下面对这三种数据结构做详细介绍：

### 三、对象
使用冒号代表，格式为`key: value`。冒号后面要加一个空格：
``` Yaml
---
#即表示url属性值；
url: https://www.liuluanyi.cn 
```
转为`JavaScript`如下:
``` JavaScript
{ url: 'https://www.liuluanyi.cn'}
```
`Yaml`也允许另一种写法，将所有键值对写成一个行内对象。
``` Yaml
--- 
host: { ip: 10.1.1.1, port: 2222 } 
```
转为`JavaScript`如下:

``` JavaScript
{ host: { ip: '10.1.1.1', port: 2222 } }
```

### 四、数组
列表中的所有成员都开始于相同的缩进级别, 并且使用一个`---`作为开头(一个横杠和一个空格):
``` Yaml
---
ipaddr:
# IP地址列表
- 120.168.117.21
- 120.168.117.22
- 120.168.117.23
```
转为`JavaScript`如下:
``` JavaScript
ipaddr: [ '120.168.117.21', '120.168.117.22', '120.168.117.23' ]
```
数据结构的子成员是一个数组，则可以在该项下面缩进一个空格。
``` Yaml
-
  - source
  - destination
  - services
```
转为`JavaScript`如下:
```JavaScript
[ [ 'source', 'destination', 'services' ] ]
```
数组也可以采用行内(或者流式)表示法。
``` Yaml
services: [FTP, SSH]
companies: [{id: 1,name: company1,price: 200W},{id: 2,name: company2,price: 500W}]
```
转为`JavaScript`如下:
``` JavaScript
{ services: [ 'FTP', 'SSH' ] }
{ companies: 
   [ { id: 1, name: 'company1', price: '200W' },
     { id: 2, name: 'company2', price: '500W' } ] }
```
对象和数组复合使用
``` Yaml
languages:
    - Ruby
    - Perl
    - Python 
websites:
    YAML: yaml.org 
    Ruby: ruby-lang.org 
    Python: python.org 
```
转为`JavaScript`如下:
``` JavaScript
{ languages: [ 'Ruby', 'Perl', 'Python' ],
  websites: { YAML: 'yaml.org', Ruby: 'ruby-lang.org', Python: 'python.org' } }
```
### 常量
+ 字符串
+ 布尔值
+ 整数
+ 浮点数
+ Null
+ 时间
+ 日期
下面使用一个例子来快速了解常量的基本使用：
``` Yaml
boolean: 
    - TRUE  #true,True都可以
    - FALSE  #false，False都可以
float:
    - 3.14
    - 6.8523015e+5  #可以使用科学计数法
int:
    - 123
    - 0b1010_0111_0100_1010_1110    #二进制表示
null:
    nodeName: 'node'
    parent: ~  #使用~表示null
string:
    - 哈哈
    - 'Hello world'  #可以使用双引号或者单引号包裹特殊字符
    - newline
      newline2    #字符串可以拆成多行，每一行会被转化成一个空格
date:
    - 2018-02-17    #日期必须使用ISO 8601格式，即yyyy-MM-dd
datetime: 
    -  2018-02-17T15:02:31+08:00    
    #时间使用ISO 8601格式，时间和日期之间使用T连接，最后使用+代表时区
```
转为`JavaScript`如下:
``` JavaScipt
{ boolean: [ true, false ],
  float: [ 3.14, 685230.15 ],
  int: [ 123, 685230 ],
  null: { nodeName: 'node', parent: null },
  string: [ '哈哈', 'Hello world', 'newline newline2' ],
  date: [ Sat Feb 17 2018 08:00:00 GMT+0800 (中国标准时间) ],
  datetime: [ Sat Feb 17 2018 15:02:31 GMT+0800 (中国标准时间) ] }
```
### 特殊符号
1、`YAML`允许使用两个感叹号，强制转换数据类型。
``` Yaml
e: !!str 123
f: !!str true
```
转为`JavaScript`如下:
``` JavaScript
{ e: '123', f: 'true' }
```
2、 `…`和`---`配合使用，在一个配置文件中代表一个文件的结束：
``` Yaml
---
time: 20:03:20
player: Sammy Sosa
action: strike (miss)
...
---
time: 20:03:47
player: Sammy Sosa
action: grand slam
...
```
3、`>`在字符串中折叠换行，`|`保留换行符，这两个符号是`YAML`中字符串经常使用的符号，比如：
``` Yaml
this: |
    Foo
    Bar
that: >
    Foo
    Bar
```
转为`JavaScript`如下:
``` JavaScript
{ this: 'Foo\nBar\n', that: 'Foo Bar\n' }
```
4、引用。重复的内容在YAML中可以使用&来完成锚点定义，使用*来完成锚点引用，例如：
``` Yaml
defaults: &defaults
    adapter:  postgres
    host:     localhost

development:
    database: myapp_development
    <<: *defaults

test:
    database: myapp_test
    <<: *defaults
```
转为`JavaScript`如下:
``` JavaScript
{ defaults: { adapter: 'postgres', host: 'localhost' },
    development: 
    {   database: 'myapp_development',
        adapter: 'postgres',
        host: 'localhost' },
    test: 
    {   database: 'myapp_test',
        adapter: 'postgres',
        host: 'localhost' } }
```
注意，不能独立的定义锚点，比如不能直接这样写： &SS Sammy Sosa；另外，锚点能够定义更复杂的内容，比如：
``` Yaml
default: &default
    - Mark McGwire
    - Sammy Sosa
hr: *default
```
那么hr相当于引用了default的数组，注意，hr: *default要写在同一行。

测试样例：cargo run --bin xdiff-live run -p rust -c  fixtures/test.yml -e a=100 -e @b=2 -e m=10
测试样例：cargo run --bin xdiff-live -- parse

https://jsonplaceholder.typicode.com/todos/1?a=1&b=2

https://jsonplaceholder.typicode.com/todos/2?a=2&b=3

todo

测试样例：cargo run --bin xreq-live -- run -p todo -c fixtures/xreq_test.yml

