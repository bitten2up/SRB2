// SONIC ROBO BLAST 2
//-----------------------------------------------------------------------------
// Copyright (C) 2021-2026 by Sonic Team Junior.
//
// This program is free software distributed under the
// terms of the GNU General Public License, version 2.
// See the 'LICENSE' file for more details.
//-----------------------------------------------------------------------------
/// \file hw_portals.c
/// \brief Hardware VisPortals (linedef 40)

#ifndef _HW_PORTALS_H_
#define _HW_PORTALS_H_
typedef struct
{
	// Viewport.
	fixed_t viewx;
	fixed_t viewy;
	fixed_t viewz;
	angle_t viewangle;

	seg_t *seg; // seg that is used for drawing to the stencil buffer
	line_t *clipline;

	// angles for the left and right edges of the portal
	// relative to the viewpoint
	angle_t angle1;
	angle_t angle2;
} gl_portal_t;

#define INIT_PORTAL_ARRAY_SIZE 4
typedef struct
{
	gl_portal_t *portals;
	unsigned int size;
	unsigned int capacity;
} gl_portal_array_t;

extern INT32 gl_portal_level; // portal recursion level

extern boolean gl_drawing_stencil;
extern sector_t *gl_portalcullsector;
extern line_t *gl_portalclipline;
extern INT32 gl_debugportal;

extern boolean gl_printportals; // print info about portals on this frame

void HWR_PortalFrame(gl_portal_t *portal, boolean set_culling);
boolean HWR_AddPortal(line_t *start, line_t *dest, seg_t *seg);
void HWR_ClearPortals(void);
gl_portal_array_t *HWR_GetPortalArray(void);
#endif
