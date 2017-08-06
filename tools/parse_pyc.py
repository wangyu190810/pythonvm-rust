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
co = compile(source, 'test.py', 'exec')

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