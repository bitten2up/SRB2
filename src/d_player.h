// SONIC ROBO BLAST 2
//-----------------------------------------------------------------------------
// Copyright (C) 1993-1996 by id Software, Inc.
// Copyright (C) 1998-2000 by DooM Legacy Team.
// Copyright (C) 1999-2016 by Sonic Team Junior.
//
// This program is free software distributed under the
// terms of the GNU General Public License, version 2.
// See the 'LICENSE' file for more details.
//-----------------------------------------------------------------------------
/// \file  d_player.h
/// \brief player data structures

#ifndef __D_PLAYER__
#define __D_PLAYER__

// The player data structure depends on a number
// of other structs: items (internal inventory),
// animation states (closely tied to the sprites
// used to represent them, unfortunately).
#include "p_pspr.h"

// In addition, the player is just a special
// case of the generic moving object/actor.
#include "p_mobj.h"

// Finally, for odd reasons, the player input
// is buffered within the player data struct,
// as commands per game tick.
#include "d_ticcmd.h"

// Extra abilities/settings for skins (combinable stuff)
typedef enum
{
	SF_SUPER            = 1,    // Can turn super in singleplayer/co-op mode?
	SF_NOSUPERSPIN      = 1<<1, // Should spin frames be played while super?
	SF_NOSPINDASHDUST   = 1<<2, // Spawn dust particles when charging a spindash?
	SF_HIRES            = 1<<3, // Draw the sprite at different size?
	SF_NOSKID           = 1<<4, // No skid particles etc
	SF_NOSPEEDADJUST    = 1<<5, // Skin-specific version of disablespeedadjust
	SF_RUNONWATER       = 1<<6, // Run on top of water FOFs?
	SF_NOJUMPSPIN       = 1<<7, // SPR2_JUMP defaults to SPR2_SPRG instead of SPR2_ROLL, falling states used, and player height is full when jumping?
	SF_NOJUMPDAMAGE     = 1<<8, // Don't damage enemies, etc whilst jumping?
	SF_STOMPDAMAGE      = 1<<9, // Always damage enemies, etc by landing on them, no matter your vunerability?
	SF_MARIODAMAGE      = SF_NOJUMPDAMAGE|SF_STOMPDAMAGE, // The Mario method of being able to damage enemies, etc.
	SF_MACHINE          = 1<<10, // Beep boop. Are you a robot?
	SF_DASHMODE         = 1<<11, // Sonic Advance 2 style top speed increase?
	// free up to and including 1<<31
} skinflags_t;

//Primary and secondary skin abilities
typedef enum
{
	CA_NONE=0,
	CA_THOK,
	CA_FLY,
	CA_GLIDEANDCLIMB,
	CA_HOMINGTHOK,
	CA_SWIM,
	CA_DOUBLEJUMP,
	CA_FLOAT,
	CA_SLOWFALL,
	CA_TELEKINESIS,
	CA_FALLSWITCH,
	CA_JUMPBOOST,
	CA_AIRDRILL,
	CA_JUMPTHOK,
	CA_BOUNCE,
	CA_TWINSPIN
} charability_t;

//Secondary skin abilities
typedef enum
{
	CA2_NONE=0,
	CA2_SPINDASH,
	CA2_MULTIABILITY,
	CA2_GUNSLINGER,
	CA2_MELEE
} charability2_t;

//
// Player states.
//
typedef enum
{
	// Playing or camping.
	PST_LIVE,
	// Dead on the ground, view follows killer.
	PST_DEAD,
	// Ready to restart/respawn???
	PST_REBORN
} playerstate_t;

