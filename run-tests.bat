@echo off
setlocal enabledelayedexpansion

set TESTDIR=testing
set DIFFOPTS=-a -i -b -w -B

cargo build --quiet

set PROGRAM=datalog-interpreter
set BINARY=target\debug\%PROGRAM%.exe

for %%B in (20 40 60 80 100) do (

    echo Bucket %%B

    if %%B==20 set NUMBERS=21 22 23
    if %%B==40 set NUMBERS=41 42 43 44 45 46
    if %%B==60 set NUMBERS=61 62 64 66 67 68
    if %%B==80 set NUMBERS=81 82 83 84 85 86
    if %%B==100 set NUMBERS=101 102 103 104 105 108

    for %%N in (!NUMBERS!) do (

        echo Running input %%N

        set INPUTFILE=%TESTDIR%\%%B\input%%N.dl
        set ANSWERFILE=%TESTDIR%\%%B\answer%%N.txt
        set OUTPUTFILE=actual%%N.txt

        "%BINARY%" "!INPUTFILE!" > "!OUTPUTFILE!"

        git diff %DIFFOPTS% "!ANSWERFILE!" "!OUTPUTFILE!"
        if errorlevel 1 (
            echo diff failed on test %%N
        )

        del "!OUTPUTFILE!"
    )
)

endlocal