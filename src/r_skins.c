// SONIC ROBO BLAST 2
//-----------------------------------------------------------------------------
// Copyright (C) 1993-1996 by id Software, Inc.
// Copyright (C) 1998-2000 by DooM Legacy Team.
// Copyright (C) 1999-2025 by Sonic Team Junior.
//
// This program is free software distributed under the
// terms of the GNU General Public License, version 2.
// See the 'LICENSE' file for more details.
//-----------------------------------------------------------------------------
/// \file  r_skins.c
/// \brief Loading skins

#include "doomdef.h"
#include "console.h"
#include "g_game.h"
#include "r_local.h"
#include "st_stuff.h"
#include "w_wad.h"
#include "z_zone.h"
#include "m_menu.h"
#include "m_misc.h"
#include "info.h"
#include "deh_soc.h" // DEH_AddSpriteName
#include "deh_tables.h" // used_skinspr
#include "i_system.h"
#include "r_things.h"
#include "r_skins.h"
#include "p_local.h"
#include "p_animation.h"
#include "dehacked.h" // get_number (for thok)
#include "m_cond.h"
#include "deh_tables.h"
#ifdef HWRENDER
#include "hardware/hw_md2.h"
#endif

INT32 numskins = 0;
skin_t **skins = NULL;

#define PLAYER_ANIMATION_NAME "player"
#define PLAYER_SUPER_ANIMATION_NAME "player_super"

static animation_list_t *base_skin_animations[NUMSKINSPRITESETS];
static UINT16 base_skin_animation_ids[NUMSKINSPRITESETS];

static const char *player_anim_names[] =
{
	"stand",
	"wait",
	"walk",
	"skid",
	"run",
	"dash",
	"pain",
	"stun",
	"dead",
	"drown",
	"roll",
	"gasp",
	"jump",
	"spring",
	"fall",
	"edge",
	"ride",
	"spindash",
	"fly",
	"swim",
	"tired",
	"glide",
	"land",
	"cling",
	"climb",
	"float",
	"float_run",
	"bounce",
	"fire",
	"twinspin",
	"melee",
	"melee_land",
	"transformation",
	"nights_stand",
	"nights_float",
	"nights_fly",
	"nights_drill",
	"nights_stun",
	"nights_pull",
	"nights_attack",

	"tail_0",
	"tail_1",
	"tail_2",
	"tail_3",
	"tail_4",
	"tail_5",
	"tail_6",
	"tail_7",
	"tail_8",
	"tail_9",
	"tail_a",
	"tail_b",
	"tail_c",

	"misc0",
	"misc1",
	"misc2",
	"misc3",
	"misc4",
	"misc5",
	"misc6",
	"misc7",
	"misc8",
	"misc9",

	"continue",
	"continue_lift",
	"continue_spin",
	"continue_partner",

	"end_sign",
	"life_icon",

	"extra"
};

static UINT16 num_player_spritesets = SKINSPRITES_FIRSTFREESLOT;

static const char *player_spriteset_names[NUMSKINSPRITESETS] =
{
	"base",
	"super"
};

UINT16 P_GetSkinAnimation(skin_t *skin, UINT8 spriteset)
{
	if (spriteset >= NUMSKINSPRITESETS)
		spriteset = SKINSPRITES_BASE;

	return skin->sprites[spriteset].animation_id;
}

static UINT16 P_GetOrCreatePlayerSpriteset(const char *spriteset_name)
{
	UINT16 sprset_id;
	char *sprset, *p;
	char *anim_name;
	size_t anim_name_size;
	animation_list_t *animation, *base_anim;

	for (UINT16 i = 0; i < num_player_spritesets; i++)
	{
		if (stricmp(player_spriteset_names[i], spriteset_name) == 0)
			return i;
	}

	if (num_player_spritesets == NUMSKINSPRITESETS)
		return NUMSKINSPRITESETS;

	sprset_id = num_player_spritesets;
	num_player_spritesets++;

	sprset = p = Z_StrDup(spriteset_name);
	player_spriteset_names[sprset_id] = sprset;
	while (*p)
	{
		*p = tolower(*p);
		p++;
	}

	// Create base player animation for this spriteset
	anim_name_size = sizeof(PLAYER_ANIMATION_NAME) + strlen(sprset) + 1;
	anim_name = Z_Malloc(anim_name_size, PU_STATIC, NULL);
	snprintf(anim_name, anim_name_size, "%s_%s", PLAYER_ANIMATION_NAME, sprset);

	animation = P_FindAnimation(anim_name);
	base_anim = base_skin_animations[SKINSPRITES_BASE];

	if (animation)
	{
		animation = P_MergeAnimations(anim_name, base_anim, animation);
	}
	else
	{
		animation = P_DuplicateAnimation(anim_name, base_anim, true);
	}

	base_skin_animations[sprset_id] = animation;
	base_skin_animation_ids[sprset_id] = P_GetNamedAnimationID(anim_name);

	Z_Free(anim_name);

	return sprset_id;
}

UINT8 P_GetPlayerSpritesetID(const char *spriteset_name)
{
	for (UINT16 i = 0; i < num_player_spritesets; i++)
	{
		if (stricmp(player_spriteset_names[i], spriteset_name) == 0)
			return i;
	}

	return NUMSKINSPRITESETS;
}

const char *P_GetPlayerSpritesetName(UINT8 id)
{
	if (id >= num_player_spritesets)
		return NULL;

	return player_spriteset_names[id];
}

UINT8 P_GetMobjSkinSpriteset(mobj_t *mobj, state_t *st)
{
	UINT8 spriteset = mobj->skinspriteset;

	if (P_ShouldUseSuperSprites(mobj, spriteset == SKINSPRITES_BASE && st->frame & SPR2F_SUPER))
	{
		spriteset = SKINSPRITES_SUPER;
	}

	return spriteset;
}

skin_t *P_IsSkinSprite(skin_t *skin, spritenum_t spritenum)
{
	// Cannot possibly be a skin sprite
	if (spritenum < SPR_FIRSTFREESLOT)
		return NULL;

	if (skin)
	{
		if (in_bit_array(used_skinspr, spritenum - SPR_FIRSTFREESLOT))
			return skin;
	}
	else
	{
		for (INT32 i = 0; i < numskins; i++)
		{
			skin = P_IsSkinSprite(skins[i], spritenum);
			if (skin != NULL)
				return skin;
		}
	}

	return NULL;
}

skin_t *P_IsAnimationForSkin(skin_t *skin, UINT16 animation_id)
{
	if (animation_id == 0)
		return NULL;

	if (skin)
	{
		for (skinspriteset_t spriteset = SKINSPRITES_BASE; spriteset < NUMSKINSPRITESETS; spriteset++)
		{
			if (skin->sprites[spriteset].animation_id == animation_id)
				return skin;
		}
	}
	else
	{
		for (INT32 i = 0; i < numskins; i++)
		{
			skin = P_IsAnimationForSkin(skins[i], animation_id);
			if (skin != NULL)
				return skin;
		}
	}

	return NULL;
}

// Checks if an object should use super sprites
boolean P_ShouldUseSuperSprites(mobj_t *mobj, boolean use_super)
{
	if (mobj->player)
	{
		if (mobj->player->charflags & SF_NOSUPERSPRITES || (mobj->player->powers[pw_carry] == CR_NIGHTSMODE && (mobj->player->charflags & SF_NONIGHTSSUPER)))
			use_super = false;
		else if (mobj->player->powers[pw_super] || (mobj->player->powers[pw_carry] == CR_NIGHTSMODE && (mobj->player->charflags & SF_SUPER)))
			use_super = true;
	}

	if (use_super)
	{
		if (mobj->eflags & MFE_FORCENOSUPER)
			use_super = false;
	}
	else if (mobj->eflags & MFE_FORCESUPER)
		use_super = true;

	return use_super;
}

