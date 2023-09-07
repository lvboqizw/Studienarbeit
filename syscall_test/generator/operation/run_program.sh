# ./operation/file_sys

scone cas attest $SCONE_CAS_ADDR --only_for_testing-trust-any --only_for_testing-debug  --only_for_testing-ignore-signer -C -G -S

export NS=socket_test_$RANDOM_$RANDOM
export MREN_SERVER=`SCONE_HASH=1 ./operation/server`
export MREN_CLIENT=`SCONE_HASH=1 ./operation/client`

echo $NS
echo $MREN_SERVER
echo $MREN_CLIENT

export PREDECESSOR=$(scone session create -e NS="$NS" -e MREN_SERVER="$MREN_SERVER" -e MREN_CLIENT="$MREN_CLIENT" /operation/session.yml)

SCONE_CONFIG_ID=$NS/server ./operation/server &
sleep 2
if pgrep -x server > /dev/null
then 
    SCONE_CONFIG_ID=$NS/client ./operation/client 
fi