# 基本知识储备

###  pyc文件生成

在python2.7中,当我们调用py模块时候，生成pyc文件。
注意：
1. 仅仅一个文件不会在运行时候生成pyc文件，也就是没有使用import 
2. 当被引用的文件更新时，重新生成pyc，更新原则是，当pyc的文件时间小于py文件时候，重新生成pyc

### pyc文件使用

通常情况下，我们不会主动使用pyc文件。特殊情况下可能仅仅会在生产环境发布pyc文件。不过pyc文件的破解好像比较容易。所以也没必
要仅仅发布pyc

通常我们学习python底层，一般都会从C语言的源码开始学习，有c语言基础的人，可能比较容易看懂。但是没有c语言基础的人，可能就会
觉得无法下手。 我们可以 尝试从pyc文件分析一些东西。从而学习一些底层的知识。虽然这两个方向根本没有交叉，但是从容易到难，总
是让我们觉得一些都不是很难。

### 开始操作

先写最简单的python代码:

helloworld.py 

''' python

    print("hello world")

'''

解析python代码执行对象

parse_pyc.py 

''' python

    #-*-coding:utf-8 -*-
    import sys
    import dis

    if len(sys.argv) < 2: 
        print("python parse_pyc.py hello.py")
        sys.exit(1)
    print(sys.argv)
    file_name = sys.argv[1]

    source = open(file_name,"rb").read()
    # 查看source读取内容
    print(source)
    # 读取的数据io流文件，文件命重新命名为test.py 然后执行
    co = compile(source, 'helloworld.py', 'exec')

    print(co)

    print(dir(co))
    # 查看文件名称
    print(co.co_filename)
    # object信息
    print( co.co_consts)
    # object  中val(func, class  etc) 名称
    print(co.co_names)
    # 查看编译信息
    print(dis.dis(co))


'''

查看代码的执行情况

'''

    python parse_pyc.py helloworld.py
    ['parse_pyc.py', 'helloworld.py']
    print('hello world')

    <code object <module> at 000000000287EA30, file "test.py", line 1>
    ['__class__', '__cmp__', '__delattr__', '__doc__', '__eq__', '__format__', '__ge__', '__getattribute__', '__gt__', '__hash__', '__init__', '__le__', '__lt__', '__ne__', '__new__', '__reduce__', '__reduce_ex__', '__repr__', '__setattr__', '__sizeof__', '__str__', '__subclasshook__', 'co_argcount', 'co_cellvars', 'co_code', 'co_consts', 'co_filename', 'co_firstlineno', 'co_flags', 'co_freevars', 'co_lnotab', 'co_name', 'co_names', 'co_nlocals', 'co_stacksize', 'co_varnames']
    helloworld.py
    ('hello world', None)
    ()
      1           0 LOAD_CONST               0 ('hello world')
                  3 PRINT_ITEM
                  4 PRINT_NEWLINE
                  5 LOAD_CONST               1 (None)
                  8 RETURN_VALUE
    None


'''

调用命令生成pyc文件，pyc文件是2进制文件。所以需要依赖工具查看具体的执行结果。

linux 可以选择 

    hexdump -C test.pyc

windows 可以选择notepad++ 安装HEX-Editor

    
'''

 python  -m compileall helloworld.py

'''

生成文件 helloworld.pyc


使用hex-Editor 之后出现的数据 

![](http://wx2.sinaimg.cn/mw690/8c49b4c2ly1fia550jt4wj20y9097dfy.jpg)

看到这些16进制的数据。似乎无法理解到底是什么东西

既然是二进制文件了。自然需要按照字节长度解析文件。也就是定义协议。


### 协议

我们经常接触到协议有tcp和udp协议，经常写的也就tcp和tcp上层协议。比如http协议和httpsocket协议，当然还有其他的网络协议。

有时候，需要我们自拟协议用来交互。

当然pyc也有自己的协议，会定义一个pyc文件，从哪里到哪里都表示什么。这样我们就可以直接根据pyc解析出来需要执行的语法。

### pyc使用的协议