//
// Player internal flags
//
typedef enum
{
	// Flip camera angle with gravity flip prefrence.
	PF_FLIPCAM = 1,

	// Cheats
	PF_GODMODE = 1<<1,
	PF_NOCLIP  = 1<<2,
	PF_INVIS   = 1<<3,

	// True if button down last tic.
	PF_ATTACKDOWN = 1<<4,
	PF_USEDOWN    = 1<<5,
	PF_JUMPDOWN   = 1<<6,
	PF_WPNDOWN    = 1<<7,

	// Unmoving states
	PF_STASIS     = 1<<8, // Player is not allowed to move
	PF_JUMPSTASIS = 1<<9, // and that includes jumping.
	PF_FULLSTASIS = PF_STASIS|PF_JUMPSTASIS,

	// Did you get a time-over?
	PF_TIMEOVER = 1<<10,

	PF_TEMPSLOT1 = 1<<11,

	// Character action status
	PF_JUMPED    = 1<<12,
	PF_SPINNING  = 1<<13,
	PF_STARTDASH = 1<<14,
	PF_THOKKED   = 1<<15,

	// Are you gliding?
	PF_GLIDING   = 1<<16,

	// Sliding (usually in water) like Labyrinth/Oil Ocean
	PF_SLIDING   = 1<<17,

	PF_TEMPSLOT2         = 1<<18,

	/*** NIGHTS STUFF ***/
	PF_TRANSFERTOCLOSEST = 1<<19,
	PF_NIGHTSFALL        = 1<<20,
	PF_DRILLING          = 1<<21,
	PF_SKIDDOWN          = 1<<22,

	/*** TAG STUFF ***/
	PF_TAGGED            = 1<<23, // Player has been tagged and awaits the next round in hide and seek.
	PF_TAGIT             = 1<<24, // The player is it! For Tag Mode

	/*** misc ***/
	PF_FORCESTRAFE       = 1<<25, // Turning inputs are translated into strafing inputs
	PF_ANALOGMODE        = 1<<26, // Analog mode?

	// Can carry another player?
	PF_CANCARRY          = 1<<27,

	// Used shield ability
	PF_SHIELDABILITY     = 1<<28,

	// Jump damage?
	PF_NOJUMPDAMAGE   = 1<<29,

	// Bouncing
	PF_BOUNCING          = 1<<30

	// 1<<31 is free
} pflags_t;

typedef enum
{
	// Are animation frames playing?
	PA_ETC=0,
	PA_IDLE,
	PA_EDGE,
	PA_WALK,
	PA_RUN,
	PA_DASH,
	PA_PAIN,
	PA_ROLL,
	PA_JUMP,
	PA_SPRING,
	PA_FALL,
	PA_ABILITY,
	PA_ABILITY2,
	PA_RIDE
} panim_t;

typedef enum
{
	SH_NONE = 0,

	// Shield flags
	SH_PROTECTFIRE = 0x400,
	SH_PROTECTWATER = 0x800,
	SH_PROTECTELECTRIC = 0x1000,

	// Indivisible shields
	SH_PITY = 1, // the world's most basic shield ever, given to players who suck at Match
	SH_WHIRLWIND,
	SH_ARMAGEDDON,

	// normal shields that use flags
	SH_ATTRACT = SH_PROTECTELECTRIC,
	SH_ELEMENTAL = SH_PROTECTFIRE|SH_PROTECTWATER,

	// Sonic 3 shields
	SH_FLAMEAURA = SH_PROTECTFIRE,
	SH_BUBBLEWRAP = SH_PROTECTWATER,
	SH_THUNDERCOIN = SH_WHIRLWIND|SH_PROTECTELECTRIC,

	// The force shield uses the lower 8 bits to count how many extra hits are left.
	SH_FORCE = 0x100,
	SH_FORCEHP = 0xFF, // to be used as a bitmask only

	// Mostly for use with Mario mode.
	SH_FIREFLOWER = 0x200,

	SH_STACK = SH_FIREFLOWER, // second-layer shields
	SH_NOSTACK = ~SH_STACK
} shieldtype_t; // pw_shield

typedef enum
{
	CR_NONE = 0,
	// The generic case is suitable for most objects.
	CR_GENERIC,
	// Tails carry.
	CR_PLAYER,
	// NiGHTS mode. Not technically a CARRYING, but doesn't stack with any of the others, so might as well go here.
	CR_NIGHTSMODE,
	// Old Brak sucks hard, but this gimmick could be used for something better, so we might as well continue supporting it.
	CR_BRAKGOOP,
	// Specific level gimmicks.
	CR_ZOOMTUBE,
	CR_ROPEHANG,
	CR_MACESPIN
} carrytype_t; // pw_carry

