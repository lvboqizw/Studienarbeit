./operation/file_sys

scone cas attest $SCONE_CAS_ADDR --only_for_testing-trust-any --only_for_testing-debug  --only_for_testing-ignore-signer -C -G -S

SESSION_NAME=socket_test_$(od /dev/urandom -A n -t d -N 1)
SESSION_NAME=`echo $SESSION_NAME | sed 's/[[:space:]]//g'`
MREN_SERVER=`SCONE_HASH=1 ./operation/server`
MREN_CLIENT=`SCONE_HASH=1 ./operation/client`

echo $SESSION_NAME
PREDECESSOR=$(scone session create -e SESSION_NAME="$SESSION_NAME" -e MREN_SERVER="$MREN_SERVER" -e MREN_CLIENT="$MREN_CLIENT" /operation/session.yml)

SCONE_CONFIG_ID=$SESSION_NAME/server ./operation/server &
sleep 2
if pgrep -x server > /dev/null
then 
    SCONE_CONFIG_ID=$SESSION_NAME/client ./operation/client 
fi