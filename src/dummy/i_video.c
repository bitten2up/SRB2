// Emacs style mode select   -*- C++ -*-
//-----------------------------------------------------------------------------
//
// Copyright (C) 1993-1996 by id Software, Inc.
// Portions Copyright (C) 1998-2000 by DooM Legacy Team.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//-----------------------------------------------------------------------------
/// \file
/// \brief SRB2 graphics stuff for 3DS

#include <math.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <3ds.h>
#include <citro3d.h>
#include <glide.h>

#include "../doomdef.h"
#include "../doomtype.h"
#include "../command.h"
#include "../i_video.h"
#include "../z_zone.h"

#include "../hardware/hw_drv.h"
#include "../hardware/hw_main.h"
//#include "../hardware/hw_vcache.h"
//#include "r_nds3d.h"
//#include "r_queue.h"
//#include "r_texcache.h"
//#include "nds_utils.h"

#define GPU_CMDBUF_SIZE		(1024 * 1024 * 9)

#define PALETTE_SIZE		256
#define PALETTE_CACHE_SIZE	2

#define MAX_TEX_SIZE			(1 * 1024 * 1024)

#define RGB8A_TO_UINT32(r,g,b,a)	((u32)(((u8)r<<24) | ((u8)g<<16) | ((u8)b<<8) | ((u8)a)))

rendermode_t rendermode = render_opengl;
rendermode_t chosenrendermode = render_none;
boolean highcolor = false;
boolean allow_fullscreen = false;

consvar_t cv_vidwait = CVAR_INIT ("vid_wait", "On", CV_SAVE, CV_OnOff, NULL);

void I_StartupGraphics(void)
{
	vid.width = 400;
	vid.height = 240;
	vid.bpp = 4;
	vid.rowbytes = vid.width * vid.bpp;
	vid.recalc = false;

	HWD.pfnInit             = NDS3DVIDEO_Init;
	HWD.pfnShutdown         = NDS3DVIDEO_Shutdown;
	HWD.pfnFinishUpdate     = NDS3DVIDEO_FinishUpdate;
	HWD.pfnDraw2DLine       = NDS3DVIDEO_Stub;
	HWD.pfnDrawPolygon      = NDS3DVIDEO_DrawPolygon;
	HWD.pfnSetBlend         = NDS3DVIDEO_SetBlend;
	HWD.pfnClearBuffer      = NDS3DVIDEO_ClearBuffer;
	HWD.pfnSetTexture       = NDS3DVIDEO_SetTexture;
	HWD.pfnReadRect         = NDS3DVIDEO_Stub;
	HWD.pfnGClipRect        = NDS3DVIDEO_Stub;
	HWD.pfnClearMipMapCache = NDS3DVIDEO_ClearMipMapCache;
	HWD.pfnSetSpecialState  = NDS3DVIDEO_SetSpecialState;
	HWD.pfnSetPalette       = NDS3DVIDEO_SetPalette;
	HWD.pfnGetTextureUsed   = NDS3DVIDEO_GetTextureUsed;
	HWD.pfnDrawMD2          = NDS3DVIDEO_Stub;
	HWD.pfnDrawMD2i         = NDS3DVIDEO_Stub;
	HWD.pfnSetTransform     = NDS3DVIDEO_SetTransform;
	HWD.pfnGetRenderVersion = NDS3DVIDEO_GetRenderVersion;
	HWD.pfnFlushScreenTextures                       = NDS3DVIDEO_FlushScreenTextures;
	HWD.pfnStartScreenWipe                           = NDS3DVIDEO_Stub;
	HWD.pfnEndScreenWipe                             = NDS3DVIDEO_Stub;
	HWD.pfnDoScreenWipe                              = NDS3DVIDEO_Stub;
	HWD.pfnDrawIntermissionBG                        = NDS3DVIDEO_Stub;
	HWD.pfnMakeScreenTexture                         = NDS3DVIDEO_Stub;
	HWD.pfnMakeScreenFinalTexture                    = NDS3DVIDEO_Stub;
	HWD.pfnDrawScreenFinalTexture                    = NDS3DVIDEO_Stub;


	CV_RegisterVar(&cv_vidwait);

	gfxSet3D(true); // Enable stereoscopic 3D
	C3D_Init(GPU_CMDBUF_SIZE);

	HWD.pfnInit(I_Error);

	osSetSpeedupEnable(true);

	HWR_Startup();
}
void I_ShutdownGraphics(void){}

void VID_StartupOpenGL(void){

}

void I_SetPalette(RGBA_t *palette)
{
	(void)palette;
}

INT32 VID_NumModes(void)
{
	return 0;
}

INT32 VID_GetModeForSize(INT32 w, INT32 h)
{
	(void)w;
	(void)h;
	return 0;
}

void VID_PrepareModeList(void){}

INT32 VID_SetMode(INT32 modenum)
{
	(void)modenum;
	return 0;
}

boolean VID_CheckRenderer(void)
{
	return false;
}

void VID_CheckGLLoaded(rendermode_t oldrender)
{
	(void)oldrender;
}

const char *VID_GetModeName(INT32 modenum)
{
	(void)modenum;
	return NULL;
}

UINT32 I_GetRefreshRate(void) { return 35; }

void I_UpdateNoBlit(void){}

void I_FinishUpdate(void){}

void I_UpdateNoVsync(void) {}

void I_WaitVBL(INT32 count)
{
	(void)count;
}

void I_ReadScreen(UINT8 *scr)
{
	(void)scr;
}

void I_BeginRead(void){}

void I_EndRead(void){}

