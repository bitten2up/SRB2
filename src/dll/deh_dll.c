// SONIC ROBO BLAST 2; shitty DLL loading edition
//-----------------------------------------------------------------------------
// Copyright (C) 2023 by bitten2up
//
// This program is free software distributed under the
// terms of the GNU General Public License, version 2.
// See the 'LICENSE' file for more details.
//-----------------------------------------------------------------------------
/// \file  deh_dll.c
/// \brief contains all the code for the stupid fucking dll shit
// ever have a legitimatly stupid idea, i know i have

#include <dlfcn.h>
#include <sys/stat.h>
#include "../doomdef.h"
#include "../keys.h"
#include "../m_menu.h"
#include "../filesrch.h"

#include "deh_dll.h"
#include "dll_api.h"

void *handle;

// void DEH_AddonDll(void)
// i dont care this isnt really a dehacked file, but just load the fucking dll
void DEH_AddonDll(INT32 ch)
{
	if (ch != 'y' && ch != KEY_ENTER)
		return;
  // make sure we only have one dll loaded at a time, i dont want to set up a system for keeping track of this right now
  if (handle != NULL)
  {
    CONS_Alert(CONS_WARNING, "Unloading currently loaded dll");
    dlclose(handle);
  }

  // setup stuff we will pass on to the dll
  DLLpointers SendPointer;
	SendPointer.emeralds = &emeralds;

	// start loading
  CONS_Printf("Loading dll, ");
	void (*patch_main)(DLLpointers);
  char *error;
  handle = dlopen(va("%s%s", menupath, dirmenu[dir_on[menudepthleft]]+DIR_STRING), RTLD_LAZY);
  if (!handle) {
  	CONS_Alert(CONS_WARNING, "Loading DLL failed, is your patch corect?");
  }
  dlerror();
  patch_main = dlsym(handle, "SRB2Entry");
  // aborting if this didnt load
  if ((error = dlerror()) != NULL)  {
    fputs(error, stderr);
    exit(1);
  }
  (*patch_main)(SendPointer);
}
