@echo off

mkdir "target/release/out"
cargo build --release --features="client"
echo F|xcopy "target\release\coremod.dll" "target\release\out\coremod_client.dll" /y
cargo build --release --features="server"
echo F|xcopy "target\release\coremod.dll" "target\release\out\coremod_server.dll" /y

rm "target\release\out\*"
rm "target\release\coremod.zip
cd "target\release\out"
7z a -r "..\coremod.zip" *
cd "..\..\.."
echo F|xcopy "target\release\coremod.zip" "..\..\server\mods\coremod.zip" /y