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

#ifdef HWRENDER
#include "hw_drv.h"
#include "hw_portals.h"
#include "hw_main.h"
#include "../r_main.h"
#include "../z_zone.h"

gl_portal_array_t gl_portal_arrays[MAXPORTALS_CAP+1] = {0};

INT32 gl_portal_level = 0; // portal recursion level

boolean gl_drawing_stencil = false; // used when drawing segs to stencil buffer
sector_t *gl_portalcullsector = NULL;
line_t *gl_portalclipline = NULL;

// debug tools
boolean gl_printportals = true; // print info about portals on this frame
INT32 gl_debugportal = 0; // hide main viewpoint and only render this portal without stencil

void HWR_PortalFrame(gl_portal_t *portal, boolean set_culling)
{
	viewx = portal->viewx;
	viewy = portal->viewy;
	viewz = portal->viewz;

	viewangle = portal->viewangle;
	viewsin = FINESINE(viewangle>>ANGLETOFINESHIFT);
	viewcos = FINECOSINE(viewangle>>ANGLETOFINESHIFT);

	// set_culling will be true if RenderBSPNode is about to be called
	// so need to enable portal culling for that
	if (set_culling)
	{
		gl_portalcullsector = portal->clipline->frontsector;
		gl_portalclipline = portal->clipline;
	}
}

// get currently used portal array
gl_portal_array_t *HWR_GetPortalArray(void)
{
	INT32 level = gl_portal_level;
	if (gl_rendering_skybox)
		level++;
	return &gl_portal_arrays[level];
}

// TODO move to r_main.c next to fixed point functions?

// More precise version of R_PointToAngle2 using floats and atan2.
static angle_t R_PointToAngle2Precise(fixed_t pviewx, fixed_t pviewy, fixed_t x, fixed_t y)
{
	fixed_t dx = x - pviewx;
	fixed_t dy = y - pviewy;
	float radians;

	if (!dx && !dy)
		return 0;

	// no need for correct scale with FIXED_TO_FLOAT here
	// since we're just calculating the angle
	radians = atan2(dy, dx);

	return (angle_t)(radians / M_PI * ANGLE_180);
}

// More precise version of R_PointToDist2 using floats and sqrt.
static fixed_t R_PointToDist2Precise(fixed_t px2, fixed_t py2, fixed_t px1, fixed_t py1)
{
	// float-fixed conversions can be omitted here
	// because they cancel each other out in this case

	float dx = px1 - px2;
	float dy = py1 - py2;
	double result = sqrt(dx*dx + dy*dy);

	return (fixed_t)result;
}

boolean HWR_AddPortal(line_t *start, line_t *dest, seg_t *seg)
{
	gl_portal_array_t *array;
	gl_portal_t *portal;

	angle_t dangle;
	fixed_t disttopoint;
	angle_t angtopoint;
	vertex_t dest_c, start_c;

	if ((gl_portal_level + gl_rendering_skybox) >= cv_maxportals.value ||
			(gl_debugportal &&
			(gl_debugportal != (start-lines) || gl_portal_level)))
	{
		return false;
	}
	if (gl_debugportal)
		gl_debugportal = -1; // skip other portal segs from the same line

	array = HWR_GetPortalArray();

	// yet another dynamic array
	if (!array->portals)
	{
		array->capacity = INIT_PORTAL_ARRAY_SIZE;
		array->portals = Z_Malloc(sizeof(gl_portal_t) * array->capacity, PU_LEVEL,
				&array->portals);
	}
	else if (array->size == array->capacity)
	{
		array->capacity *= 2;
		array->portals = Z_Realloc(array->portals, sizeof(gl_portal_t) * array->capacity,
				PU_LEVEL, &array->portals);
	}

	portal = &array->portals[array->size++];

	// Most fixed-point calculations and trigonometric function tables are replaced by
	// floats and cmath library calls in this part to improve the precision of the
	// location and angle of the new viewpoint.
	//
	// This reduces artefacts on the edges of portals, showing thin lines/pixels
	// of the underlying graphics. (for example the sky texture) It's not 100%
	// perfectly aligned and artefact-free, but looks noticeably
	// better than the original code. I'm not even sure if it's this
	// code or the nodebuilder or hw_map or something else causing the remaining issues..
//#define R_PointToAngle2Precise R_PointToAngle2
//#define R_PointToDist2Precise R_PointToDist2
	dangle = R_PointToAngle2Precise(0,0,dest->dx,dest->dy) - R_PointToAngle2Precise(start->dx,start->dy,0,0);

	// looking glass center
	start_c.x = start->v1->x/2 + start->v2->x/2;
	start_c.y = start->v1->y/2 + start->v2->y/2;

	// other side center
	dest_c.x = dest->v1->x/2 + dest->v2->x/2;
	dest_c.y = dest->v1->y/2 + dest->v2->y/2;

	disttopoint = R_PointToDist2Precise(start_c.x, start_c.y, viewx, viewy);
	angtopoint = R_PointToAngle2Precise(start_c.x, start_c.y, viewx, viewy);
	angtopoint += dangle;

	float fang = ((float)angtopoint / 4294967296.0f) * 2.0f * M_PI;

	//portal->viewx = dest_c.x + FixedMul(FINECOSINE(angtopoint>>ANGLETOFINESHIFT), disttopoint);
	//portal->viewy = dest_c.y + FixedMul(FINESINE(angtopoint>>ANGLETOFINESHIFT), disttopoint);
	//portal->viewx = dest_c.x + FixedMul(FLOAT_TO_FIXED(cos(fang)), disttopoint);
	//portal->viewy = dest_c.y + FixedMul(FLOAT_TO_FIXED(sin(fang)), disttopoint);
	// cos and sin are just scaling disttopoint so no need for float-fixed conversions
	portal->viewx = dest_c.x + (fixed_t)(cos(fang) * disttopoint);
	portal->viewy = dest_c.y + (fixed_t)(sin(fang) * disttopoint);
	portal->viewz = viewz + dest->frontsector->floorheight - start->frontsector->floorheight;
	portal->viewangle = viewangle + dangle;
	portal->seg = seg;
	portal->clipline = dest;

	portal->angle1 = R_PointToAngle64(seg->v1->x, seg->v1->y) + dangle;
	portal->angle2 = R_PointToAngle64(seg->v2->x, seg->v2->y) + dangle;

	return true;
}

void HWR_ClearPortals(void)
{
	HWR_GetPortalArray()->size = 0;
}
#endif
