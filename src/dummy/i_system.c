#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include "../doomdef.h"
#include "../doomstat.h"
#include "../d_main.h"
#include "../m_menu.h"
#include "../i_system.h"
#include "../i_joy.h"

#ifdef __3DS__
#include <3ds.h>
#endif
FILE *logstream = NULL;

UINT8 graphics_started = 0;

UINT8 keyboard_started = 0;

static INT64 start_time; // as microseconds since the epoch

size_t I_GetFreeMem(size_t *total)
{
	*total = 178 * 1024 * 1024;

	return linearSpaceFree() + vramSpaceFree();
}

void I_Sleep(UINT32 ms)
{
	(void)ms;
}


precise_t I_GetPreciseTime(void)
{
	return 0;
}

UINT64 I_GetPrecisePrecision(void)
{
	return 1000000;
}

void I_GetEvent(void){}

void I_OsPolling(void)
{
	I_GetEvent();
}

ticcmd_t *I_BaseTiccmd(void)
{
	return NULL;
}

ticcmd_t *I_BaseTiccmd2(void)
{
	return NULL;
}

void I_Quit(void)
{
	printf("EXIT!\n");
	//I_ShutdownDigMusic();
	I_ShutdownGraphics();
	exit(0);
}

void I_Error(const char *fmt, ...)
{
#if 1
	char txt[8192];
	va_list argptr;

	va_start(argptr, fmt);
	vsprintf(txt, fmt, argptr);
	va_end(argptr);

	fprintf(stderr, "%s", txt);

	exit(-1);
#else
	(void)fmt;
#endif
}


void I_Tactile(FFType Type, const JoyFF_t *Effect)
{
	(void)Type;
	(void)Effect;
}

void I_Tactile2(FFType Type, const JoyFF_t *Effect)
{
	(void)Type;
	(void)Effect;
}

void I_JoyScale(void){}

void I_JoyScale2(void){}

void I_InitJoystick(void)
{
	Joystick.bGamepadStyle = false;
	Joystick2.bGamepadStyle = false;
}

void I_InitJoystick2(void){}

INT32 I_NumJoys(void)
{
	return 0;
}

const char *I_GetJoyName(INT32 joyindex)
{
	(void)joyindex;
	return NULL;
}

#ifndef NOMUMBLE
void I_UpdateMumble(const mobj_t *mobj, const listener_t listener)
{
	(void)mobj;
	(void)listener;
}
#endif

void I_OutputMsg(const char *error, ...)
{
	char tmp[512];

	va_list args;
	va_start(args, error);
	vsnprintf(tmp, sizeof tmp, error, args);
	va_end(args);

	printf("%s", tmp);
}

void I_StartupMouse(void){}

void I_StartupMouse2(void){}

INT32 I_GetKey(void)
{
	return 0;
}

void I_StartupTimer(void)
{
	start_time = osGetTime();
}

void I_AddExitFunc(void (*func)())
{
	(void)func;
}

void I_RemoveExitFunc(void (*func)())
{
	(void)func;
}

static char exePath[0x400];
static bool wadAtExePath;

static bool parsePath(const char *path)
{
	if (strncmp(path, "sdmc:", 5) != 0)
		return false;

	const char *end = path + strlen(path);

	while (end != path)
	{
		if (*end == '/')
			break;
		end--;
	}

	if (*end == '/')
	{
		if (end - path >= 0x400)
			return false;

		memcpy(exePath, path, end - path);
		exePath[end - path] = '\0';
	}
	else return false;

	CONS_Printf("exePath: %s\n", exePath);

	return true;
}

INT32 I_StartupSystem(void)
{
	extern INT32 myargc;
	extern char **myargv;
	if (PTMSYSM_CheckNew3DS())
	{
		osSetSpeedupEnable(true);
		// enable fast clock + L2 cache on new3ds
		PTMSYSM_ConfigureNew3DSCPU(3);
		osSetSpeedupEnable(true);
	}
	gfxInitDefault();
	consoleInit(GFX_BOTTOM, NULL);

		/* Figure out where srb2 is stored */
	if (myargc >= 1 && parsePath(myargv[0]))
	{
		if (chdir(exePath) != 0)
			printf("chdir#1 failed!\n");

		FILE *f;
		if ((f = fopen("srb2.srb", "rb")) != NULL)
        	{
        		wadAtExePath = true;
        		fclose(f);
        		return 0;
        	}

		/* Could not open file */
	}
	else
	{
		// CIA version
		chdir("sdmc:/3ds/srb2");
		strcpy(exePath, "sdmc:/3ds/srb2");
		wadAtExePath = true;
		return 0;
	}

	if (chdir("sdmc:/") != 0)
		printf("chdir#2 failed!\n");

	return 0;
}

void I_ShutdownSystem(void){}

void I_GetDiskFreeSpace(INT64* freespace)
{
	*freespace = 0;
}

char *I_GetUserName(void)
{
	printf("I_GetUserName()\n");
	return NULL;
}

INT32 I_mkdir(const char *dirname, INT32 unixright)
{
	if (mkdir(dirname, unixright) != 0)
	{
		printf("I_mkdir() failed!\n");
		return -1;
	}

	return 0;
}

const CPUInfoFlags *I_CPUInfo(void)
{
	return NULL;
}

const char *I_LocateWad(void)
{
	if (wadAtExePath)
	{
		return exePath;
	}

	return "/";
}
void I_GetJoystickEvents(void){}

void I_GetJoystick2Events(void){}

void I_GetMouseEvents(void){}

void I_UpdateMouseGrab(void){}

char *I_GetEnv(const char *name)
{
	(void)name;
	return NULL;
}

INT32 I_PutEnv(char *variable)
{
	(void)variable;
	return -1;
}

INT32 I_ClipboardCopy(const char *data, size_t size)
{
	(void)data;
	(void)size;
	return -1;
}

const char *I_ClipboardPaste(void)
{
	return NULL;
}

size_t I_GetRandomBytes(char *destination, size_t amount)
{
	(void)destination;
	(void)amount;
	return 0;
}

void I_RegisterSysCommands(void){}

void I_GetCursorPosition(INT32 *x, INT32 *y)
{
	(void)x;
	(void)y;
}

#include "../sdl/dosstr.c"