// NOTE: the returned string may be freed at any time
const char *P_GetPlayerSubanimReplacement(skin_t *skin, const char *subanim_name, UINT8 spriteset, player_t *player)
{
	UINT16 animation_id;
	animation_t *subanimation;

	if (!strcmp(subanim_name, "jump"))
		return ((player ? player->charflags : skin->flags) & SF_NOJUMPSPIN) ? "spring" : "roll";
	if (!strcmp(subanim_name, "tired"))
		return ((player ? player->charability : skin->ability) == CA_SWIM) ? "swim" : "fly";

	animation_id = P_GetSkinAnimation(skin, spriteset);
	subanimation = P_GetSubAnimationByName(P_GetAnimationByID(animation_id), subanim_name);

	if (subanimation != NULL && subanimation->fallback != NULL)
		return subanimation->fallback;

	return NULL;
}

// Walks a subanimation replacement chain.
// If P_GetPlayerSubanimReplacement cannot find a suitable fallback, this tries again with a fallback spriteset.
UINT16 P_GetSkinSubanimation(skin_t *skin, const char *subanim_name, UINT8 spriteset, player_t *player, UINT8 *found_spriteset)
{
	const char *original_subanim_name = subanim_name;
	UINT8 i = 0;

	if (!skin)
	{
		if (found_spriteset)
			*found_spriteset = SKINSPRITES_BASE;
		return 0;
	}

	while (!P_IsSkinAnimationValid(skin, subanim_name, spriteset)
		&& strcmp(subanim_name, "stand")
		&& ++i < 64) // recursion limiter
	{
		subanim_name = P_GetPlayerSubanimReplacement(skin, subanim_name, spriteset, player);
		if (!subanim_name)
		{
			if (spriteset == SKINSPRITES_BASE)
			{
				subanim_name = "stand";
				break;
			}
			else
			{
				spriteset = skin->sprites[spriteset].fallback_spriteset;
			}

			subanim_name = original_subanim_name;
		}
	}

	if (i >= 64) // probably an infinite loop...
	{
		subanim_name = "stand";
		spriteset = SKINSPRITES_BASE;
	}

	if (found_spriteset)
		*found_spriteset = spriteset;

	return P_GetNamedSubanimationID(P_GetSkinAnimation(skin, spriteset), subanim_name);
}

static skinspritedef_t *P_GetSkinSpriteSet(skin_t *skin, UINT8 spriteset)
{
	if (!skin)
		return NULL;

	if (spriteset >= NUMSKINSPRITESETS)
		spriteset = SKINSPRITES_BASE;

	return &skin->sprites[spriteset];
}

static skinspritedef_t *P_GetSkinSpritesForAnimation(skin_t *skin, UINT16 anim)
{
	for (unsigned i = 0; i < NUMSKINSPRITESETS; i++)
	{
		if (skin->sprites[i].animation_id && skin->sprites[i].animation_id == anim)
			return &skin->sprites[i];
	}

	return &skin->sprites[SKINSPRITES_BASE];
}

// Gets the sprite number of a skin subanimation
spritenum_t P_GetSkinSpriteID(skin_t *skin, const char *subanim_name, UINT8 spriteset)
{
	if (!skin)
		return SPR_NULL;

	skinspritedef_t *def = P_GetSkinSpriteSet(skin, spriteset);
	animation_list_t *anim = P_GetAnimationByID(def->animation_id);
	animation_t *subanim = P_GetSubAnimationByName(anim, subanim_name);
	return subanim ? def->spritenum[subanim->id] : SPR_NULL;
}

// Gets the spritedef of a skin subanimation
spritedef_t *P_GetSkinSpritedef(skin_t *skin, const char *subanim_name, UINT8 spriteset)
{
	spritenum_t sprite = P_GetSkinSpriteID(skin, subanim_name, spriteset);
	if (sprite == SPR_NULL)
		return NULL;

	return &sprites[sprite];
}

// Gets the spriteinfo of a skin subanimation
spriteinfo_t *P_GetSkinSpriteInfo(skin_t *skin, const char *subanim_name, UINT8 spriteset)
{
	spritenum_t sprite = P_GetSkinSpriteID(skin, subanim_name, spriteset);
	if (sprite == SPR_NULL)
		return NULL;

	return &spriteinfo[sprite];
}

// Gets the spritedef of a skin from the given animation ID and subanimation name
spritedef_t *P_GetSkinAnimSpritedef(skin_t *skin, UINT16 anim, const char *subanim_name)
{
	if (!skin)
		return NULL;

	skinspritedef_t *def = P_GetSkinSpritesForAnimation(skin, anim);
	animation_t *subanim = P_GetSubAnimationByName(P_GetAnimationByID(def->animation_id), subanim_name);
	if (!subanim)
		return NULL;
	spritenum_t sprite = def->spritenum[subanim->id];
	if (sprite == SPR_NULL)
		return NULL;

	return &sprites[sprite];
}

// Gets the spriteinfo of a skin from the given animation and subanimation IDs
spriteinfo_t *P_GetSkinAnimSpriteInfo(skin_t *skin, UINT16 anim, const char *subanim_name)
{
	if (!skin)
		return NULL;

	skinspritedef_t *def = P_GetSkinSpritesForAnimation(skin, anim);
	animation_t *subanim = P_GetSubAnimationByName(P_GetAnimationByID(def->animation_id), subanim_name);
	if (!subanim)
		return NULL;
	spritenum_t sprite = def->spritenum[subanim->id];
	if (sprite == SPR_NULL)
		return NULL;

	return &spriteinfo[sprite];
}

// Checks if a skin animation is valid
boolean P_IsSkinAnimationValid(skin_t *skin, const char *subanim_name, UINT8 spriteset)
{
	spritedef_t *sprdef = P_GetSkinSpritedef(skin, subanim_name, spriteset);
	return sprdef && sprdef->numframes;
}

static void Sk_SetDefaultValue(skin_t *skin)
{
	INT32 i;
	//
	// set default skin values
	//
	memset(skin, 0, sizeof (skin_t));
	snprintf(skin->name,
		sizeof skin->name, "skin %u", (UINT32)(skin->skinnum));
	skin->name[sizeof skin->name - 1] = '\0';
	skin->wadnum = INT16_MAX;

	skin->flags = 0;

	strcpy(skin->realname, "Someone");
	strcpy(skin->hudname, "???");
	strcpy(skin->supername, "Someone super");

	skin->starttranscolor = 96;
	skin->prefcolor = SKINCOLOR_GREEN;
	skin->supercolor = SKINCOLOR_SUPERGOLD1;
	skin->prefoppositecolor = 0; // use tables

	skin->normalspeed = 36<<FRACBITS;
	skin->runspeed = 28<<FRACBITS;
	skin->thrustfactor = 5;
	skin->accelstart = 96;
	skin->acceleration = 40;

	skin->ability = CA_NONE;
	skin->ability2 = CA2_SPINDASH;
	skin->jumpfactor = FRACUNIT;
	skin->actionspd = 30<<FRACBITS;
	skin->mindash = 15<<FRACBITS;
	skin->maxdash = 70<<FRACBITS;

	skin->radius = mobjinfo[MT_PLAYER].radius;
	skin->height = mobjinfo[MT_PLAYER].height;
	skin->spinheight = FixedMul(skin->height, 2*FRACUNIT/3);

	skin->shieldscale = FRACUNIT;
	skin->camerascale = FRACUNIT;

	skin->thokitem = -1;
	skin->spinitem = -1;
	skin->revitem = -1;
	skin->followitem = 0;

	skin->highresscale = FRACUNIT;
	skin->contspeed = 17;
	skin->contangle = 0;

	skin->natkcolor = SKINCOLOR_NONE;

	for (i = 0; i < sfx_skinsoundslot0; i++)
		if (S_sfx[i].skinsound != -1)
			skin->soundsid[S_sfx[i].skinsound] = i;
}

//
// Initialize the basic skins
//
void R_InitSkins(void)
{
	// no default skin!
	numskins = 0;
}

