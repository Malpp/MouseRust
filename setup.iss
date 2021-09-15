#define AppName "mouse_rust"

[Setup]
AppName={#AppName}
AppVersion=1.1
WizardStyle=modern
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
Compression=lzma2
SolidCompression=yes
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64
OutputBaseFilename=mouse_rust_windows
SetupIconFile=cog.ico

[Files]
Source: "static/*"; DestDir: "{app}/static"; Flags: recursesubdirs
Source: "target/release/mouse.exe"; DestDir: "{app}"
Source: "cog.ico"; DestDir: "{app}"

[Run]
Filename: {app}/mouse.exe; Flags: nowait postinstall skipifsilent

[Icons]
Name: "{commonstartup}\Mouse Rust"; Filename: "{app}/mouse.exe"; IconFilename: "{app}\cog.ico";