// Player powers. (don't edit this comment)
typedef enum
{
	pw_invulnerability,
	pw_sneakers,
	pw_flashing,
	pw_shield,
	pw_carry,
	pw_tailsfly, // tails flying
	pw_underwater, // underwater timer
	pw_spacetime, // In space, no one can hear you spin!
	pw_extralife, // Extra Life timer

	pw_super, // Are you super?
	pw_gravityboots, // gravity boots

	// Weapon ammunition
	pw_infinityring,
	pw_automaticring,
	pw_bouncering,
	pw_scatterring,
	pw_grenadering,
	pw_explosionring,
	pw_railring,

	// Power Stones
	pw_emeralds, // stored like global 'emeralds' variable

	// NiGHTS powerups
	pw_nights_superloop,
	pw_nights_helper,
	pw_nights_linkfreeze,

	pw_nocontrol, //for linedef exec 427

	NUMPOWERS
} powertype_t;

#define WEP_AUTO    1
#define WEP_BOUNCE  2
#define WEP_SCATTER 3
#define WEP_GRENADE 4
#define WEP_EXPLODE 5
#define WEP_RAIL    6
#define NUM_WEAPONS 7

typedef enum
{
	RW_AUTO    =  1,
	RW_BOUNCE  =  2,
	RW_SCATTER =  4,
	RW_GRENADE =  8,
	RW_EXPLODE = 16,
	RW_RAIL    = 32
} ringweapons_t;

