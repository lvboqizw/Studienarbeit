./operation/file_sys

./operation/server &
sleep 2
if pgrep -x server > /dev/null
then 
    ./operation/client 
fi