// Creates preset player animations that skins inherit from.
void R_InitSkinAnimations(void)
{
	animation_list_t *base_anim, *super_anim, *existing_anim;
	UINT16 base_anim_id;

	base_anim = P_CreateAnimation(PLAYER_ANIMATION_NAME);

	// Create all subanimations
	for (UINT16 i = 0; i < sizeof(player_anim_names) / sizeof(*player_anim_names); i++)
	{
		if (player_anim_names[i])
		{
			P_FindOrCreateSubAnimation(base_anim, player_anim_names[i]);
		}
	}

	// Check if there is already an animation named "player"
	existing_anim = P_FindAnimation(PLAYER_ANIMATION_NAME);

	if (existing_anim == NULL)
	{
		// If there is not, just add it to the list
		P_AddAnimation(base_anim);
	}
	else
	{
		// If there is already an animation named "player", merge with that one
		base_anim = P_MergeAnimations(PLAYER_ANIMATION_NAME, base_anim, existing_anim);
	}

	// Get the ID of the newly added animation
	base_anim_id = P_GetNamedAnimationID(PLAYER_ANIMATION_NAME);

#define SET_SPEED(subanim_name, speed) P_SetSubanimationSpeed(base_anim_id, P_GetNamedSubanimationID(base_anim_id, subanim_name), speed)

	// Set default subanimation speeds
	SET_SPEED("stand", FRACUNIT / 7);
	SET_SPEED("wait", FRACUNIT / 16);
	SET_SPEED("walk", FRACUNIT / 4);
	SET_SPEED("run",  FRACUNIT / 2);
	SET_SPEED("dash", FRACUNIT / 2);
	SET_SPEED("skid", 0);
	SET_SPEED("pain", FRACUNIT / 4);
	SET_SPEED("stun", FRACUNIT / 4);
	SET_SPEED("dead", FRACUNIT / 4);
	SET_SPEED("drown", FRACUNIT / 4);
	SET_SPEED("gasp", FRACUNIT / 4);
	SET_SPEED("spring", FRACUNIT / 2);
	SET_SPEED("fall", FRACUNIT / 2);
	SET_SPEED("edge", FRACUNIT / 12);
	SET_SPEED("ride", FRACUNIT / 4);

	SET_SPEED("spindash", FRACUNIT / 2);

	SET_SPEED("fly",  FRACUNIT / 2);
	SET_SPEED("swim", FRACUNIT / 4);
	SET_SPEED("tired", FRACUNIT / 12);

	SET_SPEED("glide", FRACUNIT / 2);
	// SET_SPEED("land", FRACUNIT / 7);
	SET_SPEED("cling", FRACUNIT / 4);
	SET_SPEED("climb", FRACUNIT / 5);

	SET_SPEED("float",  FRACUNIT / 7);
	SET_SPEED("float_run", FRACUNIT / 7);

	SET_SPEED("bounce", FRACUNIT / 4);
	SET_SPEED("land", FRACUNIT / 2);

	SET_SPEED("fire", FRACUNIT / 2);

	SET_SPEED("twinspin", FRACUNIT / 2);

	SET_SPEED("melee", FRACUNIT / 2);
	SET_SPEED("melee_land", FRACUNIT / 35);

	SET_SPEED("transformation", FRACUNIT / 4);

	SET_SPEED("nights_stand", FRACUNIT / 7);
	SET_SPEED("nights_float", FRACUNIT / 7);
	SET_SPEED("nights_fly", FRACUNIT / 2);
	SET_SPEED("nights_drill", FRACUNIT / 2);
	SET_SPEED("nights_stun", FRACUNIT / 2);

	SET_SPEED("tail_0", FRACUNIT / 5);
	SET_SPEED("tail_1", FRACUNIT / 2);
	SET_SPEED("tail_2", FRACUNIT / 2);
	SET_SPEED("tail_3", FRACUNIT / 2);
	SET_SPEED("tail_4", FRACUNIT / 2);
	SET_SPEED("tail_5", FRACUNIT / 2);
	SET_SPEED("tail_6", FRACUNIT / 2);
	SET_SPEED("tail_7", FRACUNIT / 4);
	SET_SPEED("tail_8", FRACUNIT / 4);
	SET_SPEED("tail_9", FRACUNIT / 2);
	SET_SPEED("tail_a", FRACUNIT / 2);
	SET_SPEED("tail_b", FRACUNIT / 2);
	SET_SPEED("tail_c", FRACUNIT / 2);

#undef SET_SPEED

#define SET_FALLBACK(subanim_name, def_name) P_SetSubanimationFallback(base_anim_id, P_GetNamedSubanimationID(base_anim_id, subanim_name), def_name)

	// Set default subanimation fallbacks
	SET_FALLBACK("skid", "walk");
	SET_FALLBACK("run",  "walk");
	SET_FALLBACK("dash", "float_run");
	SET_FALLBACK("stun", "pain");
	SET_FALLBACK("dead", "pain");
	SET_FALLBACK("drown", "dead");
	SET_FALLBACK("gasp", "spring");
	SET_FALLBACK("spring", "fall");
	SET_FALLBACK("fall", "walk");
	SET_FALLBACK("ride", "fall");

	SET_FALLBACK("spindash", "roll");

	SET_FALLBACK("fly",  "spring");
	SET_FALLBACK("swim", "fly");

	SET_FALLBACK("glide", "fly");
	SET_FALLBACK("land", "roll");
	SET_FALLBACK("cling", "climb");
	SET_FALLBACK("climb", "roll");

	SET_FALLBACK("float",  "walk");
	SET_FALLBACK("float_run", "run");

	SET_FALLBACK("bounce", "fall");

	SET_FALLBACK("twinspin", "roll");

	SET_FALLBACK("melee", "twinspin");

	SET_FALLBACK("nights_stand", "stand");
	SET_FALLBACK("nights_float", "fall");
	SET_FALLBACK("nights_drill", "nights_fly");
	SET_FALLBACK("nights_stun", "stun");
	SET_FALLBACK("nights_pull", "nights_stun");
	SET_FALLBACK("nights_attack", "roll");

	SET_FALLBACK("tail_1", "tail_0");
	SET_FALLBACK("tail_2", "tail_1");
	SET_FALLBACK("tail_3", "tail_2");
	SET_FALLBACK("tail_4", "tail_1");
	SET_FALLBACK("tail_5", "tail_4");
	SET_FALLBACK("tail_6", "tail_0");
	SET_FALLBACK("tail_7", "tail_3");
	SET_FALLBACK("tail_8", "tail_7");
	SET_FALLBACK("tail_9", "tail_0");
	SET_FALLBACK("tail_a", "tail_9");
	SET_FALLBACK("tail_b", "tail_0");
	SET_FALLBACK("tail_c", "tail_6");

	SET_FALLBACK("continue", "wait");
	SET_FALLBACK("continue_lift", "fall");
	SET_FALLBACK("continue_spin", "spring");
	SET_FALLBACK("continue_partner", "continue");

#undef SET_FALLBACK

	base_skin_animations[SKINSPRITES_BASE] = base_anim;
	base_skin_animation_ids[SKINSPRITES_BASE] = base_anim_id;

	// Create preset player animation for the super spriteset
	// Check if there is already an animation named "player_super"
	existing_anim = P_FindAnimation(PLAYER_SUPER_ANIMATION_NAME);

	if (existing_anim == NULL)
	{
		// If there is not, duplicate "player"
		super_anim = P_DuplicateAnimation(PLAYER_SUPER_ANIMATION_NAME, base_anim, true);
	}
	else
	{
		// If there is already an animation named "player_super", merge it with "player"
		super_anim = P_MergeAnimations(PLAYER_SUPER_ANIMATION_NAME, base_anim, existing_anim);
	}

	base_skin_animations[SKINSPRITES_SUPER] = super_anim;
	base_skin_animation_ids[SKINSPRITES_SUPER] = P_GetNamedAnimationID(PLAYER_SUPER_ANIMATION_NAME);
}

UINT32 R_GetSkinAvailabilities(void)
{
	UINT32 response = 0;
	UINT32 unlockShift = 0;
	INT32 i;

	for (i = 0; i < MAXUNLOCKABLES; i++)
	{
		if (unlockables[i].type != SECRET_SKIN)
		{
			continue;
		}

		if (unlockShift >= 32)
		{
			// This crash is impossible to trigger as is,
			// but it could happen if MAXUNLOCKABLES is ever made higher than 32,
			// and someone makes a mod that has 33+ unlockable characters. :V
			// 2022/03/15: MAXUNLOCKABLES is now higher than 32
			I_Error("Too many unlockable characters! (maximum is 32)\n");
			return 0;
		}

		if (clientGamedata->unlocked[i])
		{
			response |= (1 << unlockShift);
		}

		unlockShift++;
	}

	return response;
}

