# script to set console collors to yellow letters on yellow background

$MethodDefinition = @'

[DllImport("kernel32.dll", CharSet = CharSet.Unicode)]
public static extern bool SetConsoleTextAttribute(System.IntPtr consoleOutput, System.UInt16 attributes);

[DllImport("kernel32.dll", CharSet = CharSet.Unicode)]
public static extern System.IntPtr GetStdHandle(System.UInt32 nStdHandle);

'@

$Kernel32 = Add-Type -MemberDefinition $MethodDefinition -Name "Kernel32" -Namespace "Win32" -PassThru
$stdOutHandle = [uint32]"0xFFFFFFF5" # STD_OUTPUT_HANDLE (DWORD) -11
$consoleHandle = $Kernel32::GetStdHandle($stdOutHandle )
$consAttr = [uint16]"0xEE"
$Kernel32::SetConsoleTextAttribute($consoleHandle, $consAttr)

Invoke-Expression ".\chall1.exe"