// ========================================================================
//                          PLAYER STRUCTURE
// ========================================================================
typedef struct player_s
{
	mobj_t *mo;

	// Caveat: ticcmd_t is ATTRPACK! Be careful what precedes it.
	ticcmd_t cmd;

	playerstate_t playerstate;

	// Determine POV, including viewpoint bobbing during movement.
	fixed_t camerascale;
	fixed_t shieldscale;
	// Focal origin above r.z
	fixed_t viewz;
	// Base height above floor for viewz.
	fixed_t viewheight;
	// Bob/squat speed.
	fixed_t deltaviewheight;
	// bounded/scaled total momentum.
	fixed_t bob;

	// Mouse aiming, where the guy is looking at!
	// It is updated with cmd->aiming.
	angle_t aiming;

	// player's ring count
	INT32 rings;

	SINT8 pity; // i pity the fool.
	INT32 currentweapon; // current weapon selected.
	INT32 ringweapons; // weapons currently obtained.

	// Power ups. invinc and invis are tic counters.
	UINT16 powers[NUMPOWERS];

	// Bit flags.
	// See pflags_t, above.
	pflags_t pflags;

	// playing animation.
	panim_t panim;

	// For screen flashing (bright).
	UINT16 flashcount;
	UINT16 flashpal;

	// Player skin colorshift, 0-15 for which color to draw player.
	UINT8 skincolor;

	INT32 skin;
	UINT32 availabilities;

	UINT32 score; // player score
	fixed_t dashspeed; // dashing speed

	fixed_t normalspeed; // Normal ground
	fixed_t runspeed; // Speed you break into the run animation
	UINT8 thrustfactor; // Thrust = thrustfactor * acceleration
	UINT8 accelstart; // Starting acceleration if speed = 0.
	UINT8 acceleration; // Acceleration

	// See charability_t and charability2_t for more information.
	UINT8 charability; // Ability definition
	UINT8 charability2; // Secondary ability definition

	UINT32 charflags; // Extra abilities/settings for skins (combinable stuff)
	                 // See SF_ flags

	mobjtype_t thokitem; // Object # to spawn for the thok
	mobjtype_t spinitem; // Object # to spawn for spindash/spinning
	mobjtype_t revitem; // Object # to spawn for spindash/spinning

	fixed_t actionspd; // Speed of thok/glide/fly
	fixed_t mindash; // Minimum spindash speed
	fixed_t maxdash; // Maximum spindash speed

	fixed_t jumpfactor; // How high can the player jump?

	fixed_t height; // Bounding box changes.
	fixed_t spinheight;

	SINT8 lives;
	SINT8 continues; // continues that player has acquired

	SINT8 xtralife; // Ring Extra Life counter
	UINT8 gotcontinue; // Got continue from this stage?

	fixed_t speed; // Player's speed (distance formula of MOMX and MOMY values)
	UINT8 jumping; // Holding down jump button
	UINT8 secondjump; // Jump counter

	UINT8 fly1; // Tails flying
	UINT8 scoreadd; // Used for multiple enemy attack bonus
	tic_t glidetime; // Glide counter for thrust
	UINT8 climbing; // Climbing on the wall
	INT32 deadtimer; // End game if game over lasts too long
	tic_t exiting; // Exitlevel timer

	UINT8 homing; // Are you homing?
	tic_t dashmode; // counter for dashmode ability

	tic_t skidtime; // Skid timer

	////////////////////////////
	// Conveyor Belt Movement //
	////////////////////////////
	fixed_t cmomx; // Conveyor momx
	fixed_t cmomy; // Conveyor momy
	fixed_t rmomx; // "Real" momx (momx - cmomx)
	fixed_t rmomy; // "Real" momy (momy - cmomy)

	/////////////////////
	// Race Mode Stuff //
	/////////////////////
	INT16 numboxes; // Number of item boxes obtained for Race Mode
	INT16 totalring; // Total number of rings obtained for Race Mode
	tic_t realtime; // integer replacement for leveltime
	UINT8 laps; // Number of laps (optional)

	////////////////////
	// CTF Mode Stuff //
	////////////////////
	INT32 ctfteam; // 0 == Spectator, 1 == Red, 2 == Blue
	UINT16 gotflag; // 1 == Red, 2 == Blue Do you have the flag?

	INT32 weapondelay; // Delay (if any) to fire the weapon again
	INT32 tossdelay;   // Delay (if any) to toss a flag/emeralds again

	// Starpost information
	INT16 starpostx;
	INT16 starposty;
	INT16 starpostz;
	INT32 starpostnum; // The number of the last starpost you hit
	tic_t starposttime; // Your time when you hit the starpost
	angle_t starpostangle; // Angle that the starpost is facing - you respawn facing this way

	/////////////////
	// NiGHTS Stuff//
	/////////////////
	angle_t angle_pos;
	angle_t old_angle_pos;

	mobj_t *axis1;
	mobj_t *axis2;
	tic_t bumpertime; // Currently being bounced by MT_NIGHTSBUMPER
	INT32 flyangle;
	tic_t drilltimer;
	INT32 linkcount;
	tic_t linktimer;
	INT32 anotherflyangle;
	tic_t nightstime; // How long you can fly as NiGHTS.
	INT32 drillmeter;
	UINT8 drilldelay;
	boolean bonustime; // Capsule destroyed, now it's bonus time!
	mobj_t *capsule; // Go inside the capsule
	UINT8 mare; // Current mare

	// Statistical purposes.
	tic_t marebegunat; // Leveltime when mare begun
	tic_t startedtime; // Time which you started this mare with.
	tic_t finishedtime; // Time it took you to finish the mare (used for display)
	INT16 finishedrings; // The rings you had left upon finishing the mare
	UINT32 marescore; // score for this nights stage
	UINT32 lastmarescore; // score for the last mare
	UINT8 lastmare; // previous mare
	INT32 maxlink; // maximum link obtained
	UINT8 texttimer; // nights_texttime should not be local
	UINT8 textvar; // which line of NiGHTS text to show -- let's not use cheap hacks

	INT16 lastsidehit, lastlinehit;

	tic_t losstime;
	UINT8 timeshit; // That's TIMES HIT, not TIME SHIT, you doofus!

	INT32 onconveyor; // You are on a conveyor belt if nonzero

	mobj_t *awayviewmobj;
	INT32 awayviewtics;
	angle_t awayviewaiming; // Used for cut-away view

	boolean spectator;
	UINT8 bot;

	tic_t jointime; // Timer when player joins game to change skin/color
#ifdef HWRENDER
	fixed_t fovadd; // adjust FOV for hw rendering
#endif
} player_t;

#endif
