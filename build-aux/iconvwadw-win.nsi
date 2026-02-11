!include "MUI2.nsh"

Name "iconvwadw"
OutFile "iconvwadw-win-x64.exe"
Icon "C:\usr\bin\io.github.rsvzz.iconvwadw.ico"
InstallDir "$PROGRAMFILES64\iconvwadw"
!define MUI_ABORTWARNING
!define MUI_UNABORTWARNING

Page directory
Page instfiles

Section "install"
  SetOutPath $INSTDIR
  SetOutPath "$INSTDIR\bin"
  File "C:\usr\bin\iconvwadw.exe"
  File "C:\usr\bin\*.dll"
  File "C:\usr\bin\io.github.rsvzz.iconvwadw.ico"
  SetOutPath $INSTDIR
  WriteUninstaller "Uninstall.exe"
  SetOutPath "$INSTDIR\share\icons\hicolor\scalable\apps"
  File "C:\usr\share\icons\hicolor\scalable\apps\*.svg"
  SetOutPath "$INSTDIR\share\iconvwadw\ui"
  File "C:\usr\share\iconvwadw\ui\*.ui"
  SetOutPath "$INSTDIR\share\icons\Adwaita"
  File /r "C:\usr\share\icons\Adwaita\*.*"
  SetOutPath $INSTDIR
SectionEnd

Section "Uninstall"
    Delete "$INSTDIR\bin\iconvwadw.exe"
    Delete "$INSTDIR\bin\*.dll"
    Delete "$INSTDIR\bin\*.ico"
    Delete "$INSTDIR\Uninstall.exe"
    Delete "$DESKTOP\iconvwadw.lnk"
    RMDir /r "$INSTDIR\share"
    RMDir /r "$INSTDIR\bin"
    RMDir /r "$INSTDIR"
SectionEnd

Section "Access Direct"
  CreateShortcut "$DESKTOP\iconvwadw.lnk" "$INSTDIR\bin\iconvwadw.exe" "" "$INSTDIR\bin\io.github.rsvzz.iconvwadw.ico"
SectionEnd