// returns true if available in circumstances, otherwise nope
// warning don't use with an invalid skinnum other than -1 which always returns true
boolean R_SkinUsable(INT32 playernum, INT32 skinnum)
{
	INT32 unlockID = -1;
	UINT32 unlockShift = 0;
	INT32 i;

	if (skinnum == -1)
	{
		// Simplifies things elsewhere, since there's already plenty of checks for less-than-0...
		return true;
	}

	if (modeattacking)
	{
		// If you have someone else's run you might as well take a look
		return true;
	}

	if (Playing() && mapheaderinfo[gamemap-1] && (R_SkinAvailable(mapheaderinfo[gamemap-1]->forcecharacter) == skinnum))
	{
		// Force 1.
		return true;
	}

	if (netgame && (cv_forceskin.value == skinnum))
	{
		// Force 2.
		return true;
	}

	if (metalrecording && skinnum == 5)
	{
		// Force 3.
		return true;
	}

	if (playernum != -1 && players[playernum].bot)
	{
		// Force 4.
		return true;
	}

	// We will now check if this skin is supposed to be locked or not.

	for (i = 0; i < MAXUNLOCKABLES; i++)
	{
		INT32 unlockSkin = -1;

		if (unlockables[i].type != SECRET_SKIN)
		{
			continue;
		}

		unlockSkin = M_UnlockableSkinNum(&unlockables[i]);

		if (unlockSkin == skinnum)
		{
			unlockID = i;
			break;
		}

		unlockShift++;
	}

	if (unlockID == -1)
	{
		// This skin isn't locked at all, we're good.
		return true;
	}

	if ((netgame || multiplayer) && playernum != -1)
	{
		// We want to check per-player unlockables.
		return (players[playernum].availabilities & (1 << unlockShift));
	}
	else
	{
		// We want to check our global unlockables.
		return (clientGamedata->unlocked[unlockID]);
	}
}

// returns the skin number if the skin name is found (loaded from pwad)
// warning return -1 if not found
INT32 R_SkinAvailable(const char *name)
{
	INT32 i;

	for (i = 0; i < numskins; i++)
	{
		// search in the skin list
		if (!stricmp(skins[i]->name,name))
			return i;
	}
	return -1;
}

INT32 R_GetForcedSkin(INT32 playernum)
{
	if (netgame && cv_forceskin.value >= 0 && R_SkinUsable(playernum, cv_forceskin.value))
		return cv_forceskin.value;

	if (mapheaderinfo[gamemap-1] && mapheaderinfo[gamemap-1]->forcecharacter[0] != '\0')
	{
		INT32 skinnum = R_SkinAvailable(mapheaderinfo[gamemap-1]->forcecharacter);
		if (skinnum != -1 && R_SkinUsable(playernum, skinnum))
			return skinnum;
	}

	return -1;
}

// Auxillary function that actually sets the skin
static void SetSkin(player_t *player, INT32 skinnum)
{
	skin_t *skin = skins[skinnum];
	UINT16 newcolor = 0;

	player->skin = skinnum;

	player->camerascale = skin->camerascale;
	player->shieldscale = skin->shieldscale;

	player->charability = (UINT8)skin->ability;
	player->charability2 = (UINT8)skin->ability2;

	player->charflags = (UINT32)skin->flags;

	player->thokitem = skin->thokitem < 0 ? (UINT32)mobjinfo[MT_PLAYER].painchance : (UINT32)skin->thokitem;
	player->spinitem = skin->spinitem < 0 ? (UINT32)mobjinfo[MT_PLAYER].damage : (UINT32)skin->spinitem;
	player->revitem = skin->revitem < 0 ? (UINT32)mobjinfo[MT_PLAYER].raisestate : (UINT32)skin->revitem;
	player->followitem = skin->followitem;

	if (((player->powers[pw_shield] & SH_NOSTACK) == SH_PINK) && (player->revitem == MT_LHRT || player->spinitem == MT_LHRT || player->thokitem == MT_LHRT)) // Healers can't keep their buff.
		player->powers[pw_shield] &= SH_STACK;

	player->actionspd = skin->actionspd;
	player->mindash = skin->mindash;
	player->maxdash = skin->maxdash;

	player->normalspeed = skin->normalspeed;
	player->runspeed = skin->runspeed;
	player->thrustfactor = skin->thrustfactor;
	player->accelstart = skin->accelstart;
	player->acceleration = skin->acceleration;

	player->jumpfactor = skin->jumpfactor;

	player->height = skin->height;
	player->spinheight = skin->spinheight;

	if (!(cv_debug || devparm) && !(netgame || multiplayer || demoplayback))
	{
		player->skincolor = newcolor = skin->prefcolor;
		if (player->bot && botingame)
		{
			botskin = (UINT8)(skinnum + 1);
			botcolor = skin->prefcolor;
		}
	}

	if (player->followmobj)
	{
		P_RemoveMobj(player->followmobj);
		P_SetTarget(&player->followmobj, NULL);
	}

	if (player->mo)
	{
		fixed_t radius = FixedMul(skin->radius, player->mo->scale);
		if ((player->powers[pw_carry] == CR_NIGHTSMODE) && (!P_IsSkinAnimationValid(skin, "nights_fly", SKINSPRITES_BASE))) // If you don't have a sprite for flying horizontally, use the default NiGHTS skin.
		{
			skin = skins[DEFAULTNIGHTSSKIN];
			player->followitem = skin->followitem;
			if (!(cv_debug || devparm) && !(netgame || multiplayer || demoplayback))
				newcolor = skin->prefcolor; // will be updated in thinker to flashing
		}
		player->mo->skin = skin;
		if (newcolor)
			player->mo->color = newcolor;
		P_SetScale(player->mo, player->mo->scale, false);
		player->mo->radius = radius;

		P_SetMobjState(player->mo, player->mo->state-states); // Prevent visual errors when switching between skins with differing number of frames
	}
}

// Gets the player to the first usuable skin in the game.
// (If your mod locked them all, then you kinda stupid)
INT32 GetPlayerDefaultSkin(INT32 playernum)
{
	INT32 i;

	for (i = 0; i < numskins; i++)
	{
		if (R_SkinUsable(playernum, i))
		{
			return i;
		}
	}

	I_Error("All characters are locked!");
	return 0;
}

// network code calls this when a 'skin change' is received
void SetPlayerSkin(INT32 playernum, const char *skinname)
{
	INT32 i = R_SkinAvailable(skinname);
	player_t *player = &players[playernum];

	if ((i != -1) && R_SkinUsable(playernum, i))
	{
		SetSkin(player, i);
		return;
	}

	if (P_IsLocalPlayer(player))
		CONS_Alert(CONS_WARNING, M_GetText("Skin '%s' not found.\n"), skinname);
	else if (server || IsPlayerAdmin(consoleplayer))
		CONS_Alert(CONS_WARNING, M_GetText("Player %d (%s) skin '%s' not found\n"), playernum, player_names[playernum], skinname);

	SetSkin(player, GetPlayerDefaultSkin(playernum));
}

// Same as SetPlayerSkin, but uses the skin #.
// network code calls this when a 'skin change' is received
void SetPlayerSkinByNum(INT32 playernum, INT32 skinnum)
{
	player_t *player = &players[playernum];

	if (skinnum >= 0 && skinnum < numskins && R_SkinUsable(playernum, skinnum)) // Make sure it exists!
	{
		SetSkin(player, skinnum);
		return;
	}

	if (P_IsLocalPlayer(player))
		CONS_Alert(CONS_WARNING, M_GetText("Requested skin %d not found\n"), skinnum);
	else if (server || IsPlayerAdmin(consoleplayer))
		CONS_Alert(CONS_WARNING, "Player %d (%s) skin %d not found\n", playernum, player_names[playernum], skinnum);

	SetSkin(player, GetPlayerDefaultSkin(playernum));
}

//
// Add skins from a pwad, each skin preceded by 'S_SKIN' marker
//

