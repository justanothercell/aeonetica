@echo off

echo THIS IS DEPRECATED, USE build.py INSTEAD!!

mkdir "target/debug/out/server"
mkdir "target/debug/out/client"

cargo build --features="client"
echo F|xcopy "target\debug\coremod.dll" "target\debug\out\client\coremod_client.dll" /y
cargo build --features="server"
echo F|xcopy "target\debug\coremod.dll" "target\debug\out\server\coremod_server.dll" /y


rm "target\debug\coremod.zip
rm "target\debug\out\coremod_server.zip
rm "target\debug\out\coremod_client.zip

cd "target\debug\out"

cd "server"
7z a -r "..\coremod_server.zip" *
cd ".."

cd "client"
7z a -r "..\coremod_client.zip" *
cd ".."

7z a -r "..\coremod.zip" "coremod_server.zip" "coremod_client.zip"

cd "..\..\.."

echo F|xcopy "target\debug\coremod.zip" "..\..\server\mods\coremod.zip" /y