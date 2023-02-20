#!/usr/bin/env bash

MOD_NAME='coremod'

set -xe

BUILD_DIR='target/debug'
GENERATED_MOD_FILE="lib$MOD_NAME.so"

mkdir -p "$BUILD_DIR/out/server"
mkdir -p "$BUILD_DIR/out/client"

cargo build --features='client'

pushd $BUILD_DIR

TARGET_MOD_FILE="out/client/${MOD_NAME}_client.so"

if [ -f $GENERATED_MOD_FILE ]; then
    mv $GENERATED_MOD_FILE $TARGET_MOD_FILE
else
    echo "mod file '$GENERATED_MOD_FILE' could not be found!"
    exit 1
fi

popd # $BUILD_DIR

cargo build --features='server'

pushd $BUILD_DIR

TARGET_MOD_FILE="out/server/${MOD_NAME}_server.so"

if [ -f $GENERATED_MOD_FILE ]; then
    mv $GENERATED_MOD_FILE $TARGET_MOD_FILE
else
    echo "mod file '$GENERATED_MOD_FILE' could not be found!"
    exit 1
fi

SERVER_ZIP="${MOD_NAME}_server.zip"
CLIENT_ZIP="${MOD_NAME}_client.zip"

pushd out
pushd server
zip -r "../$SERVER_ZIP" ./*
popd # server

pushd client
zip -r "../$CLIENT_ZIP" ./*
popd # client

MOD_ZIP="$MOD_NAME.zip"
zip -r "../$MOD_ZIP" $SERVER_ZIP $CLIENT_ZIP

popd # out
popd # $BUILD_DIR

cp "$BUILD_DIR/$MOD_ZIP" "../../server/mods/$MOD_ZIP"
