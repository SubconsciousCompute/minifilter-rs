# minifilter-rs

## Building Driver

1. Open `VS 2022` as Administrator
2. Goto `minifilter-rs -> minifilter -> RWatch.sln`
3. Build solution in `Debug` mode with `x64` 

## Installing Driver

1. Open Powershell or command prompt as  Administrator
2. `RUNDLL32.EXE SETUPAPI.DLL,InstallHinfSection DefaultInstall 132 <path-to>\minifilter-rs\minifilter\x64\Debug\FsFilter.inf`

You should be able to see the driver at `"C:\Windows\SysWOW64\drivers\FSFilter.sys"`
