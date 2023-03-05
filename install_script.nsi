!include "MUI2.nsh"

!getdllversion "src-tauri\target\release\Randomizer.exe" AppVersion_

Name "Randomizer"
OutFile "dist\Randomizer_${AppVersion_1}.${AppVersion_2}.${AppVersion_3}.exe"
SetCompressor /SOLID lzma
Unicode True

InstallDir "$PROGRAMFILES64\Randomizer"
InstallDirRegKey HKLM "Software\ru.oyashiro.randomizer" ""

RequestExecutionLevel admin

;--------------------------------
;Variables

Var StartMenuFolder

;--------------------------------
;Interface Settings

!define MUI_ABORTWARNING
!define MUI_LANGDLL_ALLLANGUAGES

;--------------------------------
;Language Selection Dialog Settings

!define MUI_LANGDLL_REGISTRY_ROOT "HKLM"
!define MUI_LANGDLL_REGISTRY_KEY "Software\ru.oyashiro.randomizer"
!define MUI_LANGDLL_REGISTRY_VALUENAME "Installer Language"

;--------------------------------
;Pages

!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_DIRECTORY

!define MUI_STARTMENUPAGE_REGISTRY_ROOT "HKLM"
!define MUI_STARTMENUPAGE_REGISTRY_KEY "Software\ru.oyashiro.randomizer"
!define MUI_STARTMENUPAGE_REGISTRY_VALUENAME "Start Menu Folder"

!insertmacro MUI_PAGE_STARTMENU Application $StartMenuFolder

!insertmacro MUI_PAGE_INSTFILES

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

;--------------------------------
;Languages

!insertmacro MUI_LANGUAGE "English"
!insertmacro MUI_LANGUAGE "Russian"

LangString desktop_shortcut ${LANG_ENGLISH} "Create desktop shortcut"
LangString desktop_shortcut ${LANG_RUSSIAN} "Создать ярлык на рабочем столе"

LangString install_data ${LANG_ENGLISH} "Sample data files"
LangString install_data ${LANG_RUSSIAN} "Пример файлов в data"

LangString installing_webview ${LANG_ENGLISH} "Installing WebView2 Runtime"
LangString installing_webview ${LANG_RUSSIAN} "Установка WebView2 Runtime"

LangString webview_alreay_installed ${LANG_ENGLISH} "WebView2 Runtime is already installed"
LangString webview_alreay_installed ${LANG_RUSSIAN} "WebView2 Runtime уже установлена"

LangString remove_data ${LANG_ENGLISH} "Remove $\"data$\" directory? This will remove all added backgrounds, lists and sounds."
LangString remove_data ${LANG_RUSSIAN} "Удалить папку $\"data$\"? Это удалит все добавленные фоны, списки и звуки."

;--------------------------------
;Reserve Files

!insertmacro MUI_RESERVEFILE_LANGDLL

;--------------------------------
;Installer Sections

Section "-Application files" SecMain

	Call "installWebView2"

	SetOutPath "$INSTDIR"
	File "src-tauri\target\release\Randomizer.exe"
	File "extra\bass.dll"

	SetOutPath "$INSTDIR\plugins"
	File /r "extra\plugins\*.dll"

	;Store installation folder
	WriteRegStr HKLM "Software\ru.oyashiro.randomizer" "" $INSTDIR

	;Create uninstaller
	WriteUninstaller "$INSTDIR\Uninstall.exe"

	!insertmacro MUI_STARTMENU_WRITE_BEGIN Application
		CreateDirectory "$SMPROGRAMS\$StartMenuFolder"
		CreateShortcut "$SMPROGRAMS\$StartMenuFolder\Randomizer.lnk" "$INSTDIR\Randomizer.exe"
		CreateShortcut "$SMPROGRAMS\$StartMenuFolder\Uninstall.lnk" "$INSTDIR\Uninstall.exe"
	!insertmacro MUI_STARTMENU_WRITE_END

SectionEnd

Section /o "$(desktop_shortcut)" SecDesktop
	CreateShortcut "$DESKTOP\Randomizer.lnk" "$INSTDIR\Randomizer.exe"
SectionEnd

Section /o "$(install_data)" SecDataFiles
	SetOutPath "$INSTDIR\data"
	File /r "extra\data\*.*"
SectionEnd

;--------------------------------
;Installer Functions

Function .onInit

	!insertmacro MUI_LANGDLL_DISPLAY

FunctionEnd

Function installWebView2

	ReadRegStr $0 HKLM \
		"SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" "pv"

	${If} ${Errors}
	${OrIf} $0 == ""
		SetDetailsPrint both
		DetailPrint "$(installing_webview)"
		SetDetailsPrint listonly

		InitPluginsDir
		SetOutPath "$PLUGINSDIR"
		File "extra\MicrosoftEdgeWebview2Setup.exe"
		ExecWait '"$PLUGINSDIR\MicrosoftEdgeWebview2Setup.exe" /silent /install'

		SetDetailsPrint both
	${Else}
		DetailPrint "$(webview_alreay_installed)"
	${EndIf}

FunctionEnd

;--------------------------------
;Uninstaller Section

Section "Uninstall"

Delete "$INSTDIR\Randomizer.exe"
Delete "$INSTDIR\bass.dll"
Delete "$INSTDIR\plugins\*.dll"

RMDir "$INSTDIR\plugins"

MessageBox MB_YESNO "$(remove_data)" IDYES true IDNO false
true:
	RMDir /r "$INSTDIR\data"
false:

Delete "$INSTDIR\Uninstall.exe"

RMDir "$INSTDIR"

!insertmacro MUI_STARTMENU_GETFOLDER Application $StartMenuFolder

Delete "$SMPROGRAMS\$StartMenuFolder\Randomizer.lnk"
Delete "$SMPROGRAMS\$StartMenuFolder\Uninstall.lnk"
RMDir "$SMPROGRAMS\$StartMenuFolder"

Delete "$DESKTOP\Randomizer.lnk"

DeleteRegKey HKLM "Software\ru.oyashiro.randomizer"

SectionEnd

;--------------------------------
;Uninstaller Functions

Function un.onInit

	!insertmacro MUI_UNGETLANGUAGE

FunctionEnd