// Does the same is in w_wad, but check only for
// the first 6 characters (this is so we can have S_SKIN1, S_SKIN2..
// for wad editors that don't like multiple resources of the same name)
//
static UINT16 W_CheckForSkinMarkerInPwad(UINT16 wadid, UINT16 startlump)
{
	UINT16 i;
	const char *S_SKIN = "S_SKIN";
	lumpinfo_t *lump_p;

	// scan forward, start at <startlump>
	if (startlump < wadfiles[wadid]->numlumps)
	{
		lump_p = wadfiles[wadid]->lumpinfo + startlump;
		for (i = startlump; i < wadfiles[wadid]->numlumps; i++, lump_p++)
			if (memcmp(lump_p->name,S_SKIN,6)==0)
				return i;
	}
	return INT16_MAX; // not found
}

#define HUDNAMEWRITE(value) STRBUFCPY(skin->hudname, value)

// turn _ into spaces and . into katana dot
#define SYMBOLCONVERT(name) for (value = name; *value; value++)\
					{\
						if (*value == '_') *value = ' ';\
						else if (*value == '.') *value = '\x1E';\
					}

//
// Patch skins from a pwad, each skin preceded by 'P_SKIN' marker
//

// Does the same is in w_wad, but check only for
// the first 6 characters (this is so we can have P_SKIN1, P_SKIN2..
// for wad editors that don't like multiple resources of the same name)
//
static UINT16 W_CheckForPatchSkinMarkerInPwad(UINT16 wadid, UINT16 startlump)
{
	UINT16 i;
	const char *P_SKIN = "P_SKIN";
	lumpinfo_t *lump_p;

	// scan forward, start at <startlump>
	if (startlump < wadfiles[wadid]->numlumps)
	{
		lump_p = wadfiles[wadid]->lumpinfo + startlump;
		for (i = startlump; i < wadfiles[wadid]->numlumps; i++, lump_p++)
			if (memcmp(lump_p->name,P_SKIN,6)==0)
				return i;
	}
	return INT16_MAX; // not found
}

static void InitSubanimationForSpritedef(animation_t *subanim, spritedef_t *spritedef)
{
	if (spritedef == NULL || spritedef->numframes <= subanim->num_frames)
		return;

	subanim->frames = Z_Realloc(subanim->frames, spritedef->numframes * sizeof(animation_frame_t), PU_STATIC, NULL);

	for (unsigned i = subanim->num_frames; i < spritedef->numframes; i++)
	{
		animation_frame_t *frame = &subanim->frames[i];
		P_InitAnimationFrame(frame);
		frame->frame_num = i;
	}

	subanim->num_frames = spritedef->numframes;
}

static boolean R_AddSkinSpriteDef(animation_list_t *animation, skinspritedef_t *def, const char *spritename, const char *lumpname, const char *subanimation_name, UINT16 wadnum, UINT16 startlump, UINT16 endlump)
{
	spritedef_t spritedef = { 0 };

	animation_t *subanim = P_FindOrCreateSubAnimation(animation, subanimation_name);

	if (def->spritenum[subanim->id] != SPR_NULL)
		memcpy(&spritedef, &sprites[def->spritenum[subanim->id]], sizeof(spritedef_t));

	if (R_AddSingleSpriteDef(spritename, &spritedef, wadnum, startlump, endlump, true) && spritedef.numframes > 0)
	{
		if (def->spritenum[subanim->id] == SPR_NULL)
		{
			spritenum_t sprnum = DEH_FindSpriteByName(spritename);
			if (sprnum == NUMSPRITES)
			{
				sprnum = DEH_AddSpriteName(spritename);
				if (sprnum == NUMSPRITES)
				{
					Z_Free(spritedef.spriteframes);
					return false;
				}

				set_bit_array(used_skinspr, sprnum - SPR_FIRSTFREESLOT);
			}

			def->spritenum[subanim->id] = sprnum;
		}

		spritedef_t *skin_sprdef = &sprites[def->spritenum[subanim->id]];

		memcpy(skin_sprdef, &spritedef, sizeof(spritedef_t));

		InitSubanimationForSpritedef(subanim, skin_sprdef);
	}

	return true;
}

static animation_list_t *GetSkinAnimation(const char *anim_name, UINT8 spriteset)
{
	animation_list_t *animation = P_FindAnimation(anim_name);

	if (!animation || animation->count < sizeof(player_anim_names) / sizeof(*player_anim_names))
	{
		animation_list_t *base_anim = base_skin_animations[spriteset];
		if (base_anim == NULL)
			I_Error("Missing base player animation for spriteset \"%s\"", player_spriteset_names[spriteset]);

		if (animation)
		{
			// Merge this animation with the base "player" animation
			// This also ensures that player subanimations come first
			return P_MergeAnimations(anim_name, base_anim, animation);
		}
		else
		{
			// Duplicate the base "player" animation for this skin
			return P_DuplicateAnimation(anim_name, base_anim, true);
		}
	}

	return animation;
}

static void R_LoadSkinAnimations(UINT16 wadnum, UINT16 *lump, UINT16 *lastlump, skin_t *skin)
{
	char *anim_name = NULL;
	char *spr_name = NULL;
	char *spriteset_name = NULL;
	char *subanim_name = NULL;
	char *fullname_buf = NULL;
	char sprname[5] = { 0 };
	UINT16 i = *lump;

	size_t anim_name_size = 0;
	size_t spr_name_size = 0;
	size_t spriteset_name_size = 0;
	size_t subanim_name_size = 0;
	size_t fullname_buf_size = 0;

	char *folderpath = W_GetLumpFolderPathPK3(wadnum, *lump);

	*lastlump = W_CheckNumForFolderEndPK3(folderpath, wadnum, *lump);

	i = *lump;

	while (i < *lastlump)
	{
		char *underscore;
		char *fullname, *name_p;
		UINT16 startlump, endlump;
		UINT16 anim_id;
		UINT8 spriteset;
		size_t len, name_size;
		animation_list_t *animation;

		CONS_Printf("%s line %d: iter %s\n", __func__, __LINE__, wadfiles[wadnum]->lumpinfo[i].fullname);

		// Ignore lumps that are folders
		if (W_IsLumpFolder(wadnum, i))
		{
			i++;
			continue;
		}

		fullname = wadfiles[wadnum]->lumpinfo[i].fullname;
		name_p = fullname + strlen(folderpath); // Skip parent folder of S_SKIN lump

		// Get spriteset name
		underscore = strchr(name_p, '/');
		if (underscore == NULL)
		{
			i++;
			continue;
		}

		len = underscore - name_p;
		name_size = len + 1;
		if (spriteset_name == NULL || name_size > spriteset_name_size)
		{
			spriteset_name_size = name_size;
			spriteset_name = Z_Realloc(spriteset_name, spriteset_name_size, PU_STATIC, NULL);
		}
		memcpy(spriteset_name, name_p, len);
		spriteset_name[len] = '\0';
		name_p += name_size;

		// Find or create a new spriteset
		spriteset = P_GetOrCreatePlayerSpriteset(spriteset_name);
		if (spriteset == NUMSKINSPRITESETS)
		{
			CONS_Alert(CONS_ERROR, "Could not create player spriteset \"%s\" while adding skin \"%s\" (exceeded limit)\n", spriteset_name, skin->name);
			break;
		}

		// Get subanimation name
		underscore = strchr(name_p, '/');
		if (underscore == NULL)
		{
			i++;
			continue;
		}

		len = underscore - name_p;
		name_size = len + 1;
		if (subanim_name == NULL || name_size > subanim_name_size)
		{
			subanim_name_size = name_size;
			subanim_name = Z_Realloc(subanim_name, subanim_name_size, PU_STATIC, NULL);
		}
		memcpy(subanim_name, name_p, len);
		subanim_name[len] = '\0';

		// Get path to where the sprites are located
		name_p += name_size;
		len = name_p - fullname;
		name_size = len + 1;
		if (fullname_buf == NULL || name_size > fullname_buf_size)
		{
			fullname_buf_size = name_size;
			fullname_buf = Z_Realloc(fullname_buf, fullname_buf_size, PU_STATIC, NULL);
		}
		memcpy(fullname_buf, fullname, len);
		fullname_buf[len] = '\0';

		// Find range of files where the sprites are in
		startlump = W_CheckNumForFolderStartPK3(fullname_buf, wadnum, *lump);
		endlump = W_CheckNumForFolderEndPK3(fullname_buf, wadnum, startlump);

		if (startlump == wadfiles[wadnum]->numlumps || startlump >= endlump)
		{
			i++;
			continue;
		}

		// Create combined animation name
		name_size = strlen(skin->name) + 6; // "skin_" + skin name + '/0'
		if (spriteset != SKINSPRITES_BASE)
		{
			name_size += strlen(spriteset_name) + 1; // spriteset + "_"
		}

		if (anim_name == NULL || name_size > anim_name_size)
		{
			anim_name_size = name_size;
			anim_name = Z_Realloc(anim_name, anim_name_size, PU_STATIC, NULL);
		}

		if (spriteset == SKINSPRITES_BASE)
		{
			// If this is the base spriteset, "skin_name"
			snprintf(anim_name, anim_name_size, "skin_%s", skin->name);
		}
		else
		{
			// If this is not the base spriteset, "skin_name_spriteset"
			snprintf(anim_name, anim_name_size, "skin_%s_%s", skin->name, spriteset_name);
		}

		// Get or create the animation
		animation = GetSkinAnimation(anim_name, spriteset);

		// Now get that animation's ID
		// FIXME: do this and the above in one step
		anim_id = P_GetNamedAnimationID(anim_name);
		if (anim_id == 0)
		{
			I_Error("Missing animation \"%s\" while adding skin \"%s\"", anim_name, skin->name);
		}

		skin->sprites[spriteset].animation_id = anim_id;

		// Create the sprite name
		name_size = strlen(anim_name) + strlen(subanim_name) + 2;
		if (spr_name == NULL || name_size > spr_name_size)
		{
			spr_name_size = name_size;
			spr_name = Z_Realloc(spr_name, spr_name_size, PU_STATIC, NULL);
		}
		snprintf(spr_name, spr_name_size, "%s_%s", anim_name, subanim_name);
		strupr(spr_name);

		// Finally, add the sprites
		i = endlump;

		strlcpy(sprname, wadfiles[wadnum]->lumpinfo[startlump].name, sizeof sprname);

		if (!R_AddSkinSpriteDef(animation, &skin->sprites[spriteset], spr_name, sprname, subanim_name, wadnum, startlump, endlump))
			I_Error("No more free sprite slots when adding skin animation \"%s\"", anim_name);
	}

	Z_Free(folderpath);
	Z_Free(anim_name);
	Z_Free(spr_name);
	Z_Free(spriteset_name);
	Z_Free(subanim_name);
	Z_Free(fullname_buf);
}

