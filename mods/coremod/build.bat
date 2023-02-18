@echo off

mkdir "target/debug/out"
cargo build --features="client"
echo F|xcopy "target\debug\coremod.dll" "target\debug\out\coremod_client.dll" /y
cargo build --features="server"
echo F|xcopy "target\debug\coremod.dll" "target\debug\out\coremod_server.dll" /y

rm "target\debug\out\*"
rm "target\debug\coremod.zip
cd "target\debug\out"
7z a -r "..\coremod.zip" *
cd "..\..\.."
echo F|xcopy "target\debug\coremod.zip" "..\..\server\mods\coremod.zip" /y