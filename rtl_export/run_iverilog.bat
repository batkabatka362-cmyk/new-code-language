@echo off
iverilog -o attention.vvp attention.v tb_attention.v
if %errorlevel% neq 0 ( exit /b 1 )
vvp attention.vvp
