BBA

[![N|Solid](https://github.com/EdwardPiwowar/BBA/blob/main/BBALogo.jpg?raw=true)](https://sites.google.com/view/bbaenglish)

Bridge Bot Analyzer

https://sites.google.com/view/bbaenglish

Repository for Build Native-libraries from **EPBotNET.dll**.

The version number for the DLL is automatically taken from the EPBotFFI.csproj|EPBotWasm.csproj project file and requires manual change.



Archive contents:

| Folder         | File           | Architecture  |
| -------------- | -------------- | ------------- |
| linux/x64/     | libEPBot.so    | x64           |
| linux/arm64/   | libEPBot.so    | ARM64         |
| macos/arm64/   | libEPBot.dylib | Apple Silicon |
| windows/x64/   | EPBot.dll      | x64           |
| windows/arm64/ | EPBot.dll      | ARM64         |
| wasm/          | EPBot.wasm     | WebAssembly   |





## Credits

- **EPBot native libraries**: Rick Wilson — [github.com/Rick-Wilson/bba-native-libraries](https://github.com/Rick-Wilson/bba-native-libraries)