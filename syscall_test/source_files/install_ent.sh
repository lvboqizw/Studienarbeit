if which ent > /usr/bin/ent; then
    echo "Command exists"
else
    mkdir ent
    pushd .
    cd ent
    wget http://www.fourmilab.ch/random/random.zip
    unzip random.zip
    make
    mv ./ent /usr/bin
    popd
fi