@ECHO off
SETLOCAL
CALL :find_dp0
REM echo %dp0%
REM node.exe "xxx.js" %*

node.exe  ".\bin\archive.js" %*
ENDLOCAL
EXIT /b %errorlevel%
:find_dp0
SET dp0=%~dp0
EXIT /b