// returns whether found appropriate property
static boolean R_ProcessPatchableFields(skin_t *skin, char *stoken, char *value)
{
	// custom translation table
	if (!stricmp(stoken, "startcolor"))
		skin->starttranscolor = atoi(value);

#define FULLPROCESS(field) else if (!stricmp(stoken, #field)) skin->field = get_number(value);
	// character type identification
	FULLPROCESS(flags)
	FULLPROCESS(ability)
	FULLPROCESS(ability2)

	FULLPROCESS(thokitem)
	FULLPROCESS(spinitem)
	FULLPROCESS(revitem)
	FULLPROCESS(followitem)
#undef FULLPROCESS

#define GETFRACBITS(field) else if (!stricmp(stoken, #field)) skin->field = atoi(value)<<FRACBITS;
	GETFRACBITS(normalspeed)
	GETFRACBITS(runspeed)

	GETFRACBITS(mindash)
	GETFRACBITS(maxdash)
	GETFRACBITS(actionspd)

	GETFRACBITS(radius)
	GETFRACBITS(height)
	GETFRACBITS(spinheight)
#undef GETFRACBITS

#define GETINT(field) else if (!stricmp(stoken, #field)) skin->field = atoi(value);
	GETINT(thrustfactor)
	GETINT(accelstart)
	GETINT(acceleration)
	GETINT(contspeed)
	GETINT(contangle)
#undef GETINT

#define GETSKINCOLOR(field) else if (!stricmp(stoken, #field)) \
{ \
	UINT16 color = R_GetColorByName(value); \
	skin->field = (color ? color : SKINCOLOR_GREEN); \
}
	GETSKINCOLOR(prefcolor)
	GETSKINCOLOR(prefoppositecolor)
#undef GETSKINCOLOR
	else if (!stricmp(stoken, "supercolor"))
	{
		UINT16 color = R_GetSuperColorByName(value);
		skin->supercolor = (color ? color : SKINCOLOR_SUPERGOLD1);
	}
#define GETFLOAT(field) else if (!stricmp(stoken, #field)) skin->field = FLOAT_TO_FIXED(atof(value));
	GETFLOAT(jumpfactor)
	GETFLOAT(highresscale)
	GETFLOAT(shieldscale)
	GETFLOAT(camerascale)
#undef GETFLOAT

#define GETFLAG(field) else if (!stricmp(stoken, #field)) { \
	strupr(value); \
	if (atoi(value) || value[0] == 'T' || value[0] == 'Y') \
		skin->flags |= (SF_##field); \
	else \
		skin->flags &= ~(SF_##field); \
}
	// parameters for individual character flags
	// these are uppercase so they can be concatenated with SF_
	// 1, true, yes are all valid values
	GETFLAG(SUPER)
	GETFLAG(NOSUPERSPIN)
	GETFLAG(NOSPINDASHDUST)
	GETFLAG(HIRES)
	GETFLAG(NOSKID)
	GETFLAG(NOSPEEDADJUST)
	GETFLAG(RUNONWATER)
	GETFLAG(NOJUMPSPIN)
	GETFLAG(NOJUMPDAMAGE)
	GETFLAG(STOMPDAMAGE)
	GETFLAG(MARIODAMAGE)
	GETFLAG(MACHINE)
	GETFLAG(DASHMODE)
	GETFLAG(FASTEDGE)
	GETFLAG(FASTWAIT)
	GETFLAG(JETFUME)
	GETFLAG(MULTIABILITY)
	GETFLAG(NONIGHTSROTATION)
	GETFLAG(NONIGHTSSUPER)
	GETFLAG(NOSUPERSPRITES)
	GETFLAG(NOSUPERJUMPBOOST)
	GETFLAG(CANBUSTWALLS)
	GETFLAG(NOSHIELDABILITY)
#undef GETFLAG

	else if (!stricmp(stoken, "natkcolor"))
		skin->natkcolor = R_GetColorByName(value); // SKINCOLOR_NONE is allowed here

	else // let's check if it's a sound, otherwise error out
	{
		boolean found = false;
		sfxenum_t i;
		size_t stokenadjust;

		// Remove the prefix. (We need to affect an adjusting variable so that we can print error messages if it's not actually a sound.)
		if ((stoken[0] == 'D' || stoken[0] == 'd') && (stoken[1] == 'S' || stoken[1] == 's')) // DS*
			stokenadjust = 2;
		else // sfx_*
			stokenadjust = 4;

		// Remove the prefix. (We can affect this directly since we're not going to use it again.)
		if ((value[0] == 'D' || value[0] == 'd') && (value[1] == 'S' || value[1] == 's')) // DS*
			value += 2;
		else // sfx_*
			value += 4;

		// copy name of sounds that are remapped
		// for this skin
		for (i = 0; i < sfx_skinsoundslot0; i++)
		{
			if (!S_sfx[i].name)
				continue;
			if (S_sfx[i].skinsound != -1
				&& !stricmp(S_sfx[i].name,
					stoken + stokenadjust))
			{
				skin->soundsid[S_sfx[i].skinsound] =
					S_AddSoundFx(value, S_sfx[i].singularity, S_sfx[i].pitch, true);
				found = true;
			}
		}
		return found;
	}
	return true;
}

// e.g. "RUN" => S_PLAY_RUN, "RUN1" => S_PLAY_RUN, "RUN8" => S_PLAY_RUN
static statenum_t GetCanonicalPlayerStateNumByName(const char *name)
{
	if (startswith(name, "STAND"))               return S_PLAY_STND;
	else if (startswith(name, "STND"))           return S_PLAY_STND;
	else if (startswith(name, "WAIT"))           return S_PLAY_WAIT;
	else if (startswith(name, "WALK"))           return S_PLAY_WALK;
	else if (startswith(name, "SKID"))           return S_PLAY_SKID;
	else if (startswith(name, "RUN"))            return S_PLAY_RUN;
	else if (startswith(name, "DASH"))           return S_PLAY_DASH;
	else if (startswith(name, "PAIN"))           return S_PLAY_PAIN;
	else if (startswith(name, "STUN"))           return S_PLAY_STUN;
	else if (startswith(name, "DEAD"))           return S_PLAY_DEAD;
	else if (startswith(name, "DRWN"))           return S_PLAY_DRWN;
	else if (startswith(name, "ROLL"))           return S_PLAY_ROLL;
	else if (startswith(name, "GASP"))           return S_PLAY_GASP;
	else if (startswith(name, "JUMP"))           return S_PLAY_JUMP;
	else if (startswith(name, "SPRING"))         return S_PLAY_SPRING;
	else if (startswith(name, "FALL"))           return S_PLAY_FALL;
	else if (startswith(name, "EDGE"))           return S_PLAY_EDGE;
	else if (startswith(name, "RIDE"))           return S_PLAY_RIDE;
	else if (startswith(name, "SPINDASH"))       return S_PLAY_SPINDASH;
	else if (startswith(name, "FLY"))            return S_PLAY_FLY;
	else if (startswith(name, "SWIM"))           return S_PLAY_SWIM;
	else if (startswith(name, "FLY_TIRED"))      return S_PLAY_FLY_TIRED;
	else if (startswith(name, "GLIDE"))          return S_PLAY_GLIDE;
	else if (startswith(name, "GLIDE_LANDING"))  return S_PLAY_GLIDE_LANDING;
	else if (startswith(name, "CLING"))          return S_PLAY_CLING;
	else if (startswith(name, "CLIMB"))          return S_PLAY_CLIMB;
	else if (startswith(name, "FLOAT"))          return S_PLAY_FLOAT;
	else if (startswith(name, "FLOAT_RUN"))      return S_PLAY_FLOAT_RUN;
	else if (startswith(name, "BOUNCE"))         return S_PLAY_BOUNCE;
	else if (startswith(name, "BOUNCE_LANDING")) return S_PLAY_BOUNCE_LANDING;
	else if (startswith(name, "FIRE"))           return S_PLAY_FIRE;
	else if (startswith(name, "FIRE_FINISH"))    return S_PLAY_FIRE_FINISH;
	else if (startswith(name, "TWINSPIN"))       return S_PLAY_TWINSPIN;
	else if (startswith(name, "MELEE"))          return S_PLAY_MELEE;
	else if (startswith(name, "MELEE_FINISH"))   return S_PLAY_MELEE_FINISH;
	else if (startswith(name, "MELEE_LANDING"))  return S_PLAY_MELEE_LANDING;
	else if (startswith(name, "NIGHTS_STAND"))   return S_PLAY_NIGHTS_STAND;
	else if (startswith(name, "NIGHTS_FLOAT"))   return S_PLAY_NIGHTS_FLOAT;
	else if (startswith(name, "NIGHTS_FLY"))     return S_PLAY_NIGHTS_FLY;
	else if (startswith(name, "NIGHTS_DRILL"))   return S_PLAY_NIGHTS_DRILL;
	else if (startswith(name, "NIGHTS_STUN"))    return S_PLAY_NIGHTS_STUN;
	else if (startswith(name, "NIGHTS_PULL"))    return S_PLAY_NIGHTS_PULL;
	else if (startswith(name, "NIGHTS_ATTACK"))  return S_PLAY_NIGHTS_ATTACK;
	else return S_NULL;
}

static void CacheCustomSkinStates(skin_t *skin)
{
	SIMPLEHASH_CLEAR(skin->defaulttocustomstate, hashentry_int32_int32_t)
	SIMPLEHASH_CLEAR(skin->customtodefaultstate, hashentry_int32_int32_t)

	char *skinstateprefix = va("SKIN_%s_", skin->name);
	strupr(skinstateprefix);
	size_t skinstateprefixlen = strlen(skinstateprefix);

	for (INT32 state = 0; state < NUMSTATES; state++)
	{
		const char *statename = FREE_STATES[state];

		if (!statename)
			continue;
		if (strncmp(statename, skinstateprefix, skinstateprefixlen))
			continue;

		statenum_t defaultstate = GetCanonicalPlayerStateNumByName(&statename[skinstateprefixlen]);
		if (defaultstate)
		{
			// If a default state is overriden by multiple custom states, use the first one as reference
			// e.g. WALK+WALK2+WALK3+WALK4 instead of just WALK
			statenum_t alreadyoverriden;
			SIMPLEHASH_FIND_INT(skin->defaulttocustomstate, hashentry_int32_int32_t, defaultstate, S_NULL, alreadyoverriden)
			if (!alreadyoverriden)
				SIMPLEHASH_REPLACE_INT(skin->defaulttocustomstate, hashentry_int32_int32_t, defaultstate, state)

			SIMPLEHASH_REPLACE_INT(skin->customtodefaultstate, hashentry_int32_int32_t, state, defaultstate)
		}
	}
}

//
// Find skin sprites, sounds & optional status bar face, & add them
//
void R_AddSkins(UINT16 wadnum, boolean mainfile)
{
	UINT16 lump, lastlump = 0;
	char *buf;
	char *buf2;
	char *stoken;
	char *value;
	size_t size;
	skin_t *skin;
	boolean hudname, realname, supername;

	//
	// search for all skin markers in pwad
	//

	while ((lump = W_CheckForSkinMarkerInPwad(wadnum, lastlump)) != INT16_MAX)
	{
		// advance by default
		lastlump = lump + 1;

		if (numskins >= MAXSKINS)
		{
			CONS_Debug(DBG_RENDER, "ignored skin (%d skins maximum)\n", MAXSKINS);
			continue; // so we know how many skins couldn't be added
		}
		buf = W_CacheLumpNumPwad(wadnum, lump, PU_CACHE);
		size = W_LumpLengthPwad(wadnum, lump);

		// for strtok
		buf2 = malloc(size+1);
		if (!buf2)
			I_Error("R_AddSkins: No more free memory\n");
		M_Memcpy(buf2,buf,size);
		buf2[size] = '\0';

		// set defaults
		skins = Z_Realloc(skins, sizeof(skin_t*) * (numskins + 1), PU_STATIC, NULL);
		skin = skins[numskins] = Z_Calloc(sizeof(skin_t), PU_STATIC, NULL);
		Sk_SetDefaultValue(skin);
		skin->skinnum = numskins;
		skin->wadnum = wadnum;
		hudname = realname = supername = false;
		// parse
		stoken = strtok (buf2, "\r\n= ");
		while (stoken)
		{
			if ((stoken[0] == '/' && stoken[1] == '/')
				|| (stoken[0] == '#'))// skip comments
			{
				stoken = strtok(NULL, "\r\n"); // skip end of line
				goto next_token;              // find the real next token
			}

			value = strtok(NULL, "\r\n= ");

			if (!value)
				I_Error("R_AddSkins: syntax error in S_SKIN lump# %d(%s) in WAD %s\n", lump, W_CheckNameForNumPwad(wadnum,lump), wadfiles[wadnum]->filename);

			// Some of these can't go in R_ProcessPatchableFields because they have side effects for future lines.
			// Others can't go in there because we don't want them to be patchable.
			if (!stricmp(stoken, "name"))
			{
				INT32 skinnum = R_SkinAvailable(value);
				strlwr(value);
				if (skinnum == -1)
					STRBUFCPY(skin->name, value);
				// the skin name must uniquely identify a single skin
				// if the name is already used I make the name 'namex'
				// using the default skin name's number set above
				else
				{
					const size_t stringspace =
						strlen(value) + sizeof (numskins) + 1;
					char *value2 = Z_Malloc(stringspace, PU_STATIC, NULL);
					snprintf(value2, stringspace,
						"%s%d", value, numskins);
					value2[stringspace - 1] = '\0';
					if (R_SkinAvailable(value2) == -1)
						// I'm lazy so if NEW name is already used I leave the 'skin x'
						// default skin name set in Sk_SetDefaultValue
						STRBUFCPY(skin->name, value2);
					Z_Free(value2);
				}

				// copy to hudname, realname, and supername as a default.
				if (!realname)
				{
					STRBUFCPY(skin->realname, skin->name);
					for (value = skin->realname; *value; value++)
					{
						if (*value == '_') *value = ' '; // turn _ into spaces.
						else if (*value == '.') *value = '\x1E'; // turn . into katana dot.
					}
				}
				if (!hudname)
				{
					HUDNAMEWRITE(skin->name);
					strupr(skin->hudname);
					SYMBOLCONVERT(skin->hudname)
				}
				if (!supername)
				{
					char superstring[SKINNAMESIZE+7];
					strcpy(superstring, "Super ");
					strlcat(superstring, skin->name, sizeof(superstring));
					STRBUFCPY(skin->supername, superstring);
				}
			}
			else if (!stricmp(stoken, "supername"))
			{ // Super name (eg. "Super Knuckles")
				supername = true;
				STRBUFCPY(skin->supername, value);
				SYMBOLCONVERT(skin->supername)
			}
			else if (!stricmp(stoken, "realname"))
			{ // Display name (eg. "Knuckles")
				realname = true;
				STRBUFCPY(skin->realname, value);
				SYMBOLCONVERT(skin->realname)
				if (!hudname)
					HUDNAMEWRITE(skin->realname);
				if (!supername) //copy over default to capitalise the name
				{
					char superstring[SKINNAMESIZE+7];
					strcpy(superstring, "Super ");
					strlcat(superstring, skin->realname, sizeof(superstring));
					STRBUFCPY(skin->supername, superstring);
				}
			}
			else if (!stricmp(stoken, "hudname"))
			{ // Life icon name (eg. "K.T.E")
				hudname = true;
				HUDNAMEWRITE(value);
				SYMBOLCONVERT(skin->hudname)
				if (!realname)
					STRBUFCPY(skin->realname, skin->hudname);
			}
			else if (!R_ProcessPatchableFields(skin, stoken, value))
				CONS_Debug(DBG_SETUP, "R_AddSkins: Unknown keyword '%s' in S_SKIN lump #%d (WAD %s)\n", stoken, lump, wadfiles[wadnum]->filename);

next_token:
			stoken = strtok(NULL, "\r\n= ");
		}
		free(buf2);

		// Add sprites
		if (W_FileHasFolders(wadfiles[wadnum]))
			R_LoadSkinAnimations(wadnum, &lump, &lastlump, skin);

		R_FlushTranslationColormapCache();

		CacheCustomSkinStates(skin);

		if (mainfile == false)
			CONS_Printf(M_GetText("Added skin '%s'\n"), skin->name);

		numskins++;
	}
	return;
}

//
// Patch skin sprites
//
void R_PatchSkins(UINT16 wadnum, boolean mainfile)
{
	UINT16 lump, lastlump = 0;
	char *buf;
	char *buf2;
	char *stoken;
	char *value;
	size_t size;
	skin_t *skin;
	boolean noskincomplain, realname, hudname, supername;

	//
	// search for all skin patch markers in pwad
	//

	while ((lump = W_CheckForPatchSkinMarkerInPwad(wadnum, lastlump)) != INT16_MAX)
	{
		INT32 skinnum = 0;

		// advance by default
		lastlump = lump + 1;

		buf = W_CacheLumpNumPwad(wadnum, lump, PU_CACHE);
		size = W_LumpLengthPwad(wadnum, lump);

		// for strtok
		buf2 = malloc(size+1);
		if (!buf2)
			I_Error("R_PatchSkins: No more free memory\n");
		M_Memcpy(buf2,buf,size);
		buf2[size] = '\0';

		skin = NULL;
		noskincomplain = realname = hudname = supername = false;

		/*
		Parse. Has more phases than the parser in R_AddSkins because it needs to have the patching name first (no default skin name is acceptible for patching, unlike skin creation)
		*/

		stoken = strtok(buf2, "\r\n= ");
		while (stoken)
		{
			if ((stoken[0] == '/' && stoken[1] == '/')
				|| (stoken[0] == '#'))// skip comments
			{
				stoken = strtok(NULL, "\r\n"); // skip end of line
				goto next_token;              // find the real next token
			}

			value = strtok(NULL, "\r\n= ");

			if (!value)
				I_Error("R_PatchSkins: syntax error in P_SKIN lump# %d(%s) in WAD %s\n", lump, W_CheckNameForNumPwad(wadnum,lump), wadfiles[wadnum]->filename);

			if (!skin) // Get the name!
			{
				if (!stricmp(stoken, "name"))
				{
					strlwr(value);
					skinnum = R_SkinAvailable(value);
					if (skinnum != -1)
						skin = skins[skinnum];
					else
					{
						CONS_Debug(DBG_SETUP, "R_PatchSkins: unknown skin name in P_SKIN lump# %d(%s) in WAD %s\n", lump, W_CheckNameForNumPwad(wadnum,lump), wadfiles[wadnum]->filename);
						noskincomplain = true;
					}
				}
			}
			else // Get the properties!
			{
				// Some of these can't go in R_ProcessPatchableFields because they have side effects for future lines.
				if (!stricmp(stoken, "supername"))
				{ // Super name (eg. "Super Knuckles")
					supername = true;
					STRBUFCPY(skin->supername, value);
					SYMBOLCONVERT(skin->supername)
				}
				else if (!stricmp(stoken, "realname"))
				{ // Display name (eg. "Knuckles")
					realname = true;
					STRBUFCPY(skin->realname, value);
					SYMBOLCONVERT(skin->realname)
					if (!hudname)
						HUDNAMEWRITE(skin->realname);
					if (!supername) //copy over default to capitalise the name
					{
						char superstring[SKINNAMESIZE+7];
						strcpy(superstring, "Super ");
						strlcat(superstring, skin->realname, sizeof(superstring));
						STRBUFCPY(skin->supername, superstring);
					}
				}
				else if (!stricmp(stoken, "hudname"))
				{ // Life icon name (eg. "K.T.E")
					hudname = true;
					HUDNAMEWRITE(value);
					SYMBOLCONVERT(skin->hudname)
					if (!realname)
						STRBUFCPY(skin->realname, skin->hudname);
				}
				else if (!R_ProcessPatchableFields(skin, stoken, value))
					CONS_Debug(DBG_SETUP, "R_PatchSkins: Unknown keyword '%s' in P_SKIN lump #%d (WAD %s)\n", stoken, lump, wadfiles[wadnum]->filename);
			}

			if (!skin)
				break;

next_token:
			stoken = strtok(NULL, "\r\n= ");
		}
		free(buf2);

		if (!skin) // Didn't include a name parameter? What a waste.
		{
			if (!noskincomplain)
				CONS_Debug(DBG_SETUP, "R_PatchSkins: no skin name given in P_SKIN lump #%d (WAD %s)\n", lump, wadfiles[wadnum]->filename);
			continue;
		}

		// Patch sprites
		if (W_FileHasFolders(wadfiles[wadnum]))
			R_LoadSkinAnimations(wadnum, &lump, &lastlump, skin);

		R_FlushTranslationColormapCache();

		CacheCustomSkinStates(skin);

		if (mainfile == false)
			CONS_Printf(M_GetText("Patched skin '%s'\n"), skin->name);
	}
	return;
}

#undef HUDNAMEWRITE
#undef SYMBOLCONVERT

static UINT16 W_CheckForEitherSkinMarkerInPwad(UINT16 wadid, UINT16 startlump)
{
	UINT16 i;
	const char *S_SKIN = "S_SKIN";
	const char *P_SKIN = "P_SKIN";
	lumpinfo_t *lump_p;

	// scan forward, start at <startlump>
	if (startlump < wadfiles[wadid]->numlumps)
	{
		lump_p = wadfiles[wadid]->lumpinfo + startlump;
		for (i = startlump; i < wadfiles[wadid]->numlumps; i++, lump_p++)
			if (memcmp(lump_p->name,S_SKIN,6)==0 || memcmp(lump_p->name,P_SKIN,6)==0)
				return i;
	}
	return INT16_MAX; // not found
}
