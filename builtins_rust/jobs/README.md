
找到项目bash
先make clean
找到makefile（注意有多个）将CC = gcc 改为  CC = gcc -fPIC
再make
然后生成动态链接库，例如我需要用到jobs
gcc -shared -fPIC -o libjobs.so jobs.o
批量处理脚本create-so.sh


添加动态库
新建  /etc/ld.so.conf.d/jobs.conf
将 依赖的os路径写入 /etc/ld.so.conf.d/jobs.conf
ldconfig -X


