#!/bin/bash
find .  -name "*.o" > o.list


for line in `cat o.list`
do
   
   echo "000 $line"
   #soname=`basename $line`
   soname=${line%.o}
   echo "AAA $soname"
   soname=${soname#./}
   echo "BBB $soname"
   soname=`echo $soname |sed 's/\//_/g'`
   echo "CCC $soname"
   echo "gcc -shared -fPIC -o /opt/rsbash/builtins/lib${soname}.so $line    "
   gcc -shared -fPIC -o /opt/rsbash/builtins/lib${soname}.so $line    
done
