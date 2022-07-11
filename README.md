# minifilter-rs

## Building Driver

1. Open `VS 2022` as Administrator
2. Goto `minifilter-rs -> minifilter -> RWatch.sln`
3. Build solution in `Debug` mode with `x64`

## Installing Driver

1. Open Powershell or command prompt as Administrator
2. `RUNDLL32.EXE SETUPAPI.DLL,InstallHinfSection DefaultInstall 132 <path-to>\minifilter-rs\minifilter\x64\Debug\FsFilter.inf`

You should be able to see the driver at `"C:\Windows\System32\drivers\FsFilter.sys"`

## Loading Driver

1. Open Powershell or command prompt as Administrator
2. Start the driver using `sc start FSFilter`, expected output:
    ```
   SERVICE_NAME: FSFilter
        TYPE               : 2  FILE_SYSTEM_DRIVER
        STATE              : 4  RUNNING
                                (STOPPABLE, NOT_PAUSABLE, IGNORES_SHUTDOWN)
        WIN32_EXIT_CODE    : 0  (0x0)
        SERVICE_EXIT_CODE  : 0  (0x0)
        CHECKPOINT         : 0x0
        WAIT_HINT          : 0x0
        PID                : 0
        FLAGS              :
   ```
3. Stop the driver using `sc stop FSFilter`, should give the following output:
    ```
   SERVICE_NAME: FSFilter
        TYPE               : 2  FILE_SYSTEM_DRIVER
        STATE              : 1  STOPPED
        WIN32_EXIT_CODE    : 0  (0x0)
        SERVICE_EXIT_CODE  : 0  (0x0)
        CHECKPOINT         : 0x0
        WAIT_HINT          : 0x0
   ```
4. Remove it by `sc delete FSFilter`, should give the following output:
     ```
   [SC] DeleteService SUCCESS
   ```   

You can also run `Fltmc.exe` to see the currently loaded drivers:

```

Filter Name                     Num Instances    Altitude    Frame
------------------------------  -------------  ------------  -----
bindflt                                 1       409800         0
FSFilter                                4       378781         0
WdFilter                                5       328010         0
storqosflt                              0       244000         0
wcifs                                   0       189900         0
CldFlt                                  0       180451         0
FileCrypt                               0       141100         0
luafv                                   1       135000         0
npsvctrig                               1        46000         0
Wof                                     3        40700         0
FileInfo                                5        40500         0
```