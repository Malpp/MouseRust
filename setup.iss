#define AppName "mouse_rust"

[Setup]
AppName={#AppName}
AppVersion=1.0
WizardStyle=modern
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
Compression=lzma2
SolidCompression=yes
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64
OutputBaseFilename=mouse_rust_windows

[Files]
Source: "static/*"; DestDir: "{app}/static"; Flags: recursesubdirs
Source: "bin/*"; DestDir: "{app}/bin"
Source: "target/release/mouse.exe"; DestDir: "{app}"

[Run]
Filename: {app}\bin\nssm.exe; Parameters: "install mouse_rust ""{app}\mouse.exe""" ; Flags: runhidden;
Filename: {app}\bin\nssm.exe; Parameters: "start mouse_rust" ; Flags: runhidden;

[UninstallRun]
Filename: {app}\bin\nssm.exe; Parameters: "remove mouse_rust confirm" ; Flags: runhidden;