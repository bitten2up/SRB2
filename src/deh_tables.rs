use ::libc;
extern "C" {
    fn A_Repeat();
    fn A_SuperSneakers();
    fn A_BunnyHop();
    fn A_BubbleSpawn();
    fn A_FanBubbleSpawn();
    fn A_BubbleRise();
    fn A_BubbleCheck();
    fn A_AwardScore();
    fn A_ExtraLife();
    fn A_GiveShield();
    fn A_GravityBox();
    fn A_ScoreRise();
    fn A_AttractChase();
    fn A_DropMine();
    fn A_FishJump();
    fn A_ThrownRing();
    fn A_SetSolidSteam();
    fn A_UnsetSolidSteam();
    fn A_SignSpin();
    fn A_SignPlayer();
    fn A_OverlayThink();
    fn A_JetChase();
    fn A_JetbThink();
    fn A_JetgThink();
    fn A_JetgShoot();
    fn A_ShootBullet();
    fn A_MinusDigging();
    fn A_MinusPopup();
    fn A_MinusCheck();
    fn A_ChickenCheck();
    fn A_MouseThink();
    fn A_DetonChase();
    fn A_CapeChase();
    fn A_RotateSpikeBall();
    fn A_SlingAppear();
    fn A_UnidusBall();
    fn A_RockSpawn();
    fn A_SetFuse();
    fn A_CrawlaCommanderThink();
    fn A_SmokeTrailer();
    fn A_RingExplode();
    fn A_OldRingExplode();
    fn A_MixUp();
    fn A_RecyclePowers();
    fn A_BossScream();
    fn A_Boss2TakeDamage();
    fn A_GoopSplat();
    fn A_Boss2PogoSFX();
    fn A_Boss2PogoTarget();
    fn A_EggmanBox();
    fn A_TurretFire();
    fn A_SuperTurretFire();
    fn A_TurretStop();
    fn A_JetJawRoam();
    fn A_JetJawChomp();
    fn A_PointyThink();
    fn A_CheckBuddy();
    fn A_HoodFire();
    fn A_HoodThink();
    fn A_HoodFall();
    fn A_ArrowBonks();
    fn A_SnailerThink();
    fn A_SharpChase();
    fn A_SharpSpin();
    fn A_SharpDecel();
    fn A_CrushstaceanWalk();
    fn A_CrushstaceanPunch();
    fn A_CrushclawAim();
    fn A_CrushclawLaunch();
    fn A_VultureVtol();
    fn A_VultureCheck();
    fn A_VultureHover();
    fn A_VultureBlast();
    fn A_VultureFly();
    fn A_SkimChase();
    fn A_SkullAttack();
    fn A_LobShot();
    fn A_FireShot();
    fn A_SuperFireShot();
    fn A_BossFireShot();
    fn A_Boss7FireMissiles();
    fn A_Boss1Laser();
    fn A_FocusTarget();
    fn A_Boss4Reverse();
    fn A_Boss4SpeedUp();
    fn A_Boss4Raise();
    fn A_SparkFollow();
    fn A_BuzzFly();
    fn A_GuardChase();
    fn A_EggShield();
    fn A_SetReactionTime();
    fn A_Boss1Spikeballs();
    fn A_Boss3TakeDamage();
    fn A_Boss3Path();
    fn A_Boss3ShockThink();
    fn A_Shockwave();
    fn A_LinedefExecute();
    fn A_LinedefExecuteFromArg();
    fn A_PlaySeeSound();
    fn A_PlayAttackSound();
    fn A_PlayActiveSound();
    fn A_1upThinker();
    fn A_BossZoom();
    fn A_Boss1Chase();
    fn A_Boss2Chase();
    fn A_Boss2Pogo();
    fn A_Boss7Chase();
    fn A_BossJetFume();
    fn A_SpawnObjectAbsolute();
    fn A_SpawnObjectRelative();
    fn A_ChangeAngleRelative();
    fn A_ChangeAngleAbsolute();
    fn A_RollAngle();
    fn A_ChangeRollAngleRelative();
    fn A_ChangeRollAngleAbsolute();
    fn A_PlaySound();
    fn A_FindTarget();
    fn A_FindTracer();
    fn A_SetTics();
    fn A_SetRandomTics();
    fn A_ChangeColorRelative();
    fn A_ChangeColorAbsolute();
    fn A_Dye();
    fn A_MoveRelative();
    fn A_MoveAbsolute();
    fn A_Thrust();
    fn A_ZThrust();
    fn A_SetTargetsTarget();
    fn A_SetObjectFlags();
    fn A_SetObjectFlags2();
    fn A_RandomState();
    fn A_RandomStateRange();
    fn A_StateRangeByAngle();
    fn A_StateRangeByParameter();
    fn A_DualAction();
    fn A_RemoteAction();
    fn A_ToggleFlameJet();
    fn A_OrbitNights();
    fn A_GhostMe();
    fn A_SetObjectState();
    fn A_SetObjectTypeState();
    fn A_KnockBack();
    fn A_PushAway();
    fn A_RingDrain();
    fn A_SplitShot();
    fn A_MissileSplit();
    fn A_MultiShot();
    fn A_InstaLoop();
    fn A_Custom3DRotate();
    fn A_SearchForPlayers();
    fn A_CheckRandom();
    fn A_CheckTargetRings();
    fn A_CheckRings();
    fn A_CheckTotalRings();
    fn A_CheckHealth();
    fn A_CheckRange();
    fn A_CheckHeight();
    fn A_CheckTrueRange();
    fn A_CheckThingCount();
    fn A_CheckAmbush();
    fn A_CheckCustomValue();
    fn A_CheckCusValMemo();
    fn A_SetCustomValue();
    fn A_UseCusValMemo();
    fn A_RelayCustomValue();
    fn A_CusValAction();
    fn A_ForceStop();
    fn A_ForceWin();
    fn A_SpikeRetract();
    fn A_InfoState();
    fn A_Invincibility();
    fn A_Explode();
    fn A_Pain();
    fn A_Fall();
    fn A_MonitorPop();
    fn A_GoldMonitorPop();
    fn A_GoldMonitorRestore();
    fn A_GoldMonitorSparkle();
    fn A_Look();
    fn A_Chase();
    fn A_FaceStabChase();
    fn A_FaceStabRev();
    fn A_FaceStabHurl();
    fn A_FaceStabMiss();
    fn A_StatueBurst();
    fn A_FaceTarget();
    fn A_FaceTracer();
    fn A_Scream();
    fn A_BossDeath();
    fn A_ShadowScream();
    fn A_CustomPower();
    fn A_GiveWeapon();
    fn A_RingBox();
    fn A_SetShadowScale();
    fn A_CryingToMomma();
    fn A_SetScale();
    fn A_RemoteDamage();
    fn A_HomingChase();
    fn A_TrapShot();
    fn A_VileTarget();
    fn A_VileAttack();
    fn A_VileFire();
    fn A_BrakChase();
    fn A_BrakFireShot();
    fn A_BrakLobShot();
    fn A_NapalmScatter();
    fn A_SpawnFreshCopy();
    fn A_FlickySpawn();
    fn A_FlickyCenter();
    fn A_FlickyAim();
    fn A_FlickyFly();
    fn A_FlickySoar();
    fn A_FlickyCoast();
    fn A_FlickyHop();
    fn A_FlickyFlounder();
    fn A_FlickyCheck();
    fn A_FlickyHeightCheck();
    fn A_FlickyFlutter();
    fn A_FlameParticle();
    fn A_FadeOverlay();
    fn A_Boss5Jump();
    fn A_LightBeamReset();
    fn A_MineExplode();
    fn A_MineRange();
    fn A_ConnectToGround();
    fn A_SpawnParticleRelative();
    fn A_MultiShotDist();
    fn A_WhoCaresIfYourSonIsABee();
    fn A_ParentTriesToSleep();
    fn A_FallingLavaCheck();
    fn A_Boss5MakeJunk();
    fn A_LookForBetter();
    fn A_Boss5BombExplode();
    fn A_DustDevilThink();
    fn A_TNTExplode();
    fn A_DebrisRandom();
    fn A_TrainCameo();
    fn A_TrainCameo2();
    fn A_CanarivoreGas();
    fn A_KillSegments();
    fn A_SnapperSpawn();
    fn A_SnapperThinker();
    fn A_SaloonDoorSpawn();
    fn A_MinecartSparkThink();
    fn A_ModuloToState();
    fn A_LavafallRocks();
    fn A_Boss5PinchShot();
    fn A_Boss5CheckFalling();
    fn A_Boss5CheckOnGround();
    fn A_Boss5Calm();
    fn A_Boss5ExtraRepeat();
    fn A_PrepareRepeat();
    fn A_DoNPCPain();
    fn A_DoNPCSkid();
    fn A_Boss5FindWaypoint();
    fn A_CheckFlags2();
    fn A_Boss5MakeItRain();
    fn A_LavafallLava();
    fn A_ChangeHeight();
    fn A_DragonSegment();
    fn A_DragonWing();
    fn A_DragonbomberSpawn();
    fn A_RolloutRock();
    fn A_RolloutSpawn();
    fn A_PterabyteHover();
    fn A_SpawnPterabytes();
    fn A_FireShrink();
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type C2RustUnnamed = libc::c_uint;
pub const NUMSUPERCOLORS: C2RustUnnamed = 9;
pub const MAXSKINCOLORS: C2RustUnnamed = 1182;
pub const SKINCOLOR_LASTFREESLOT: C2RustUnnamed = 1181;
pub const SKINCOLOR_FIRSTFREESLOT: C2RustUnnamed = 158;
pub const SKINCOLOR_SUPERTAN5: C2RustUnnamed = 157;
pub const SKINCOLOR_SUPERTAN4: C2RustUnnamed = 156;
pub const SKINCOLOR_SUPERTAN3: C2RustUnnamed = 155;
pub const SKINCOLOR_SUPERTAN2: C2RustUnnamed = 154;
pub const SKINCOLOR_SUPERTAN1: C2RustUnnamed = 153;
pub const SKINCOLOR_SUPERRUST5: C2RustUnnamed = 152;
pub const SKINCOLOR_SUPERRUST4: C2RustUnnamed = 151;
pub const SKINCOLOR_SUPERRUST3: C2RustUnnamed = 150;
pub const SKINCOLOR_SUPERRUST2: C2RustUnnamed = 149;
pub const SKINCOLOR_SUPERRUST1: C2RustUnnamed = 148;
pub const SKINCOLOR_SUPERPURPLE5: C2RustUnnamed = 147;
pub const SKINCOLOR_SUPERPURPLE4: C2RustUnnamed = 146;
pub const SKINCOLOR_SUPERPURPLE3: C2RustUnnamed = 145;
pub const SKINCOLOR_SUPERPURPLE2: C2RustUnnamed = 144;
pub const SKINCOLOR_SUPERPURPLE1: C2RustUnnamed = 143;
pub const SKINCOLOR_SUPERSKY5: C2RustUnnamed = 142;
pub const SKINCOLOR_SUPERSKY4: C2RustUnnamed = 141;
pub const SKINCOLOR_SUPERSKY3: C2RustUnnamed = 140;
pub const SKINCOLOR_SUPERSKY2: C2RustUnnamed = 139;
pub const SKINCOLOR_SUPERSKY1: C2RustUnnamed = 138;
pub const SKINCOLOR_SUPERPERIDOT5: C2RustUnnamed = 137;
pub const SKINCOLOR_SUPERPERIDOT4: C2RustUnnamed = 136;
pub const SKINCOLOR_SUPERPERIDOT3: C2RustUnnamed = 135;
pub const SKINCOLOR_SUPERPERIDOT2: C2RustUnnamed = 134;
pub const SKINCOLOR_SUPERPERIDOT1: C2RustUnnamed = 133;
pub const SKINCOLOR_SUPERGOLD5: C2RustUnnamed = 132;
pub const SKINCOLOR_SUPERGOLD4: C2RustUnnamed = 131;
pub const SKINCOLOR_SUPERGOLD3: C2RustUnnamed = 130;
pub const SKINCOLOR_SUPERGOLD2: C2RustUnnamed = 129;
pub const SKINCOLOR_SUPERGOLD1: C2RustUnnamed = 128;
pub const SKINCOLOR_SUPERORANGE5: C2RustUnnamed = 127;
pub const SKINCOLOR_SUPERORANGE4: C2RustUnnamed = 126;
pub const SKINCOLOR_SUPERORANGE3: C2RustUnnamed = 125;
pub const SKINCOLOR_SUPERORANGE2: C2RustUnnamed = 124;
pub const SKINCOLOR_SUPERORANGE1: C2RustUnnamed = 123;
pub const SKINCOLOR_SUPERRED5: C2RustUnnamed = 122;
pub const SKINCOLOR_SUPERRED4: C2RustUnnamed = 121;
pub const SKINCOLOR_SUPERRED3: C2RustUnnamed = 120;
pub const SKINCOLOR_SUPERRED2: C2RustUnnamed = 119;
pub const SKINCOLOR_SUPERRED1: C2RustUnnamed = 118;
pub const SKINCOLOR_SUPERSILVER5: C2RustUnnamed = 117;
pub const SKINCOLOR_SUPERSILVER4: C2RustUnnamed = 116;
pub const SKINCOLOR_SUPERSILVER3: C2RustUnnamed = 115;
pub const SKINCOLOR_SUPERSILVER2: C2RustUnnamed = 114;
pub const SKINCOLOR_SUPERSILVER1: C2RustUnnamed = 113;
pub const FIRSTSUPERCOLOR: C2RustUnnamed = 113;
pub const SKINCOLOR_VOLCANIC: C2RustUnnamed = 112;
pub const SKINCOLOR_SANGRIA: C2RustUnnamed = 111;
pub const SKINCOLOR_FANCY: C2RustUnnamed = 110;
pub const SKINCOLOR_ROSY: C2RustUnnamed = 109;
pub const SKINCOLOR_TAFFY: C2RustUnnamed = 108;
pub const SKINCOLOR_RASPBERRY: C2RustUnnamed = 107;
pub const SKINCOLOR_PLUM: C2RustUnnamed = 106;
pub const SKINCOLOR_EVENTIDE: C2RustUnnamed = 105;
pub const SKINCOLOR_MAUVE: C2RustUnnamed = 104;
pub const SKINCOLOR_LILAC: C2RustUnnamed = 103;
pub const SKINCOLOR_ROYAL: C2RustUnnamed = 102;
pub const SKINCOLOR_VIOLET: C2RustUnnamed = 101;
pub const SKINCOLOR_NEON: C2RustUnnamed = 100;
pub const SKINCOLOR_MAGENTA: C2RustUnnamed = 99;
pub const SKINCOLOR_SIBERITE: C2RustUnnamed = 98;
pub const SKINCOLOR_BUBBLEGUM: C2RustUnnamed = 97;
pub const SKINCOLOR_FUCHSIA: C2RustUnnamed = 96;
pub const SKINCOLOR_NOBLE: C2RustUnnamed = 95;
pub const SKINCOLOR_PURPLE: C2RustUnnamed = 94;
pub const SKINCOLOR_PASTEL: C2RustUnnamed = 93;
pub const SKINCOLOR_MAJESTY: C2RustUnnamed = 92;
pub const SKINCOLOR_DUSK: C2RustUnnamed = 91;
pub const SKINCOLOR_VAPOR: C2RustUnnamed = 90;
pub const SKINCOLOR_GALAXY: C2RustUnnamed = 89;
pub const SKINCOLOR_MIDNIGHT: C2RustUnnamed = 88;
pub const SKINCOLOR_COBALT: C2RustUnnamed = 87;
pub const SKINCOLOR_BLUE: C2RustUnnamed = 86;
pub const SKINCOLOR_CORNFLOWER: C2RustUnnamed = 85;
pub const SKINCOLOR_ARCTIC: C2RustUnnamed = 84;
pub const SKINCOLOR_SAPPHIRE: C2RustUnnamed = 83;
pub const SKINCOLOR_DAYBREAK: C2RustUnnamed = 82;
pub const SKINCOLOR_ICY: C2RustUnnamed = 81;
pub const SKINCOLOR_DREAM: C2RustUnnamed = 80;
pub const SKINCOLOR_CERULEAN: C2RustUnnamed = 79;
pub const SKINCOLOR_MARINE: C2RustUnnamed = 78;
pub const SKINCOLOR_SKY: C2RustUnnamed = 77;
pub const SKINCOLOR_AQUAMARINE: C2RustUnnamed = 76;
pub const SKINCOLOR_TURQUOISE: C2RustUnnamed = 75;
pub const SKINCOLOR_CYAN: C2RustUnnamed = 74;
pub const SKINCOLOR_WAVE: C2RustUnnamed = 73;
pub const SKINCOLOR_OCEAN: C2RustUnnamed = 72;
pub const SKINCOLOR_TEAL: C2RustUnnamed = 71;
pub const SKINCOLOR_AQUA: C2RustUnnamed = 70;
pub const SKINCOLOR_BOTTLE: C2RustUnnamed = 69;
pub const SKINCOLOR_ISLAND: C2RustUnnamed = 68;
pub const SKINCOLOR_SEAFOAM: C2RustUnnamed = 67;
pub const SKINCOLOR_EMERALD: C2RustUnnamed = 66;
pub const SKINCOLOR_MASTER: C2RustUnnamed = 65;
pub const SKINCOLOR_MINT: C2RustUnnamed = 64;
pub const SKINCOLOR_JADE: C2RustUnnamed = 63;
pub const SKINCOLOR_SHAMROCK: C2RustUnnamed = 62;
pub const SKINCOLOR_FOREST: C2RustUnnamed = 61;
pub const SKINCOLOR_GREEN: C2RustUnnamed = 60;
pub const SKINCOLOR_CHARTREUSE: C2RustUnnamed = 59;
pub const SKINCOLOR_HEADLIGHT: C2RustUnnamed = 58;
pub const SKINCOLOR_APPLE: C2RustUnnamed = 57;
pub const SKINCOLOR_PERIDOT: C2RustUnnamed = 56;
pub const SKINCOLOR_LIME: C2RustUnnamed = 55;
pub const SKINCOLOR_LEMON: C2RustUnnamed = 54;
pub const SKINCOLOR_PEAR: C2RustUnnamed = 53;
pub const SKINCOLOR_OLIVE: C2RustUnnamed = 52;
pub const SKINCOLOR_YELLOW: C2RustUnnamed = 51;
pub const SKINCOLOR_GOLDENROD: C2RustUnnamed = 50;
pub const SKINCOLOR_SANDY: C2RustUnnamed = 49;
pub const SKINCOLOR_GOLD: C2RustUnnamed = 48;
pub const SKINCOLOR_TOPAZ: C2RustUnnamed = 47;
pub const SKINCOLOR_TANGERINE: C2RustUnnamed = 46;
pub const SKINCOLOR_RUST: C2RustUnnamed = 45;
pub const SKINCOLOR_ORANGE: C2RustUnnamed = 44;
pub const SKINCOLOR_APRICOT: C2RustUnnamed = 43;
pub const SKINCOLOR_COPPER: C2RustUnnamed = 42;
pub const SKINCOLOR_SUNSET: C2RustUnnamed = 41;
pub const SKINCOLOR_FOUNDATION: C2RustUnnamed = 40;
pub const SKINCOLOR_QUAIL: C2RustUnnamed = 39;
pub const SKINCOLOR_PEACHY: C2RustUnnamed = 38;
pub const SKINCOLOR_KETCHUP: C2RustUnnamed = 37;
pub const SKINCOLOR_GARNET: C2RustUnnamed = 36;
pub const SKINCOLOR_FLAME: C2RustUnnamed = 35;
pub const SKINCOLOR_CRIMSON: C2RustUnnamed = 34;
pub const SKINCOLOR_RED: C2RustUnnamed = 33;
pub const SKINCOLOR_PEPPER: C2RustUnnamed = 32;
pub const SKINCOLOR_SALMON: C2RustUnnamed = 31;
pub const SKINCOLOR_CHERRY: C2RustUnnamed = 30;
pub const SKINCOLOR_RUBY: C2RustUnnamed = 29;
pub const SKINCOLOR_LAVENDER: C2RustUnnamed = 28;
pub const SKINCOLOR_EGGPLANT: C2RustUnnamed = 27;
pub const SKINCOLOR_AZURE: C2RustUnnamed = 26;
pub const SKINCOLOR_MOSS: C2RustUnnamed = 25;
pub const SKINCOLOR_ROSEBUSH: C2RustUnnamed = 24;
pub const SKINCOLOR_BEIGE: C2RustUnnamed = 23;
pub const SKINCOLOR_TAN: C2RustUnnamed = 22;
pub const SKINCOLOR_ECRU: C2RustUnnamed = 21;
pub const SKINCOLOR_SEPIA: C2RustUnnamed = 20;
pub const SKINCOLOR_BRONZE: C2RustUnnamed = 19;
pub const SKINCOLOR_BOULDER: C2RustUnnamed = 18;
pub const SKINCOLOR_BROWN: C2RustUnnamed = 17;
pub const SKINCOLOR_LATTE: C2RustUnnamed = 16;
pub const SKINCOLOR_YOGURT: C2RustUnnamed = 15;
pub const SKINCOLOR_ROSEWOOD: C2RustUnnamed = 14;
pub const SKINCOLOR_PINK: C2RustUnnamed = 13;
pub const SKINCOLOR_BLUEBELL: C2RustUnnamed = 12;
pub const SKINCOLOR_MOONSTONE: C2RustUnnamed = 11;
pub const SKINCOLOR_SLATE: C2RustUnnamed = 10;
pub const SKINCOLOR_AETHER: C2RustUnnamed = 9;
pub const SKINCOLOR_BLACK: C2RustUnnamed = 8;
pub const SKINCOLOR_JET: C2RustUnnamed = 7;
pub const SKINCOLOR_CARBON: C2RustUnnamed = 6;
pub const SKINCOLOR_SILVER: C2RustUnnamed = 5;
pub const SKINCOLOR_GREY: C2RustUnnamed = 4;
pub const SKINCOLOR_CLOUDY: C2RustUnnamed = 3;
pub const SKINCOLOR_BONE: C2RustUnnamed = 2;
pub const SKINCOLOR_WHITE: C2RustUnnamed = 1;
pub const SKINCOLOR_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_int;
pub const LE_PARAMWIDTH: C2RustUnnamed_0 = -100;
pub const LE_AXE: C2RustUnnamed_0 = 649;
pub const LE_KOOPA: C2RustUnnamed_0 = 650;
pub const LE_CAPSULE0: C2RustUnnamed_0 = 680;
pub const LE_CAPSULE1: C2RustUnnamed_0 = 681;
pub const LE_CAPSULE2: C2RustUnnamed_0 = 682;
pub const LE_BRAKPLATFORM: C2RustUnnamed_0 = 4200;
pub const LE_TURRET: C2RustUnnamed_0 = 32000;
pub const LE_BRAKVILEATACK: C2RustUnnamed_0 = -6;
pub const LE_BOSS4DROP: C2RustUnnamed_0 = -5;
pub const LE_BOSSDEAD: C2RustUnnamed_0 = -4;
pub const LE_ALLBOSSESDEAD: C2RustUnnamed_0 = -3;
pub const LE_PINCHPHASE: C2RustUnnamed_0 = -2;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const GS_WAITINGPLAYERS: C2RustUnnamed_1 = 13;
pub const GS_DEDICATEDSERVER: C2RustUnnamed_1 = 12;
pub const GS_CUTSCENE: C2RustUnnamed_1 = 11;
pub const GS_ENDING: C2RustUnnamed_1 = 10;
pub const GS_INTRO: C2RustUnnamed_1 = 9;
pub const GS_GAMEEND: C2RustUnnamed_1 = 8;
pub const GS_EVALUATION: C2RustUnnamed_1 = 7;
pub const GS_CREDITS: C2RustUnnamed_1 = 6;
pub const GS_TIMEATTACK: C2RustUnnamed_1 = 5;
pub const GS_TITLESCREEN: C2RustUnnamed_1 = 4;
pub const GS_CONTINUING: C2RustUnnamed_1 = 3;
pub const GS_INTERMISSION: C2RustUnnamed_1 = 2;
pub const GS_LEVEL: C2RustUnnamed_1 = 1;
pub const GS_NULL: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const NUMSKINSOUNDS: C2RustUnnamed_2 = 21;
pub const SKSJUMP: C2RustUnnamed_2 = 20;
pub const SKSGASP: C2RustUnnamed_2 = 19;
pub const SKSSKID: C2RustUnnamed_2 = 18;
pub const SKSZOOM: C2RustUnnamed_2 = 17;
pub const SKSSPNDSH: C2RustUnnamed_2 = 16;
pub const SKSTHOK: C2RustUnnamed_2 = 15;
pub const SKSPLVCT4: C2RustUnnamed_2 = 14;
pub const SKSPLVCT3: C2RustUnnamed_2 = 13;
pub const SKSPLVCT2: C2RustUnnamed_2 = 12;
pub const SKSPLVCT1: C2RustUnnamed_2 = 11;
pub const SKSPLDET4: C2RustUnnamed_2 = 10;
pub const SKSPLDET3: C2RustUnnamed_2 = 9;
pub const SKSPLDET2: C2RustUnnamed_2 = 8;
pub const SKSPLDET1: C2RustUnnamed_2 = 7;
pub const SKSPLPAN4: C2RustUnnamed_2 = 6;
pub const SKSPLPAN3: C2RustUnnamed_2 = 5;
pub const SKSPLPAN2: C2RustUnnamed_2 = 4;
pub const SKSPLPAN1: C2RustUnnamed_2 = 3;
pub const SKSPUDPUD: C2RustUnnamed_2 = 2;
pub const SKSPUTPUT: C2RustUnnamed_2 = 1;
pub const SKSSPIN: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const COM_LUA: C2RustUnnamed_3 = 8;
pub const COM_LOCAL: C2RustUnnamed_3 = 4;
pub const COM_SPLITSCREEN: C2RustUnnamed_3 = 2;
pub const COM_ADMIN: C2RustUnnamed_3 = 1;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const CV_ALLOWLUA: C2RustUnnamed_4 = 4096;
pub const CV_CHEAT: C2RustUnnamed_4 = 2048;
pub const CV_HIDEN: C2RustUnnamed_4 = 1024;
pub const CV_NOSHOWHELP: C2RustUnnamed_4 = 512;
pub const CV_SHOWMODIFONETIME: C2RustUnnamed_4 = 256;
pub const CV_SHOWMODIF: C2RustUnnamed_4 = 128;
pub const CV_MODIFIED: C2RustUnnamed_4 = 64;
pub const CV_NOTINNET: C2RustUnnamed_4 = 32;
pub const CV_FLOAT: C2RustUnnamed_4 = 16;
pub const CV_NOINIT: C2RustUnnamed_4 = 8;
pub const CV_NETVAR: C2RustUnnamed_4 = 4;
pub const CV_CALL: C2RustUnnamed_4 = 2;
pub const CV_SAVE: C2RustUnnamed_4 = 1;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const MU_MID_EX: C2RustUnnamed_5 = 9;
pub const MU_MOD_EX: C2RustUnnamed_5 = 8;
pub const MU_GME: C2RustUnnamed_5 = 7;
pub const MU_FLAC: C2RustUnnamed_5 = 6;
pub const MU_MP3: C2RustUnnamed_5 = 5;
pub const MU_OGG: C2RustUnnamed_5 = 4;
pub const MU_MID: C2RustUnnamed_5 = 3;
pub const MU_MOD: C2RustUnnamed_5 = 2;
pub const MU_WAV: C2RustUnnamed_5 = 1;
pub const MU_NONE: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const SF_X2AWAYSOUND: C2RustUnnamed_6 = 64;
pub const SF_NOINTERRUPT: C2RustUnnamed_6 = 32;
pub const SF_X8AWAYSOUND: C2RustUnnamed_6 = 16;
pub const SF_X4AWAYSOUND: C2RustUnnamed_6 = 8;
pub const SF_OUTSIDESOUND: C2RustUnnamed_6 = 4;
pub const SF_NOMULTIPLESOUND: C2RustUnnamed_6 = 2;
pub const SF_TOTALLYSINGLE: C2RustUnnamed_6 = 1;
pub type actionf_v = Option::<unsafe extern "C" fn() -> ()>;
pub type actionf_p1 = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union actionf_t {
    pub acv: actionf_v,
    pub acp1: actionf_p1,
}
pub type mobj_type = libc::c_uint;
pub const NUMMOBJTYPES: mobj_type = 1163;
pub const MT_LASTFREESLOT: mobj_type = 1162;
pub const MT_FIRSTFREESLOT: mobj_type = 651;
pub const MT_RAY: mobj_type = 650;
pub const MT_NAMECHECK: mobj_type = 649;
pub const MT_YELLOWBRICKDEBRIS: mobj_type = 648;
pub const MT_BLUEBRICKDEBRIS: mobj_type = 647;
pub const MT_REDBRICKDEBRIS: mobj_type = 646;
pub const MT_WOODDEBRIS: mobj_type = 645;
pub const MT_BRICKDEBRIS: mobj_type = 644;
pub const MT_GFZDEBRIS: mobj_type = 643;
pub const MT_ROCKCRUMBLE16: mobj_type = 642;
pub const MT_ROCKCRUMBLE15: mobj_type = 641;
pub const MT_ROCKCRUMBLE14: mobj_type = 640;
pub const MT_ROCKCRUMBLE13: mobj_type = 639;
pub const MT_ROCKCRUMBLE12: mobj_type = 638;
pub const MT_ROCKCRUMBLE11: mobj_type = 637;
pub const MT_ROCKCRUMBLE10: mobj_type = 636;
pub const MT_ROCKCRUMBLE9: mobj_type = 635;
pub const MT_ROCKCRUMBLE8: mobj_type = 634;
pub const MT_ROCKCRUMBLE7: mobj_type = 633;
pub const MT_ROCKCRUMBLE6: mobj_type = 632;
pub const MT_ROCKCRUMBLE5: mobj_type = 631;
pub const MT_ROCKCRUMBLE4: mobj_type = 630;
pub const MT_ROCKCRUMBLE3: mobj_type = 629;
pub const MT_ROCKCRUMBLE2: mobj_type = 628;
pub const MT_ROCKCRUMBLE1: mobj_type = 627;
pub const MT_FALLINGROCK: mobj_type = 626;
pub const MT_ROCKSPAWNER: mobj_type = 625;
pub const MT_DUST: mobj_type = 624;
pub const MT_UWEXPLODE: mobj_type = 623;
pub const MT_EXPLODE: mobj_type = 622;
pub const MT_SPARK: mobj_type = 621;
pub const MT_SKYBOX: mobj_type = 620;
pub const MT_POLYSPAWN: mobj_type = 619;
pub const MT_POLYANCHOR: mobj_type = 618;
pub const MT_ANGLEMAN: mobj_type = 617;
pub const MT_OVERLAY: mobj_type = 616;
pub const MT_GHOST: mobj_type = 615;
pub const MT_PUSH: mobj_type = 614;
pub const MT_TUBEWAYPOINT: mobj_type = 613;
pub const MT_CRUMBLEOBJ: mobj_type = 612;
pub const MT_ALTVIEWMAN: mobj_type = 611;
pub const MT_TELEPORTMAN: mobj_type = 610;
pub const MT_HANGSTER: mobj_type = 609;
pub const MT_SPINBOBERT_FIRE2: mobj_type = 608;
pub const MT_SPINBOBERT_FIRE1: mobj_type = 607;
pub const MT_SPINBOBERT: mobj_type = 606;
pub const MT_CACOFIRE: mobj_type = 605;
pub const MT_CACOSHARD: mobj_type = 604;
pub const MT_CACOLANTERN: mobj_type = 603;
pub const MT_SMASHINGSPIKEBALL: mobj_type = 602;
pub const MT_BUGGLE: mobj_type = 601;
pub const MT_BUMBLEBORE: mobj_type = 600;
pub const MT_HIVEELEMENTAL: mobj_type = 599;
pub const MT_POPSHOT_TRAIL: mobj_type = 598;
pub const MT_POPSHOT: mobj_type = 597;
pub const MT_POPHAT: mobj_type = 596;
pub const MT_PENGUINATOR: mobj_type = 595;
pub const MT_SHLEEP: mobj_type = 594;
pub const MT_PIAN: mobj_type = 593;
pub const MT_NIGHTOPIANHELPER: mobj_type = 592;
pub const MT_IDEYAANCHOR: mobj_type = 591;
pub const MT_EGGCAPSULE: mobj_type = 590;
pub const MT_NIGHTSLINKFREEZE: mobj_type = 589;
pub const MT_NIGHTSEXTRATIME: mobj_type = 588;
pub const MT_NIGHTSHELPER: mobj_type = 587;
pub const MT_NIGHTSDRILLREFILL: mobj_type = 586;
pub const MT_NIGHTSSUPERLOOP: mobj_type = 585;
pub const MT_FLINGNIGHTSSTAR: mobj_type = 584;
pub const MT_NIGHTSSTAR: mobj_type = 583;
pub const MT_FLINGNIGHTSCHIP: mobj_type = 582;
pub const MT_NIGHTSCHIP: mobj_type = 581;
pub const MT_NIGHTSCORE: mobj_type = 580;
pub const MT_HOOPCENTER: mobj_type = 579;
pub const MT_HOOPCOLLIDE: mobj_type = 578;
pub const MT_HOOP: mobj_type = 577;
pub const MT_NIGHTSBUMPER: mobj_type = 576;
pub const MT_NIGHTSLOOPHELPER: mobj_type = 575;
pub const MT_NIGHTSPARKLE: mobj_type = 574;
pub const MT_NIGHTSDRONE_GOAL: mobj_type = 573;
pub const MT_NIGHTSDRONE_SPARKLING: mobj_type = 572;
pub const MT_NIGHTSDRONE_MAN: mobj_type = 571;
pub const MT_NIGHTSDRONE: mobj_type = 570;
pub const MT_AXISTRANSFERLINE: mobj_type = 569;
pub const MT_AXISTRANSFER: mobj_type = 568;
pub const MT_AXIS: mobj_type = 567;
pub const MT_TOAD: mobj_type = 566;
pub const MT_MARIOBUSH2: mobj_type = 565;
pub const MT_MARIOBUSH1: mobj_type = 564;
pub const MT_AXE: mobj_type = 563;
pub const MT_KOOPAFLAME: mobj_type = 562;
pub const MT_KOOPA: mobj_type = 561;
pub const MT_HAMMER: mobj_type = 560;
pub const MT_PUMATRAIL: mobj_type = 559;
pub const MT_PUMA: mobj_type = 558;
pub const MT_SHELL: mobj_type = 557;
pub const MT_FIREBALLTRAIL: mobj_type = 556;
pub const MT_FIREBALL: mobj_type = 555;
pub const MT_FIREFLOWER: mobj_type = 554;
pub const MT_BLUEGOOMBA: mobj_type = 553;
pub const MT_GOOMBA: mobj_type = 552;
pub const MT_FLINGCOIN: mobj_type = 551;
pub const MT_COIN: mobj_type = 550;
pub const MT_THROWNGRENADE: mobj_type = 549;
pub const MT_THROWNEXPLOSION: mobj_type = 548;
pub const MT_THROWNSCATTER: mobj_type = 547;
pub const MT_THROWNAUTOMATIC: mobj_type = 546;
pub const MT_THROWNINFINITY: mobj_type = 545;
pub const MT_THROWNBOUNCE: mobj_type = 544;
pub const MT_GRENADEPICKUP: mobj_type = 543;
pub const MT_SCATTERPICKUP: mobj_type = 542;
pub const MT_EXPLODEPICKUP: mobj_type = 541;
pub const MT_AUTOPICKUP: mobj_type = 540;
pub const MT_RAILPICKUP: mobj_type = 539;
pub const MT_BOUNCEPICKUP: mobj_type = 538;
pub const MT_GRENADERING: mobj_type = 537;
pub const MT_SCATTERRING: mobj_type = 536;
pub const MT_EXPLOSIONRING: mobj_type = 535;
pub const MT_AUTOMATICRING: mobj_type = 534;
pub const MT_INFINITYRING: mobj_type = 533;
pub const MT_RAILRING: mobj_type = 532;
pub const MT_BOUNCERING: mobj_type = 531;
pub const MT_REDRING: mobj_type = 530;
pub const MT_LHRT: mobj_type = 529;
pub const MT_CORK: mobj_type = 528;
pub const MT_AMBIENT: mobj_type = 527;
pub const MT_FINISHFLAG: mobj_type = 526;
pub const MT_GOTFLAG: mobj_type = 525;
pub const MT_TAG: mobj_type = 524;
pub const MT_LOCKONINF: mobj_type = 523;
pub const MT_LOCKON: mobj_type = 522;
pub const MT_GOTEMERALD: mobj_type = 521;
pub const MT_DROWNNUMBERS: mobj_type = 520;
pub const MT_SCORE: mobj_type = 519;
pub const MT_PARTICLEGEN: mobj_type = 518;
pub const MT_PARTICLE: mobj_type = 517;
pub const MT_TFOG: mobj_type = 516;
pub const MT_SPINDUST: mobj_type = 515;
pub const MT_WATERZAP: mobj_type = 514;
pub const MT_EXTRALARGEBUBBLE: mobj_type = 513;
pub const MT_MEDIUMBUBBLE: mobj_type = 512;
pub const MT_SMALLBUBBLE: mobj_type = 511;
pub const MT_SMOKE: mobj_type = 510;
pub const MT_LAVASPLISH: mobj_type = 509;
pub const MT_SPLISH: mobj_type = 508;
pub const MT_SNOWFLAKE: mobj_type = 507;
pub const MT_RAIN: mobj_type = 506;
pub const MT_SEED: mobj_type = 505;
pub const MT_SECRETFLICKY_02_CENTER: mobj_type = 504;
pub const MT_SECRETFLICKY_02: mobj_type = 503;
pub const MT_SECRETFLICKY_01_CENTER: mobj_type = 502;
pub const MT_SECRETFLICKY_01: mobj_type = 501;
pub const MT_FLICKY_16_CENTER: mobj_type = 500;
pub const MT_FLICKY_16: mobj_type = 499;
pub const MT_FLICKY_15_CENTER: mobj_type = 498;
pub const MT_FLICKY_15: mobj_type = 497;
pub const MT_FLICKY_14_CENTER: mobj_type = 496;
pub const MT_FLICKY_14: mobj_type = 495;
pub const MT_FLICKY_13_CENTER: mobj_type = 494;
pub const MT_FLICKY_13: mobj_type = 493;
pub const MT_FLICKY_12_CENTER: mobj_type = 492;
pub const MT_FLICKY_12: mobj_type = 491;
pub const MT_FLICKY_11_CENTER: mobj_type = 490;
pub const MT_FLICKY_11: mobj_type = 489;
pub const MT_FLICKY_10_CENTER: mobj_type = 488;
pub const MT_FLICKY_10: mobj_type = 487;
pub const MT_FLICKY_09_CENTER: mobj_type = 486;
pub const MT_FLICKY_09: mobj_type = 485;
pub const MT_FLICKY_08_CENTER: mobj_type = 484;
pub const MT_FLICKY_08: mobj_type = 483;
pub const MT_FLICKY_07_CENTER: mobj_type = 482;
pub const MT_FLICKY_07: mobj_type = 481;
pub const MT_FLICKY_06_CENTER: mobj_type = 480;
pub const MT_FLICKY_06: mobj_type = 479;
pub const MT_FLICKY_05_CENTER: mobj_type = 478;
pub const MT_FLICKY_05: mobj_type = 477;
pub const MT_FLICKY_04_CENTER: mobj_type = 476;
pub const MT_FLICKY_04: mobj_type = 475;
pub const MT_FLICKY_03_CENTER: mobj_type = 474;
pub const MT_FLICKY_03: mobj_type = 473;
pub const MT_FLICKY_02_CENTER: mobj_type = 472;
pub const MT_FLICKY_02: mobj_type = 471;
pub const MT_FLICKY_01_CENTER: mobj_type = 470;
pub const MT_FLICKY_01: mobj_type = 469;
pub const MT_SUPERSPARK: mobj_type = 468;
pub const MT_IVSP: mobj_type = 467;
pub const MT_THUNDERCOIN_SPARK: mobj_type = 466;
pub const MT_THUNDERCOIN_ORB: mobj_type = 465;
pub const MT_BUBBLEWRAP_ORB: mobj_type = 464;
pub const MT_FLAMEAURA_ORB: mobj_type = 463;
pub const MT_PITY_ORB: mobj_type = 462;
pub const MT_WHIRLWIND_ORB: mobj_type = 461;
pub const MT_ARMAGEDDON_ORB: mobj_type = 460;
pub const MT_FORCE_ORB: mobj_type = 459;
pub const MT_ATTRACT_ORB: mobj_type = 458;
pub const MT_ELEMENTAL_ORB: mobj_type = 457;
pub const MT_EGGSTATUE2: mobj_type = 456;
pub const MT_DBALL: mobj_type = 455;
pub const MT_PALMTREE_TOP: mobj_type = 454;
pub const MT_PALMTREE_TRUNK: mobj_type = 453;
pub const MT_BIG_PALMTREE_TOP: mobj_type = 452;
pub const MT_BIG_PALMTREE_TRUNK: mobj_type = 451;
pub const MT_BSZCLOVER: mobj_type = 450;
pub const MT_BSZSHRUB: mobj_type = 449;
pub const MT_BSZVINE_ORANGE: mobj_type = 448;
pub const MT_BSZVINE_YELLOW: mobj_type = 447;
pub const MT_BSZVINE_CYAN: mobj_type = 446;
pub const MT_BSZVINE_BLUE: mobj_type = 445;
pub const MT_BSZVINE_PURPLE: mobj_type = 444;
pub const MT_BSZVINE_RED: mobj_type = 443;
pub const MT_BSZBUSH_ORANGE: mobj_type = 442;
pub const MT_BSZBUSH_YELLOW: mobj_type = 441;
pub const MT_BSZBUSH_CYAN: mobj_type = 440;
pub const MT_BSZBUSH_BLUE: mobj_type = 439;
pub const MT_BSZBUSH_PURPLE: mobj_type = 438;
pub const MT_BSZBUSH_RED: mobj_type = 437;
pub const MT_BSZCLUSTER_ORANGE: mobj_type = 436;
pub const MT_BSZCLUSTER_YELLOW: mobj_type = 435;
pub const MT_BSZCLUSTER_CYAN: mobj_type = 434;
pub const MT_BSZCLUSTER_BLUE: mobj_type = 433;
pub const MT_BSZCLUSTER_PURPLE: mobj_type = 432;
pub const MT_BSZCLUSTER_RED: mobj_type = 431;
pub const MT_BSZTULIP_ORANGE: mobj_type = 430;
pub const MT_BSZTULIP_YELLOW: mobj_type = 429;
pub const MT_BSZTULIP_CYAN: mobj_type = 428;
pub const MT_BSZTULIP_BLUE: mobj_type = 427;
pub const MT_BSZTULIP_PURPLE: mobj_type = 426;
pub const MT_BSZTULIP_RED: mobj_type = 425;
pub const MT_BSZSHORTFLOWER_ORANGE: mobj_type = 424;
pub const MT_BSZSHORTFLOWER_YELLOW: mobj_type = 423;
pub const MT_BSZSHORTFLOWER_CYAN: mobj_type = 422;
pub const MT_BSZSHORTFLOWER_BLUE: mobj_type = 421;
pub const MT_BSZSHORTFLOWER_PURPLE: mobj_type = 420;
pub const MT_BSZSHORTFLOWER_RED: mobj_type = 419;
pub const MT_BSZFLOWER_ORANGE: mobj_type = 418;
pub const MT_BSZFLOWER_YELLOW: mobj_type = 417;
pub const MT_BSZFLOWER_CYAN: mobj_type = 416;
pub const MT_BSZFLOWER_BLUE: mobj_type = 415;
pub const MT_BSZFLOWER_PURPLE: mobj_type = 414;
pub const MT_BSZFLOWER_RED: mobj_type = 413;
pub const MT_BSZTALLFLOWER_ORANGE: mobj_type = 412;
pub const MT_BSZTALLFLOWER_YELLOW: mobj_type = 411;
pub const MT_BSZTALLFLOWER_CYAN: mobj_type = 410;
pub const MT_BSZTALLFLOWER_BLUE: mobj_type = 409;
pub const MT_BSZTALLFLOWER_PURPLE: mobj_type = 408;
pub const MT_BSZTALLFLOWER_RED: mobj_type = 407;
pub const MT_HHZSTALAGMITE_SHORT: mobj_type = 406;
pub const MT_HHZSTALAGMITE_TALL: mobj_type = 405;
pub const MT_HHZTENTACLE2: mobj_type = 404;
pub const MT_HHZTENTACLE1: mobj_type = 403;
pub const MT_HHZGRASS: mobj_type = 402;
pub const MT_HHZSHROOM: mobj_type = 401;
pub const MT_HHZTREE_PART: mobj_type = 400;
pub const MT_HHZTREE_TOP: mobj_type = 399;
pub const MT_JACKO3: mobj_type = 398;
pub const MT_JACKO2: mobj_type = 397;
pub const MT_JACKO1: mobj_type = 396;
pub const MT_CDLHRT: mobj_type = 395;
pub const MT_ROSY: mobj_type = 394;
pub const MT_FHZICE2: mobj_type = 393;
pub const MT_FHZICE1: mobj_type = 392;
pub const MT_XMASBUSH: mobj_type = 391;
pub const MT_XMASBERRYBUSH: mobj_type = 390;
pub const MT_XMASBLUEBERRYBUSH: mobj_type = 389;
pub const MT_MISTLETOE: mobj_type = 388;
pub const MT_HANGSTAR: mobj_type = 387;
pub const MT_LAMPPOST2: mobj_type = 386;
pub const MT_LAMPPOST1: mobj_type = 385;
pub const MT_SNOWMANHAT: mobj_type = 384;
pub const MT_SNOWMAN: mobj_type = 383;
pub const MT_CANDYCANE: mobj_type = 382;
pub const MT_XMASPOLE: mobj_type = 381;
pub const MT_STALAGMITE9: mobj_type = 380;
pub const MT_STALAGMITE8: mobj_type = 379;
pub const MT_STALAGMITE7: mobj_type = 378;
pub const MT_STALAGMITE6: mobj_type = 377;
pub const MT_STALAGMITE5: mobj_type = 376;
pub const MT_STALAGMITE4: mobj_type = 375;
pub const MT_STALAGMITE3: mobj_type = 374;
pub const MT_STALAGMITE2: mobj_type = 373;
pub const MT_STALAGMITE1: mobj_type = 372;
pub const MT_STALAGMITE0: mobj_type = 371;
pub const MT_BLUEGARGOYLE: mobj_type = 370;
pub const MT_GREENFLAME: mobj_type = 369;
pub const MT_TARGET: mobj_type = 368;
pub const MT_GLAREGOYLELONG: mobj_type = 367;
pub const MT_GLAREGOYLEDOWN: mobj_type = 366;
pub const MT_GLAREGOYLEUP: mobj_type = 365;
pub const MT_GLAREGOYLE: mobj_type = 364;
pub const MT_WALLVINE_SHORT: mobj_type = 363;
pub const MT_WALLVINE_LONG: mobj_type = 362;
pub const MT_TORCHFLOWER: mobj_type = 361;
pub const MT_JUNGLEPALM: mobj_type = 360;
pub const MT_BIGFERN: mobj_type = 359;
pub const MT_BIGFERNLEAF: mobj_type = 358;
pub const MT_ROLLOUTROCK: mobj_type = 357;
pub const MT_ROLLOUTSPAWN: mobj_type = 356;
pub const MT_LAVAFALLROCK: mobj_type = 355;
pub const MT_LAVAFALL_LAVA: mobj_type = 354;
pub const MT_LAVAFALL: mobj_type = 353;
pub const MT_FLAMEJETFLAMEB: mobj_type = 352;
pub const MT_FJSPINAXISB: mobj_type = 351;
pub const MT_FJSPINAXISA: mobj_type = 350;
pub const MT_FLAMEJETFLAME: mobj_type = 349;
pub const MT_VERTICALFLAMEJET: mobj_type = 348;
pub const MT_FLAMEJET: mobj_type = 347;
pub const MT_MINECARTSWITCHPOINT: mobj_type = 346;
pub const MT_TRAINSTEAMSPAWNER: mobj_type = 345;
pub const MT_TRAINDUSTSPAWNER: mobj_type = 344;
pub const MT_TRAINSEG: mobj_type = 343;
pub const MT_TRAINCAMEOSPAWNER: mobj_type = 342;
pub const MT_SALOONDOORCENTER: mobj_type = 341;
pub const MT_SALOONDOOR: mobj_type = 340;
pub const MT_MINECARTSPARK: mobj_type = 339;
pub const MT_MINECARTSIDEMARK: mobj_type = 338;
pub const MT_MINECARTENDSOLID: mobj_type = 337;
pub const MT_MINECARTEND: mobj_type = 336;
pub const MT_MINECARTSPAWNER: mobj_type = 335;
pub const MT_MINECARTSEG: mobj_type = 334;
pub const MT_MINECART: mobj_type = 333;
pub const MT_ARIDDUST: mobj_type = 332;
pub const MT_DUSTLAYER: mobj_type = 331;
pub const MT_DUSTDEVIL: mobj_type = 330;
pub const MT_PROXIMITYTNT: mobj_type = 329;
pub const MT_TNTBARREL: mobj_type = 328;
pub const MT_OILLAMP: mobj_type = 327;
pub const MT_ARIDSIGN_SHARPTURN: mobj_type = 326;
pub const MT_ARIDSIGN_CACTI: mobj_type = 325;
pub const MT_ARIDSIGN_CAUTION: mobj_type = 324;
pub const MT_CACTISMALLSEG: mobj_type = 323;
pub const MT_CACTITINYSEG: mobj_type = 322;
pub const MT_CACTI11: mobj_type = 321;
pub const MT_CACTI10: mobj_type = 320;
pub const MT_CACTI9: mobj_type = 319;
pub const MT_CACTI8: mobj_type = 318;
pub const MT_CACTI7: mobj_type = 317;
pub const MT_CACTI6: mobj_type = 316;
pub const MT_CACTI5: mobj_type = 315;
pub const MT_CACTI4: mobj_type = 314;
pub const MT_CACTI3: mobj_type = 313;
pub const MT_CACTI2: mobj_type = 312;
pub const MT_CACTI1: mobj_type = 311;
pub const MT_LITTLETUMBLEWEED: mobj_type = 310;
pub const MT_BIGTUMBLEWEED: mobj_type = 309;
pub const MT_BRAMBLES: mobj_type = 308;
pub const MT_SUSPICIOUSFACESTABBERSTATUE: mobj_type = 307;
pub const MT_FACESTABBERSTATUE: mobj_type = 306;
pub const MT_CRAWLASTATUE: mobj_type = 305;
pub const MT_WAVINGFLAGSEG2: mobj_type = 304;
pub const MT_WAVINGFLAGSEG1: mobj_type = 303;
pub const MT_WAVINGFLAG2: mobj_type = 302;
pub const MT_WAVINGFLAG1: mobj_type = 301;
pub const MT_FIRETORCH: mobj_type = 300;
pub const MT_FLAMEHOLDER: mobj_type = 299;
pub const MT_CANDLEPRICKET: mobj_type = 298;
pub const MT_CANDLE: mobj_type = 297;
pub const MT_CEZBUSH2: mobj_type = 296;
pub const MT_CEZBUSH1: mobj_type = 295;
pub const MT_PINETREE: mobj_type = 294;
pub const MT_CEZBANNER2: mobj_type = 293;
pub const MT_CEZBANNER1: mobj_type = 292;
pub const MT_CEZPOLE2: mobj_type = 291;
pub const MT_CEZPOLE1: mobj_type = 290;
pub const MT_CEZFLOWER: mobj_type = 289;
pub const MT_BIGFIREBAR: mobj_type = 288;
pub const MT_SMALLFIREBAR: mobj_type = 287;
pub const MT_REDSPRINGBALL: mobj_type = 286;
pub const MT_YELLOWSPRINGBALL: mobj_type = 285;
pub const MT_BIGGRABCHAIN: mobj_type = 284;
pub const MT_SMALLGRABCHAIN: mobj_type = 283;
pub const MT_BIGMACE: mobj_type = 282;
pub const MT_SMALLMACE: mobj_type = 281;
pub const MT_BIGMACECHAIN: mobj_type = 280;
pub const MT_SMALLMACECHAIN: mobj_type = 279;
pub const MT_CUSTOMMACEPOINT: mobj_type = 278;
pub const MT_FIREBARPOINT: mobj_type = 277;
pub const MT_HIDDEN_SLING: mobj_type = 276;
pub const MT_CHAINPOINT: mobj_type = 275;
pub const MT_SPRINGBALLPOINT: mobj_type = 274;
pub const MT_CHAINMACEPOINT: mobj_type = 273;
pub const MT_MACEPOINT: mobj_type = 272;
pub const MT_EGGSTATUE: mobj_type = 271;
pub const MT_FLAMEPARTICLE: mobj_type = 270;
pub const MT_FLAME: mobj_type = 269;
pub const MT_CHAIN: mobj_type = 268;
pub const MT_LIGHTBEAM: mobj_type = 267;
pub const MT_DSZ2STALAGMITE: mobj_type = 266;
pub const MT_DSZSTALAGMITE: mobj_type = 265;
pub const MT_ANIMALGAESEG: mobj_type = 264;
pub const MT_ANIMALGAETOP: mobj_type = 263;
pub const MT_KELP: mobj_type = 262;
pub const MT_BLUECRYSTAL: mobj_type = 261;
pub const MT_CORAL5: mobj_type = 260;
pub const MT_CORAL4: mobj_type = 259;
pub const MT_CORAL3: mobj_type = 258;
pub const MT_CORAL2: mobj_type = 257;
pub const MT_CORAL1: mobj_type = 256;
pub const MT_WATERDROP: mobj_type = 255;
pub const MT_WATERDRIP: mobj_type = 254;
pub const MT_SEAWEED: mobj_type = 253;
pub const MT_BIGGARGOYLE: mobj_type = 252;
pub const MT_GARGOYLE: mobj_type = 251;
pub const MT_ALARM: mobj_type = 250;
pub const MT_THZTREEBRANCH: mobj_type = 249;
pub const MT_THZTREE: mobj_type = 248;
pub const MT_THZFLOWER3: mobj_type = 247;
pub const MT_THZFLOWER2: mobj_type = 246;
pub const MT_THZFLOWER1: mobj_type = 245;
pub const MT_SPRINGTREE: mobj_type = 244;
pub const MT_BUSHREDTREE: mobj_type = 243;
pub const MT_BUSHTREE: mobj_type = 242;
pub const MT_POLYGONTREE: mobj_type = 241;
pub const MT_FHZPINKTREE: mobj_type = 240;
pub const MT_FHZTREE: mobj_type = 239;
pub const MT_CHECKERSUNSETTREE: mobj_type = 238;
pub const MT_CHECKERTREE: mobj_type = 237;
pub const MT_GFZCHERRYTREE: mobj_type = 236;
pub const MT_GFZBERRYTREE: mobj_type = 235;
pub const MT_GFZTREE: mobj_type = 234;
pub const MT_BUSH: mobj_type = 233;
pub const MT_BERRYBUSH: mobj_type = 232;
pub const MT_BLUEBERRYBUSH: mobj_type = 231;
pub const MT_GFZFLOWER3: mobj_type = 230;
pub const MT_GFZFLOWER2: mobj_type = 229;
pub const MT_GFZFLOWER1: mobj_type = 228;
pub const MT_TUTORIALFLOWERF: mobj_type = 227;
pub const MT_TUTORIALFLOWER: mobj_type = 226;
pub const MT_TUTORIALLEAF: mobj_type = 225;
pub const MT_TUTORIALPLANT: mobj_type = 224;
pub const MT_LETTER: mobj_type = 223;
pub const MT_DEMONFIRE: mobj_type = 222;
pub const MT_ARROW: mobj_type = 221;
pub const MT_CANNONBALLDECOR: mobj_type = 220;
pub const MT_CANNONBALL: mobj_type = 219;
pub const MT_TURRETLASER: mobj_type = 218;
pub const MT_JETTBULLET: mobj_type = 217;
pub const MT_MINE: mobj_type = 216;
pub const MT_ENERGYBALL: mobj_type = 215;
pub const MT_TORPEDO2: mobj_type = 214;
pub const MT_TORPEDO: mobj_type = 213;
pub const MT_LASER: mobj_type = 212;
pub const MT_ROCKET: mobj_type = 211;
pub const MT_THUNDERCOIN_ICON: mobj_type = 210;
pub const MT_BUBBLEWRAP_ICON: mobj_type = 209;
pub const MT_FLAMEAURA_ICON: mobj_type = 208;
pub const MT_SCORE10K_ICON: mobj_type = 207;
pub const MT_SCORE1K_ICON: mobj_type = 206;
pub const MT_RECYCLER_ICON: mobj_type = 205;
pub const MT_GRAVITY_ICON: mobj_type = 204;
pub const MT_MIXUP_ICON: mobj_type = 203;
pub const MT_EGGMAN_ICON: mobj_type = 202;
pub const MT_1UP_ICON: mobj_type = 201;
pub const MT_INVULN_ICON: mobj_type = 200;
pub const MT_SNEAKERS_ICON: mobj_type = 199;
pub const MT_ELEMENTAL_ICON: mobj_type = 198;
pub const MT_WHIRLWIND_ICON: mobj_type = 197;
pub const MT_ARMAGEDDON_ICON: mobj_type = 196;
pub const MT_FORCE_ICON: mobj_type = 195;
pub const MT_ATTRACT_ICON: mobj_type = 194;
pub const MT_PITY_ICON: mobj_type = 193;
pub const MT_RING_ICON: mobj_type = 192;
pub const MT_RING_BLUEBOX: mobj_type = 191;
pub const MT_RING_REDBOX: mobj_type = 190;
pub const MT_THUNDERCOIN_GOLDBOX: mobj_type = 189;
pub const MT_BUBBLEWRAP_GOLDBOX: mobj_type = 188;
pub const MT_FLAMEAURA_GOLDBOX: mobj_type = 187;
pub const MT_GRAVITY_GOLDBOX: mobj_type = 186;
pub const MT_EGGMAN_GOLDBOX: mobj_type = 185;
pub const MT_INVULN_GOLDBOX: mobj_type = 184;
pub const MT_SNEAKERS_GOLDBOX: mobj_type = 183;
pub const MT_ELEMENTAL_GOLDBOX: mobj_type = 182;
pub const MT_WHIRLWIND_GOLDBOX: mobj_type = 181;
pub const MT_ARMAGEDDON_GOLDBOX: mobj_type = 180;
pub const MT_FORCE_GOLDBOX: mobj_type = 179;
pub const MT_ATTRACT_GOLDBOX: mobj_type = 178;
pub const MT_PITY_GOLDBOX: mobj_type = 177;
pub const MT_THUNDERCOIN_BOX: mobj_type = 176;
pub const MT_BUBBLEWRAP_BOX: mobj_type = 175;
pub const MT_FLAMEAURA_BOX: mobj_type = 174;
pub const MT_SCORE10K_BOX: mobj_type = 173;
pub const MT_SCORE1K_BOX: mobj_type = 172;
pub const MT_RECYCLER_BOX: mobj_type = 171;
pub const MT_GRAVITY_BOX: mobj_type = 170;
pub const MT_MYSTERY_BOX: mobj_type = 169;
pub const MT_MIXUP_BOX: mobj_type = 168;
pub const MT_EGGMAN_BOX: mobj_type = 167;
pub const MT_1UP_BOX: mobj_type = 166;
pub const MT_INVULN_BOX: mobj_type = 165;
pub const MT_SNEAKERS_BOX: mobj_type = 164;
pub const MT_ELEMENTAL_BOX: mobj_type = 163;
pub const MT_WHIRLWIND_BOX: mobj_type = 162;
pub const MT_ARMAGEDDON_BOX: mobj_type = 161;
pub const MT_FORCE_BOX: mobj_type = 160;
pub const MT_ATTRACT_BOX: mobj_type = 159;
pub const MT_PITY_BOX: mobj_type = 158;
pub const MT_RING_BOX: mobj_type = 157;
pub const MT_BOXSPARKLE: mobj_type = 156;
pub const MT_CANNONLAUNCHER: mobj_type = 155;
pub const MT_BLASTEXECUTOR: mobj_type = 154;
pub const MT_BIGMINE: mobj_type = 153;
pub const MT_STARPOST: mobj_type = 152;
pub const MT_WALLSPIKEBASE: mobj_type = 151;
pub const MT_WALLSPIKE: mobj_type = 150;
pub const MT_SPIKE: mobj_type = 149;
pub const MT_SPINFIRE: mobj_type = 148;
pub const MT_SPIKEBALL: mobj_type = 147;
pub const MT_SIGN: mobj_type = 146;
pub const MT_BUBBLES: mobj_type = 145;
pub const MT_REDBOOSTER: mobj_type = 144;
pub const MT_YELLOWBOOSTER: mobj_type = 143;
pub const MT_BOOSTERROLLER: mobj_type = 142;
pub const MT_BOOSTERSEG: mobj_type = 141;
pub const MT_BLUEHORIZ: mobj_type = 140;
pub const MT_REDHORIZ: mobj_type = 139;
pub const MT_YELLOWHORIZ: mobj_type = 138;
pub const MT_BLUEDIAG: mobj_type = 137;
pub const MT_REDDIAG: mobj_type = 136;
pub const MT_YELLOWDIAG: mobj_type = 135;
pub const MT_BLUESPRING: mobj_type = 134;
pub const MT_REDSPRING: mobj_type = 133;
pub const MT_YELLOWSPRING: mobj_type = 132;
pub const MT_BALLOON: mobj_type = 131;
pub const MT_BUMPER: mobj_type = 130;
pub const MT_STEAM: mobj_type = 129;
pub const MT_FAN: mobj_type = 128;
pub const MT_FLINGEMERALD: mobj_type = 127;
pub const MT_EMERALDSPAWN: mobj_type = 126;
pub const MT_EMERHUNT: mobj_type = 125;
pub const MT_EMERALD7: mobj_type = 124;
pub const MT_EMERALD6: mobj_type = 123;
pub const MT_EMERALD5: mobj_type = 122;
pub const MT_EMERALD4: mobj_type = 121;
pub const MT_EMERALD3: mobj_type = 120;
pub const MT_EMERALD2: mobj_type = 119;
pub const MT_EMERALD1: mobj_type = 118;
pub const MT_EMBLEM: mobj_type = 117;
pub const MT_BLUEFLAG: mobj_type = 116;
pub const MT_REDFLAG: mobj_type = 115;
pub const MT_TOKEN: mobj_type = 114;
pub const MT_BLUETEAMRING: mobj_type = 113;
pub const MT_REDTEAMRING: mobj_type = 112;
pub const MT_BOMBSPHERE: mobj_type = 111;
pub const MT_FLINGBLUESPHERE: mobj_type = 110;
pub const MT_BLUESPHERE: mobj_type = 109;
pub const MT_FLINGRING: mobj_type = 108;
pub const MT_RING: mobj_type = 107;
pub const MT_MSGATHER: mobj_type = 106;
pub const MT_MSSHIELD_FRONT: mobj_type = 105;
pub const MT_METALSONIC_BATTLE: mobj_type = 104;
pub const MT_METALSONIC_RACE: mobj_type = 103;
pub const MT_CYBRAKDEMON_VILE_EXPLOSION: mobj_type = 102;
pub const MT_CYBRAKDEMON_NAPALM_FLAMES: mobj_type = 101;
pub const MT_CYBRAKDEMON_NAPALM_BOMB_SMALL: mobj_type = 100;
pub const MT_CYBRAKDEMON_NAPALM_BOMB_LARGE: mobj_type = 99;
pub const MT_CYBRAKDEMON_TARGET_DOT: mobj_type = 98;
pub const MT_CYBRAKDEMON_TARGET_RETICULE: mobj_type = 97;
pub const MT_CYBRAKDEMON_FLAMEREST: mobj_type = 96;
pub const MT_CYBRAKDEMON_FLAMESHOT: mobj_type = 95;
pub const MT_CYBRAKDEMON_MISSILE: mobj_type = 94;
pub const MT_CYBRAKDEMON_ELECTRIC_BARRIER: mobj_type = 93;
pub const MT_CYBRAKDEMON: mobj_type = 92;
pub const MT_BLACKEGGMAN_MISSILE: mobj_type = 91;
pub const MT_BLACKEGGMAN_GOOPFIRE: mobj_type = 90;
pub const MT_BLACKEGGMAN_HELPER: mobj_type = 89;
pub const MT_BLACKEGGMAN: mobj_type = 88;
pub const MT_FANGWAYPOINT: mobj_type = 87;
pub const MT_FSGNB: mobj_type = 86;
pub const MT_FSGNA: mobj_type = 85;
pub const MT_TNTDUST: mobj_type = 84;
pub const MT_FBOMB: mobj_type = 83;
pub const MT_PROJECTORLIGHT: mobj_type = 82;
pub const MT_VWREB: mobj_type = 81;
pub const MT_VWREF: mobj_type = 80;
pub const MT_BROKENROBOT: mobj_type = 79;
pub const MT_FANG: mobj_type = 78;
pub const MT_EGGROBO1JET: mobj_type = 77;
pub const MT_EGGROBO1: mobj_type = 76;
pub const MT_JETFLAME: mobj_type = 75;
pub const MT_EGGMOBILE4_MACE: mobj_type = 74;
pub const MT_EGGMOBILE4: mobj_type = 73;
pub const MT_SHOCKWAVE: mobj_type = 72;
pub const MT_FAKEMOBILE: mobj_type = 71;
pub const MT_EGGMOBILE3: mobj_type = 70;
pub const MT_GOOPTRAIL: mobj_type = 69;
pub const MT_GOOP: mobj_type = 68;
pub const MT_EGGMOBILE2_POGO: mobj_type = 67;
pub const MT_EGGMOBILE2: mobj_type = 66;
pub const MT_EGGMOBILE_FIRE: mobj_type = 65;
pub const MT_EGGMOBILE_TARGET: mobj_type = 64;
pub const MT_EGGMOBILE_BALL: mobj_type = 63;
pub const MT_JETFUME1: mobj_type = 62;
pub const MT_EGGMOBILE: mobj_type = 61;
pub const MT_BOSSJUNK: mobj_type = 60;
pub const MT_BOSS9GATHERPOINT: mobj_type = 59;
pub const MT_BOSS3WAYPOINT: mobj_type = 58;
pub const MT_EGGTRAP: mobj_type = 57;
pub const MT_BOSSFLYPOINT: mobj_type = 56;
pub const MT_SONIC3KBOSSEXPLODE: mobj_type = 55;
pub const MT_BOSSEXPLODE: mobj_type = 54;
pub const MT_DRAGONMINE: mobj_type = 53;
pub const MT_DRAGONTAIL: mobj_type = 52;
pub const MT_DRAGONWING: mobj_type = 51;
pub const MT_DRAGONBOMBER: mobj_type = 50;
pub const MT_PTERABYTE: mobj_type = 49;
pub const MT_PTERABYTEWAYPOINT: mobj_type = 48;
pub const MT_PTERABYTESPAWNER: mobj_type = 47;
pub const MT_PYREFLY_FIRE: mobj_type = 46;
pub const MT_PYREFLY: mobj_type = 45;
pub const MT_CANARIVORE_GAS: mobj_type = 44;
pub const MT_CANARIVORE: mobj_type = 43;
pub const MT_UNIBALL: mobj_type = 42;
pub const MT_UNIDUS: mobj_type = 41;
pub const MT_YELLOWSHELL: mobj_type = 40;
pub const MT_SPRINGSHELL: mobj_type = 39;
pub const MT_MINUSDIRT: mobj_type = 38;
pub const MT_MINUS: mobj_type = 37;
pub const MT_SNAPPER_HEAD: mobj_type = 36;
pub const MT_SNAPPER_LEG: mobj_type = 35;
pub const MT_GSNAPPER: mobj_type = 34;
pub const MT_EGGSHIELD: mobj_type = 33;
pub const MT_EGGGUARD: mobj_type = 32;
pub const MT_FACESTABBERSPEAR: mobj_type = 31;
pub const MT_FACESTABBER: mobj_type = 30;
pub const MT_ROBOHOOD: mobj_type = 29;
pub const MT_POINTYBALL: mobj_type = 28;
pub const MT_POINTY: mobj_type = 27;
pub const MT_VULTURE: mobj_type = 26;
pub const MT_SNAILER: mobj_type = 25;
pub const MT_JETJAW: mobj_type = 24;
pub const MT_BANPSPRING: mobj_type = 23;
pub const MT_BANPYURA: mobj_type = 22;
pub const MT_CRUSHCHAIN: mobj_type = 21;
pub const MT_CRUSHCLAW: mobj_type = 20;
pub const MT_CRUSHSTACEAN: mobj_type = 19;
pub const MT_SPINCUSHION: mobj_type = 18;
pub const MT_POPUPTURRET: mobj_type = 17;
pub const MT_TURRET: mobj_type = 16;
pub const MT_SKIM: mobj_type = 15;
pub const MT_DETON: mobj_type = 14;
pub const MT_CRAWLACOMMANDER: mobj_type = 13;
pub const MT_JETTGUNNER: mobj_type = 12;
pub const MT_JETTBOMBER: mobj_type = 11;
pub const MT_REDBUZZ: mobj_type = 10;
pub const MT_GOLDBUZZ: mobj_type = 9;
pub const MT_GFZFISH: mobj_type = 8;
pub const MT_REDCRAWLA: mobj_type = 7;
pub const MT_BLUECRAWLA: mobj_type = 6;
pub const MT_METALJETFUME: mobj_type = 5;
pub const MT_TAILSOVERLAY: mobj_type = 4;
pub const MT_PLAYER: mobj_type = 3;
pub const MT_THOK: mobj_type = 2;
pub const MT_UNKNOWN: mobj_type = 1;
pub const MT_NULL: mobj_type = 0;
pub type mobjtype_t = mobj_type;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const PA_RIDE: C2RustUnnamed_7 = 13;
pub const PA_ABILITY2: C2RustUnnamed_7 = 12;
pub const PA_ABILITY: C2RustUnnamed_7 = 11;
pub const PA_FALL: C2RustUnnamed_7 = 10;
pub const PA_SPRING: C2RustUnnamed_7 = 9;
pub const PA_JUMP: C2RustUnnamed_7 = 8;
pub const PA_ROLL: C2RustUnnamed_7 = 7;
pub const PA_PAIN: C2RustUnnamed_7 = 6;
pub const PA_DASH: C2RustUnnamed_7 = 5;
pub const PA_RUN: C2RustUnnamed_7 = 4;
pub const PA_WALK: C2RustUnnamed_7 = 3;
pub const PA_EDGE: C2RustUnnamed_7 = 2;
pub const PA_IDLE: C2RustUnnamed_7 = 1;
pub const PA_ETC: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const PST_REBORN: C2RustUnnamed_8 = 2;
pub const PST_DEAD: C2RustUnnamed_8 = 1;
pub const PST_LIVE: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const FOF_SPLAT: C2RustUnnamed_9 = 1073741824;
pub const FOF_BOUNCY: C2RustUnnamed_9 = 536870912;
pub const FOF_COLORMAPONLY: C2RustUnnamed_9 = 268435456;
pub const FOF_RIPPLE: C2RustUnnamed_9 = 134217728;
pub const FOF_INTANGIBLEFLATS: C2RustUnnamed_9 = 100663296;
pub const FOF_REVERSEPLATFORM: C2RustUnnamed_9 = 67108864;
pub const FOF_PLATFORM: C2RustUnnamed_9 = 33554432;
pub const FOF_QUICKSAND: C2RustUnnamed_9 = 16777216;
pub const FOF_BUSTUP: C2RustUnnamed_9 = 8388608;
pub const FOF_MARIO: C2RustUnnamed_9 = 4194304;
pub const FOF_GOOWATER: C2RustUnnamed_9 = 2097152;
pub const FOF_CRUMBLE: C2RustUnnamed_9 = 1048576;
pub const FOF_NORETURN: C2RustUnnamed_9 = 524288;
pub const FOF_FLOATBOB: C2RustUnnamed_9 = 262144;
pub const FOF_DOUBLESHADOW: C2RustUnnamed_9 = 131072;
pub const FOF_INVERTSIDES: C2RustUnnamed_9 = 65536;
pub const FOF_ALLSIDES: C2RustUnnamed_9 = 32768;
pub const FOF_INVERTPLANES: C2RustUnnamed_9 = 16384;
pub const FOF_FOG: C2RustUnnamed_9 = 8192;
pub const FOF_TRANSLUCENT: C2RustUnnamed_9 = 4096;
pub const FOF_EXTRA: C2RustUnnamed_9 = 2048;
pub const FOF_BOTHPLANES: C2RustUnnamed_9 = 1024;
pub const FOF_CUTSPRITES: C2RustUnnamed_9 = 512;
pub const FOF_CUTLEVEL: C2RustUnnamed_9 = 384;
pub const FOF_CUTEXTRA: C2RustUnnamed_9 = 256;
pub const FOF_CUTSOLIDS: C2RustUnnamed_9 = 128;
pub const FOF_NOSHADE: C2RustUnnamed_9 = 64;
pub const FOF_SWIMMABLE: C2RustUnnamed_9 = 32;
pub const FOF_RENDERALL: C2RustUnnamed_9 = 24;
pub const FOF_RENDERPLANES: C2RustUnnamed_9 = 16;
pub const FOF_RENDERSIDES: C2RustUnnamed_9 = 8;
pub const FOF_SOLID: C2RustUnnamed_9 = 6;
pub const FOF_BLOCKOTHERS: C2RustUnnamed_9 = 4;
pub const FOF_BLOCKPLAYER: C2RustUnnamed_9 = 2;
pub const FOF_EXISTS: C2RustUnnamed_9 = 1;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const FB_ONLYBOTTOM: C2RustUnnamed_10 = 4;
pub const FB_EXECUTOR: C2RustUnnamed_10 = 2;
pub const FB_PUSHABLES: C2RustUnnamed_10 = 1;
pub type C2RustUnnamed_11 = libc::c_uint;
pub const NUMTRANSMAPS: C2RustUnnamed_11 = 10;
pub const tr_trans90: C2RustUnnamed_11 = 9;
pub const tr_trans80: C2RustUnnamed_11 = 8;
pub const tr_trans70: C2RustUnnamed_11 = 7;
pub const tr_trans60: C2RustUnnamed_11 = 6;
pub const tr_trans50: C2RustUnnamed_11 = 5;
pub const tr_trans40: C2RustUnnamed_11 = 4;
pub const tr_trans30: C2RustUnnamed_11 = 3;
pub const tr_trans20: C2RustUnnamed_11 = 2;
pub const tr_trans10: C2RustUnnamed_11 = 1;
pub type C2RustUnnamed_12 = libc::c_int;
pub const NUMDIRS: C2RustUnnamed_12 = 8;
pub const DI_SOUTHEAST: C2RustUnnamed_12 = 7;
pub const DI_SOUTH: C2RustUnnamed_12 = 6;
pub const DI_SOUTHWEST: C2RustUnnamed_12 = 5;
pub const DI_WEST: C2RustUnnamed_12 = 4;
pub const DI_NORTHWEST: C2RustUnnamed_12 = 3;
pub const DI_NORTH: C2RustUnnamed_12 = 2;
pub const DI_NORTHEAST: C2RustUnnamed_12 = 1;
pub const DI_EAST: C2RustUnnamed_12 = 0;
pub const DI_NODIR: C2RustUnnamed_12 = -1;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const BT_CUSTOM3: C2RustUnnamed_13 = 32768;
pub const BT_CUSTOM2: C2RustUnnamed_13 = 16384;
pub const BT_CUSTOM1: C2RustUnnamed_13 = 8192;
pub const BT_FIRENORMAL: C2RustUnnamed_13 = 4096;
pub const BT_JUMP: C2RustUnnamed_13 = 2048;
pub const BT_TOSSFLAG: C2RustUnnamed_13 = 1024;
pub const BT_CAMRIGHT: C2RustUnnamed_13 = 512;
pub const BT_CAMLEFT: C2RustUnnamed_13 = 256;
pub const BT_SPIN: C2RustUnnamed_13 = 128;
pub const BT_ATTACK: C2RustUnnamed_13 = 64;
pub const BT_WEAPONPREV: C2RustUnnamed_13 = 32;
pub const BT_WEAPONNEXT: C2RustUnnamed_13 = 16;
pub const BT_WEAPONMASK: C2RustUnnamed_13 = 15;
pub type C2RustUnnamed_14 = libc::c_uint;
pub const SF_NOSHIELDABILITY: C2RustUnnamed_14 = 524288;
pub const SF_CANBUSTWALLS: C2RustUnnamed_14 = 262144;
pub const SF_NOSUPERJUMPBOOST: C2RustUnnamed_14 = 131072;
pub const SF_NOSUPERSPRITES: C2RustUnnamed_14 = 65536;
pub const SF_NONIGHTSSUPER: C2RustUnnamed_14 = 32768;
pub const SF_NONIGHTSROTATION: C2RustUnnamed_14 = 16384;
pub const SF_MULTIABILITY: C2RustUnnamed_14 = 8192;
pub const SF_FASTEDGE: C2RustUnnamed_14 = 4096;
pub const SF_DASHMODE: C2RustUnnamed_14 = 2048;
pub const SF_MACHINE: C2RustUnnamed_14 = 1024;
pub const SF_MARIODAMAGE: C2RustUnnamed_14 = 768;
pub const SF_STOMPDAMAGE: C2RustUnnamed_14 = 512;
pub const SF_NOJUMPDAMAGE: C2RustUnnamed_14 = 256;
pub const SF_NOJUMPSPIN: C2RustUnnamed_14 = 128;
pub const SF_RUNONWATER: C2RustUnnamed_14 = 64;
pub const SF_NOSPEEDADJUST: C2RustUnnamed_14 = 32;
pub const SF_NOSKID: C2RustUnnamed_14 = 16;
pub const SF_HIRES: C2RustUnnamed_14 = 8;
pub const SF_NOSPINDASHDUST: C2RustUnnamed_14 = 4;
pub const SF_NOSUPERSPIN: C2RustUnnamed_14 = 2;
pub const SF_SUPER: C2RustUnnamed_14 = 1;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const CA_TWINSPIN: C2RustUnnamed_15 = 15;
pub const CA_BOUNCE: C2RustUnnamed_15 = 14;
pub const CA_JUMPTHOK: C2RustUnnamed_15 = 13;
pub const CA_AIRDRILL: C2RustUnnamed_15 = 12;
pub const CA_JUMPBOOST: C2RustUnnamed_15 = 11;
pub const CA_FALLSWITCH: C2RustUnnamed_15 = 10;
pub const CA_TELEKINESIS: C2RustUnnamed_15 = 9;
pub const CA_SLOWFALL: C2RustUnnamed_15 = 8;
pub const CA_FLOAT: C2RustUnnamed_15 = 7;
pub const CA_DOUBLEJUMP: C2RustUnnamed_15 = 6;
pub const CA_SWIM: C2RustUnnamed_15 = 5;
pub const CA_HOMINGTHOK: C2RustUnnamed_15 = 4;
pub const CA_GLIDEANDCLIMB: C2RustUnnamed_15 = 3;
pub const CA_FLY: C2RustUnnamed_15 = 2;
pub const CA_THOK: C2RustUnnamed_15 = 1;
pub const CA_NONE: C2RustUnnamed_15 = 0;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const CA2_MELEE: C2RustUnnamed_16 = 3;
pub const CA2_GUNSLINGER: C2RustUnnamed_16 = 2;
pub const CA2_SPINDASH: C2RustUnnamed_16 = 1;
pub const CA2_NONE: C2RustUnnamed_16 = 0;
pub type C2RustUnnamed_17 = libc::c_int;
pub const SH_NOSTACK: C2RustUnnamed_17 = -513;
pub const SH_STACK: C2RustUnnamed_17 = 512;
pub const SH_FIREFLOWER: C2RustUnnamed_17 = 512;
pub const SH_FORCEHP: C2RustUnnamed_17 = 255;
pub const SH_FORCE: C2RustUnnamed_17 = 256;
pub const SH_THUNDERCOIN: C2RustUnnamed_17 = 4098;
pub const SH_BUBBLEWRAP: C2RustUnnamed_17 = 2049;
pub const SH_FLAMEAURA: C2RustUnnamed_17 = 1025;
pub const SH_ELEMENTAL: C2RustUnnamed_17 = 3073;
pub const SH_ATTRACT: C2RustUnnamed_17 = 4097;
pub const SH_PINK: C2RustUnnamed_17 = 4;
pub const SH_ARMAGEDDON: C2RustUnnamed_17 = 3;
pub const SH_WHIRLWIND: C2RustUnnamed_17 = 2;
pub const SH_PITY: C2RustUnnamed_17 = 1;
pub const SH_PROTECTSPIKE: C2RustUnnamed_17 = 8192;
pub const SH_PROTECTELECTRIC: C2RustUnnamed_17 = 4096;
pub const SH_PROTECTWATER: C2RustUnnamed_17 = 2048;
pub const SH_PROTECTFIRE: C2RustUnnamed_17 = 1024;
pub const SH_NONE: C2RustUnnamed_17 = 0;
pub type C2RustUnnamed_18 = libc::c_uint;
pub const CR_FAN: C2RustUnnamed_18 = 13;
pub const CR_DUSTDEVIL: C2RustUnnamed_18 = 12;
pub const CR_PTERABYTE: C2RustUnnamed_18 = 11;
pub const CR_ROLLOUT: C2RustUnnamed_18 = 10;
pub const CR_MINECART: C2RustUnnamed_18 = 9;
pub const CR_MACESPIN: C2RustUnnamed_18 = 8;
pub const CR_ROPEHANG: C2RustUnnamed_18 = 7;
pub const CR_ZOOMTUBE: C2RustUnnamed_18 = 6;
pub const CR_BRAKGOOP: C2RustUnnamed_18 = 5;
pub const CR_NIGHTSFALL: C2RustUnnamed_18 = 4;
pub const CR_NIGHTSMODE: C2RustUnnamed_18 = 3;
pub const CR_PLAYER: C2RustUnnamed_18 = 2;
pub const CR_GENERIC: C2RustUnnamed_18 = 1;
pub const CR_NONE: C2RustUnnamed_18 = 0;
pub type C2RustUnnamed_19 = libc::c_uint;
pub const STR_METAL: C2RustUnnamed_19 = 4224;
pub const STR_BOUNCE: C2RustUnnamed_19 = 521;
pub const STR_MELEE: C2RustUnnamed_19 = 6979;
pub const STR_TWINSPIN: C2RustUnnamed_19 = 7967;
pub const STR_GLIDE: C2RustUnnamed_19 = 3;
pub const STR_FLY: C2RustUnnamed_19 = 17;
pub const STR_BUST: C2RustUnnamed_19 = 1792;
pub const STR_ATTACK: C2RustUnnamed_19 = 30;
pub const STR_SPIKE: C2RustUnnamed_19 = 4096;
pub const STR_SPRING: C2RustUnnamed_19 = 2048;
pub const STR_CEILING: C2RustUnnamed_19 = 1024;
pub const STR_FLOOR: C2RustUnnamed_19 = 512;
pub const STR_WALL: C2RustUnnamed_19 = 256;
pub const STR_DASH: C2RustUnnamed_19 = 128;
pub const STR_HEAVY: C2RustUnnamed_19 = 64;
pub const STR_GUARD: C2RustUnnamed_19 = 32;
pub const STR_UPPER: C2RustUnnamed_19 = 16;
pub const STR_STOMP: C2RustUnnamed_19 = 8;
pub const STR_TAIL: C2RustUnnamed_19 = 4;
pub const STR_PUNCH: C2RustUnnamed_19 = 2;
pub const STR_ANIM: C2RustUnnamed_19 = 1;
pub const STR_NONE: C2RustUnnamed_19 = 0;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const RW_RAIL: C2RustUnnamed_20 = 32;
pub const RW_EXPLODE: C2RustUnnamed_20 = 16;
pub const RW_GRENADE: C2RustUnnamed_20 = 8;
pub const RW_SCATTER: C2RustUnnamed_20 = 4;
pub const RW_BOUNCE: C2RustUnnamed_20 = 2;
pub const RW_AUTO: C2RustUnnamed_20 = 1;
pub type C2RustUnnamed_21 = libc::c_uint;
pub const BOT_MPAI: C2RustUnnamed_21 = 3;
pub const BOT_2PHUMAN: C2RustUnnamed_21 = 2;
pub const BOT_2PAI: C2RustUnnamed_21 = 1;
pub const BOT_NONE: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_22 = libc::c_uint;
pub const MA_INGAME: C2RustUnnamed_22 = 8;
pub const MA_NOCUTSCENES: C2RustUnnamed_22 = 4;
pub const MA_INIT: C2RustUnnamed_22 = 2;
pub const MA_RUNNING: C2RustUnnamed_22 = 1;
pub type C2RustUnnamed_23 = libc::c_uint;
pub const KR_LEAVE: C2RustUnnamed_23 = 6;
pub const KR_BAN: C2RustUnnamed_23 = 5;
pub const KR_TIMEOUT: C2RustUnnamed_23 = 4;
pub const KR_SYNCH: C2RustUnnamed_23 = 3;
pub const KR_PINGLIMIT: C2RustUnnamed_23 = 2;
pub const KR_KICK: C2RustUnnamed_23 = 1;
pub type C2RustUnnamed_24 = libc::c_uint;
pub const POF_SPLAT: C2RustUnnamed_24 = 8192;
pub const POF_NOSPECIALS: C2RustUnnamed_24 = 4096;
pub const POF_ONESIDE: C2RustUnnamed_24 = 2048;
pub const POF_LDEXEC: C2RustUnnamed_24 = 1024;
pub const POF_PUSHABLESTOP: C2RustUnnamed_24 = 512;
pub const POF_INVERTPLANESONLY: C2RustUnnamed_24 = 256;
pub const POF_INVERTPLANES: C2RustUnnamed_24 = 128;
pub const POF_INVERT: C2RustUnnamed_24 = 64;
pub const POF_RENDERALL: C2RustUnnamed_24 = 56;
pub const POF_RENDERPLANES: C2RustUnnamed_24 = 48;
pub const POF_RENDERBOTTOM: C2RustUnnamed_24 = 32;
pub const POF_RENDERTOP: C2RustUnnamed_24 = 16;
pub const POF_RENDERSIDES: C2RustUnnamed_24 = 8;
pub const POF_TESTHEIGHT: C2RustUnnamed_24 = 4;
pub const POF_SOLID: C2RustUnnamed_24 = 3;
pub const POF_CLIPPLANES: C2RustUnnamed_24 = 2;
pub const POF_CLIPLINES: C2RustUnnamed_24 = 1;
pub type C2RustUnnamed_25 = libc::c_uint;
pub const FF_OLD_COLORMAPONLY: C2RustUnnamed_25 = 2147483648;
pub const FF_OLD_RIPPLE: C2RustUnnamed_25 = 1073741824;
pub const FF_OLD_STRONGBUST: C2RustUnnamed_25 = 536870912;
pub const FF_OLD_SPINBUST: C2RustUnnamed_25 = 268435456;
pub const FF_OLD_SHATTER: C2RustUnnamed_25 = 134217728;
pub const FF_OLD_INTANGIBLEFLATS: C2RustUnnamed_25 = 100663296;
pub const FF_OLD_REVERSEPLATFORM: C2RustUnnamed_25 = 67108864;
pub const FF_OLD_PLATFORM: C2RustUnnamed_25 = 33554432;
pub const FF_OLD_QUICKSAND: C2RustUnnamed_25 = 16777216;
pub const FF_OLD_BUSTUP: C2RustUnnamed_25 = 8388608;
pub const FF_OLD_MARIO: C2RustUnnamed_25 = 4194304;
pub const FF_OLD_GOOWATER: C2RustUnnamed_25 = 2097152;
pub const FF_OLD_SHATTERBOTTOM: C2RustUnnamed_25 = 2097152;
pub const FF_OLD_CRUMBLE: C2RustUnnamed_25 = 1048576;
pub const FF_OLD_NORETURN: C2RustUnnamed_25 = 524288;
pub const FF_OLD_FLOATBOB: C2RustUnnamed_25 = 262144;
pub const FF_OLD_DOUBLESHADOW: C2RustUnnamed_25 = 131072;
pub const FF_OLD_INVERTSIDES: C2RustUnnamed_25 = 65536;
pub const FF_OLD_ALLSIDES: C2RustUnnamed_25 = 32768;
pub const FF_OLD_INVERTPLANES: C2RustUnnamed_25 = 16384;
pub const FF_OLD_FOG: C2RustUnnamed_25 = 8192;
pub const FF_OLD_TRANSLUCENT: C2RustUnnamed_25 = 4096;
pub const FF_OLD_EXTRA: C2RustUnnamed_25 = 2048;
pub const FF_OLD_BOTHPLANES: C2RustUnnamed_25 = 1024;
pub const FF_OLD_CUTSPRITES: C2RustUnnamed_25 = 512;
pub const FF_OLD_CUTLEVEL: C2RustUnnamed_25 = 384;
pub const FF_OLD_CUTEXTRA: C2RustUnnamed_25 = 256;
pub const FF_OLD_CUTSOLIDS: C2RustUnnamed_25 = 128;
pub const FF_OLD_NOSHADE: C2RustUnnamed_25 = 64;
pub const FF_OLD_SWIMMABLE: C2RustUnnamed_25 = 32;
pub const FF_OLD_RENDERALL: C2RustUnnamed_25 = 24;
pub const FF_OLD_RENDERPLANES: C2RustUnnamed_25 = 16;
pub const FF_OLD_RENDERSIDES: C2RustUnnamed_25 = 8;
pub const FF_OLD_SOLID: C2RustUnnamed_25 = 6;
pub const FF_OLD_BLOCKOTHERS: C2RustUnnamed_25 = 4;
pub const FF_OLD_BLOCKPLAYER: C2RustUnnamed_25 = 2;
pub const FF_OLD_EXISTS: C2RustUnnamed_25 = 1;
pub type C2RustUnnamed_26 = libc::c_uint;
pub const BT_STRONG: C2RustUnnamed_26 = 3;
pub const BT_REGULAR: C2RustUnnamed_26 = 2;
pub const BT_SPINBUST: C2RustUnnamed_26 = 1;
pub const BT_TOUCH: C2RustUnnamed_26 = 0;
pub type C2RustUnnamed_27 = libc::c_uint;
pub const SL_DYNAMIC: C2RustUnnamed_27 = 2;
pub const SL_NOPHYSICS: C2RustUnnamed_27 = 1;
pub type patchalphastyle = libc::c_uint;
pub const AST_FOG: patchalphastyle = 7;
pub const AST_OVERLAY: patchalphastyle = 6;
pub const AST_MODULATE: patchalphastyle = 5;
pub const AST_REVERSESUBTRACT: patchalphastyle = 4;
pub const AST_SUBTRACT: patchalphastyle = 3;
pub const AST_ADD: patchalphastyle = 2;
pub const AST_TRANSLUCENT: patchalphastyle = 1;
pub const AST_COPY: patchalphastyle = 0;
pub type C2RustUnnamed_28 = libc::c_uint;
pub const RF_DROPSHADOW: C2RustUnnamed_28 = 49664;
pub const RF_SHADOWEFFECTS: C2RustUnnamed_28 = 32768;
pub const RF_SHADOWDRAW: C2RustUnnamed_28 = 16384;
pub const RF_FLOORSPRITE: C2RustUnnamed_28 = 8192;
pub const RF_PAPERSPRITE: C2RustUnnamed_28 = 4096;
pub const RF_SPRITETYPEMASK: C2RustUnnamed_28 = 12288;
pub const RF_NOCOLORMAPS: C2RustUnnamed_28 = 1024;
pub const RF_SEMIBRIGHT: C2RustUnnamed_28 = 768;
pub const RF_FULLDARK: C2RustUnnamed_28 = 512;
pub const RF_FULLBRIGHT: C2RustUnnamed_28 = 256;
pub const RF_BRIGHTMASK: C2RustUnnamed_28 = 768;
pub const RF_NOSPLATROLLANGLE: C2RustUnnamed_28 = 128;
pub const RF_NOSPLATBILLBOARD: C2RustUnnamed_28 = 64;
pub const RF_OBJECTSLOPESPLAT: C2RustUnnamed_28 = 32;
pub const RF_SLOPESPLAT: C2RustUnnamed_28 = 16;
pub const RF_SPLATMASK: C2RustUnnamed_28 = 240;
pub const RF_FLIPOFFSETS: C2RustUnnamed_28 = 8;
pub const RF_ABSOLUTEOFFSETS: C2RustUnnamed_28 = 4;
pub const RF_VERTICALFLIP: C2RustUnnamed_28 = 2;
pub const RF_HORIZONTALFLIP: C2RustUnnamed_28 = 1;
pub type C2RustUnnamed_29 = libc::c_uint;
pub const ROTAXIS_Z: C2RustUnnamed_29 = 2;
pub const ROTAXIS_Y: C2RustUnnamed_29 = 1;
pub const ROTAXIS_X: C2RustUnnamed_29 = 0;
pub type C2RustUnnamed_30 = libc::c_uint;
pub const NUMJINGLES: C2RustUnnamed_30 = 12;
pub const JT_SSTIMEOUT: C2RustUnnamed_30 = 11;
pub const JT_NIGHTSTIMEOUT: C2RustUnnamed_30 = 10;
pub const JT_GOVER: C2RustUnnamed_30 = 9;
pub const JT_SUPER: C2RustUnnamed_30 = 8;
pub const JT_DROWN: C2RustUnnamed_30 = 7;
pub const JT_MINV: C2RustUnnamed_30 = 6;
pub const JT_INV: C2RustUnnamed_30 = 5;
pub const JT_SHOES: C2RustUnnamed_30 = 4;
pub const JT_1UP: C2RustUnnamed_30 = 3;
pub const JT_MASTER: C2RustUnnamed_30 = 2;
pub const JT_OTHER: C2RustUnnamed_30 = 1;
pub const JT_NONE: C2RustUnnamed_30 = 0;
pub type lua_Number = int32_t;
pub type C2RustUnnamed_31 = libc::c_uint;
pub const int_comp: C2RustUnnamed_31 = 7;
pub const int_race: C2RustUnnamed_31 = 6;
pub const int_spec: C2RustUnnamed_31 = 5;
pub const int_ctf: C2RustUnnamed_31 = 4;
pub const int_teammatch: C2RustUnnamed_31 = 3;
pub const int_match: C2RustUnnamed_31 = 2;
pub const int_coop: C2RustUnnamed_31 = 1;
pub const int_none: C2RustUnnamed_31 = 0;
pub type C2RustUnnamed_32 = libc::c_int;
pub const TC_DEFAULT: C2RustUnnamed_32 = -122;
pub const TC_DASHMODE: C2RustUnnamed_32 = -123;
pub const TC_BLINK: C2RustUnnamed_32 = -124;
pub const TC_RAINBOW: C2RustUnnamed_32 = -125;
pub const TC_ALLWHITE: C2RustUnnamed_32 = -126;
pub const TC_METALSONIC: C2RustUnnamed_32 = -127;
pub const TC_BOSS: C2RustUnnamed_32 = -128;
pub type C2RustUnnamed_33 = libc::c_uint;
pub const JA_FIRENORMAL: C2RustUnnamed_33 = 8;
pub const JA_FIRE: C2RustUnnamed_33 = 7;
pub const JA_SPIN: C2RustUnnamed_33 = 6;
pub const JA_JUMP: C2RustUnnamed_33 = 5;
pub const JA_DIGITAL: C2RustUnnamed_33 = 5;
pub const JA_STRAFE: C2RustUnnamed_33 = 4;
pub const JA_LOOK: C2RustUnnamed_33 = 3;
pub const JA_MOVE: C2RustUnnamed_33 = 2;
pub const JA_TURN: C2RustUnnamed_33 = 1;
pub const JA_NONE: C2RustUnnamed_33 = 0;
pub type C2RustUnnamed_34 = libc::c_uint;
pub const NUM_GAMECONTROLS: C2RustUnnamed_34 = 43;
pub const GC_CUSTOM3: C2RustUnnamed_34 = 42;
pub const GC_CUSTOM2: C2RustUnnamed_34 = 41;
pub const GC_CUSTOM1: C2RustUnnamed_34 = 40;
pub const GC_VIEWPOINTPREV: C2RustUnnamed_34 = 39;
pub const GC_VIEWPOINTNEXT: C2RustUnnamed_34 = 38;
pub const GC_RECORDGIF: C2RustUnnamed_34 = 37;
pub const GC_SCREENSHOT: C2RustUnnamed_34 = 36;
pub const GC_SYSTEMMENU: C2RustUnnamed_34 = 35;
pub const GC_PAUSE: C2RustUnnamed_34 = 34;
pub const GC_CONSOLE: C2RustUnnamed_34 = 33;
pub const GC_JUMP: C2RustUnnamed_34 = 32;
pub const GC_SCORES: C2RustUnnamed_34 = 31;
pub const GC_TEAMKEY: C2RustUnnamed_34 = 30;
pub const GC_TALKKEY: C2RustUnnamed_34 = 29;
pub const GC_MOUSEAIMING: C2RustUnnamed_34 = 28;
pub const GC_CENTERVIEW: C2RustUnnamed_34 = 27;
pub const GC_LOOKDOWN: C2RustUnnamed_34 = 26;
pub const GC_LOOKUP: C2RustUnnamed_34 = 25;
pub const GC_CAMRESET: C2RustUnnamed_34 = 24;
pub const GC_CAMTOGGLE: C2RustUnnamed_34 = 23;
pub const GC_SPIN: C2RustUnnamed_34 = 22;
pub const GC_TOSSFLAG: C2RustUnnamed_34 = 21;
pub const GC_FIRENORMAL: C2RustUnnamed_34 = 20;
pub const GC_FIRE: C2RustUnnamed_34 = 19;
pub const GC_WEPSLOT10: C2RustUnnamed_34 = 18;
pub const GC_WEPSLOT9: C2RustUnnamed_34 = 17;
pub const GC_WEPSLOT8: C2RustUnnamed_34 = 16;
pub const GC_WEPSLOT7: C2RustUnnamed_34 = 15;
pub const GC_WEPSLOT6: C2RustUnnamed_34 = 14;
pub const GC_WEPSLOT5: C2RustUnnamed_34 = 13;
pub const GC_WEPSLOT4: C2RustUnnamed_34 = 12;
pub const GC_WEPSLOT3: C2RustUnnamed_34 = 11;
pub const GC_WEPSLOT2: C2RustUnnamed_34 = 10;
pub const GC_WEPSLOT1: C2RustUnnamed_34 = 9;
pub const GC_WEAPONPREV: C2RustUnnamed_34 = 8;
pub const GC_WEAPONNEXT: C2RustUnnamed_34 = 7;
pub const GC_TURNRIGHT: C2RustUnnamed_34 = 6;
pub const GC_TURNLEFT: C2RustUnnamed_34 = 5;
pub const GC_STRAFERIGHT: C2RustUnnamed_34 = 4;
pub const GC_STRAFELEFT: C2RustUnnamed_34 = 3;
pub const GC_BACKWARD: C2RustUnnamed_34 = 2;
pub const GC_FORWARD: C2RustUnnamed_34 = 1;
pub const GC_NULL: C2RustUnnamed_34 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flickytypes_s {
    pub name: *const libc::c_char,
    pub type_0: mobjtype_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct actionpointer_t {
    pub action: actionf_t,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct int_const_s {
    pub n: *const libc::c_char,
    pub v: lua_Number,
}
#[no_mangle]
pub static mut FREE_STATES: [*mut libc::c_char; 4096] = [0 as *const libc::c_char
    as *mut libc::c_char; 4096];
#[no_mangle]
pub static mut FREE_MOBJS: [*mut libc::c_char; 512] = [0 as *const libc::c_char
    as *mut libc::c_char; 512];
#[no_mangle]
pub static mut FREE_SKINCOLORS: [*mut libc::c_char; 1024] = [0 as *const libc::c_char
    as *mut libc::c_char; 1024];
#[no_mangle]
pub static mut used_spr: [uint8_t; 65] = [0; 65];
#[no_mangle]
pub static mut NIGHTSGRADE_LIST: [libc::c_char; 8] = [
    'F' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    '\0' as i32 as libc::c_char,
];
#[no_mangle]
pub static mut FLICKYTYPES: [flickytypes_s; 21] = [
    {
        let mut init = flickytypes_s {
            name: b"BLUEBIRD\0" as *const u8 as *const libc::c_char,
            type_0: MT_FLICKY_01,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"RABBIT\0" as *const u8 as *const libc::c_char,
            type_0: MT_FLICKY_02,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"CHICKEN\0" as *const u8 as *const libc::c_char,
            type_0: MT_FLICKY_03,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"SEAL\0" as *const u8 as *const libc::c_char,
            type_0: MT_FLICKY_04,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"PIG\0" as *const u8 as *const libc::c_char,
            type_0: MT_FLICKY_05,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"CHIPMUNK\0" as *const u8 as *const libc::c_char,
            type_0: MT_FLICKY_06,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"PENGUIN\0" as *const u8 as *const libc::c_char,
            type_0: MT_FLICKY_07,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"FISH\0" as *const u8 as *const libc::c_char,
            type_0: MT_FLICKY_08,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"RAM\0" as *const u8 as *const libc::c_char,
            type_0: MT_FLICKY_09,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"PUFFIN\0" as *const u8 as *const libc::c_char,
            type_0: MT_FLICKY_10,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"COW\0" as *const u8 as *const libc::c_char,
            type_0: MT_FLICKY_11,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"RAT\0" as *const u8 as *const libc::c_char,
            type_0: MT_FLICKY_12,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"BEAR\0" as *const u8 as *const libc::c_char,
            type_0: MT_FLICKY_13,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"DOVE\0" as *const u8 as *const libc::c_char,
            type_0: MT_FLICKY_14,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"CAT\0" as *const u8 as *const libc::c_char,
            type_0: MT_FLICKY_15,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"CANARY\0" as *const u8 as *const libc::c_char,
            type_0: MT_FLICKY_16,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"a\0" as *const u8 as *const libc::c_char,
            type_0: MT_NULL,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"SPIDER\0" as *const u8 as *const libc::c_char,
            type_0: MT_SECRETFLICKY_01,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"BAT\0" as *const u8 as *const libc::c_char,
            type_0: MT_SECRETFLICKY_02,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: b"SEED\0" as *const u8 as *const libc::c_char,
            type_0: MT_SEED,
        };
        init
    },
    {
        let mut init = flickytypes_s {
            name: 0 as *const libc::c_char,
            type_0: MT_NULL,
        };
        init
    },
];
#[no_mangle]
pub static mut actionpointers: [actionpointer_t; 269] = unsafe {
    [
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Explode),
                    ),
                },
                name: b"A_EXPLODE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Pain),
                    ),
                },
                name: b"A_PAIN\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Fall),
                    ),
                },
                name: b"A_FALL\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_MonitorPop),
                    ),
                },
                name: b"A_MONITORPOP\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_GoldMonitorPop),
                    ),
                },
                name: b"A_GOLDMONITORPOP\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_GoldMonitorRestore),
                    ),
                },
                name: b"A_GOLDMONITORRESTORE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_GoldMonitorSparkle),
                    ),
                },
                name: b"A_GOLDMONITORSPARKLE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Look),
                    ),
                },
                name: b"A_LOOK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Chase),
                    ),
                },
                name: b"A_CHASE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FaceStabChase),
                    ),
                },
                name: b"A_FACESTABCHASE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FaceStabRev),
                    ),
                },
                name: b"A_FACESTABREV\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FaceStabHurl),
                    ),
                },
                name: b"A_FACESTABHURL\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FaceStabMiss),
                    ),
                },
                name: b"A_FACESTABMISS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_StatueBurst),
                    ),
                },
                name: b"A_STATUEBURST\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FaceTarget),
                    ),
                },
                name: b"A_FACETARGET\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FaceTracer),
                    ),
                },
                name: b"A_FACETRACER\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Scream),
                    ),
                },
                name: b"A_SCREAM\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_BossDeath),
                    ),
                },
                name: b"A_BOSSDEATH\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SetShadowScale),
                    ),
                },
                name: b"A_SETSHADOWSCALE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ShadowScream),
                    ),
                },
                name: b"A_SHADOWSCREAM\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CustomPower),
                    ),
                },
                name: b"A_CUSTOMPOWER\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_GiveWeapon),
                    ),
                },
                name: b"A_GIVEWEAPON\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_RingBox),
                    ),
                },
                name: b"A_RINGBOX\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Invincibility),
                    ),
                },
                name: b"A_INVINCIBILITY\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SuperSneakers),
                    ),
                },
                name: b"A_SUPERSNEAKERS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_BunnyHop),
                    ),
                },
                name: b"A_BUNNYHOP\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_BubbleSpawn),
                    ),
                },
                name: b"A_BUBBLESPAWN\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FanBubbleSpawn),
                    ),
                },
                name: b"A_FANBUBBLESPAWN\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_BubbleRise),
                    ),
                },
                name: b"A_BUBBLERISE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_BubbleCheck),
                    ),
                },
                name: b"A_BUBBLECHECK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_AwardScore),
                    ),
                },
                name: b"A_AWARDSCORE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ExtraLife),
                    ),
                },
                name: b"A_EXTRALIFE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_GiveShield),
                    ),
                },
                name: b"A_GIVESHIELD\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_GravityBox),
                    ),
                },
                name: b"A_GRAVITYBOX\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ScoreRise),
                    ),
                },
                name: b"A_SCORERISE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_AttractChase),
                    ),
                },
                name: b"A_ATTRACTCHASE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_DropMine),
                    ),
                },
                name: b"A_DROPMINE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FishJump),
                    ),
                },
                name: b"A_FISHJUMP\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ThrownRing),
                    ),
                },
                name: b"A_THROWNRING\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SetSolidSteam),
                    ),
                },
                name: b"A_SETSOLIDSTEAM\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_UnsetSolidSteam),
                    ),
                },
                name: b"A_UNSETSOLIDSTEAM\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SignSpin),
                    ),
                },
                name: b"A_SIGNSPIN\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SignPlayer),
                    ),
                },
                name: b"A_SIGNPLAYER\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_OverlayThink),
                    ),
                },
                name: b"A_OVERLAYTHINK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_JetChase),
                    ),
                },
                name: b"A_JETCHASE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_JetbThink),
                    ),
                },
                name: b"A_JETBTHINK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_JetgThink),
                    ),
                },
                name: b"A_JETGTHINK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_JetgShoot),
                    ),
                },
                name: b"A_JETGSHOOT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ShootBullet),
                    ),
                },
                name: b"A_SHOOTBULLET\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_MinusDigging),
                    ),
                },
                name: b"A_MINUSDIGGING\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_MinusPopup),
                    ),
                },
                name: b"A_MINUSPOPUP\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_MinusCheck),
                    ),
                },
                name: b"A_MINUSCHECK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ChickenCheck),
                    ),
                },
                name: b"A_CHICKENCHECK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_MouseThink),
                    ),
                },
                name: b"A_MOUSETHINK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_DetonChase),
                    ),
                },
                name: b"A_DETONCHASE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CapeChase),
                    ),
                },
                name: b"A_CAPECHASE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_RotateSpikeBall),
                    ),
                },
                name: b"A_ROTATESPIKEBALL\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SlingAppear),
                    ),
                },
                name: b"A_SLINGAPPEAR\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_UnidusBall),
                    ),
                },
                name: b"A_UNIDUSBALL\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_RockSpawn),
                    ),
                },
                name: b"A_ROCKSPAWN\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SetFuse),
                    ),
                },
                name: b"A_SETFUSE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CrawlaCommanderThink),
                    ),
                },
                name: b"A_CRAWLACOMMANDERTHINK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SmokeTrailer),
                    ),
                },
                name: b"A_SMOKETRAILER\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_RingExplode),
                    ),
                },
                name: b"A_RINGEXPLODE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_OldRingExplode),
                    ),
                },
                name: b"A_OLDRINGEXPLODE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_MixUp),
                    ),
                },
                name: b"A_MIXUP\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_RecyclePowers),
                    ),
                },
                name: b"A_RECYCLEPOWERS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss1Chase),
                    ),
                },
                name: b"A_BOSS1CHASE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FocusTarget),
                    ),
                },
                name: b"A_FOCUSTARGET\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss2Chase),
                    ),
                },
                name: b"A_BOSS2CHASE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss2Pogo),
                    ),
                },
                name: b"A_BOSS2POGO\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_BossZoom),
                    ),
                },
                name: b"A_BOSSZOOM\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_BossScream),
                    ),
                },
                name: b"A_BOSSSCREAM\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss2TakeDamage),
                    ),
                },
                name: b"A_BOSS2TAKEDAMAGE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss7Chase),
                    ),
                },
                name: b"A_BOSS7CHASE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_GoopSplat),
                    ),
                },
                name: b"A_GOOPSPLAT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss2PogoSFX),
                    ),
                },
                name: b"A_BOSS2POGOSFX\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss2PogoTarget),
                    ),
                },
                name: b"A_BOSS2POGOTARGET\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_BossJetFume),
                    ),
                },
                name: b"A_BOSSJETFUME\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_EggmanBox),
                    ),
                },
                name: b"A_EGGMANBOX\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_TurretFire),
                    ),
                },
                name: b"A_TURRETFIRE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SuperTurretFire),
                    ),
                },
                name: b"A_SUPERTURRETFIRE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_TurretStop),
                    ),
                },
                name: b"A_TURRETSTOP\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_JetJawRoam),
                    ),
                },
                name: b"A_JETJAWROAM\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_JetJawChomp),
                    ),
                },
                name: b"A_JETJAWCHOMP\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_PointyThink),
                    ),
                },
                name: b"A_POINTYTHINK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CheckBuddy),
                    ),
                },
                name: b"A_CHECKBUDDY\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_HoodFire),
                    ),
                },
                name: b"A_HOODFIRE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_HoodThink),
                    ),
                },
                name: b"A_HOODTHINK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_HoodFall),
                    ),
                },
                name: b"A_HOODFALL\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ArrowBonks),
                    ),
                },
                name: b"A_ARROWBONKS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SnailerThink),
                    ),
                },
                name: b"A_SNAILERTHINK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SharpChase),
                    ),
                },
                name: b"A_SHARPCHASE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SharpSpin),
                    ),
                },
                name: b"A_SHARPSPIN\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SharpDecel),
                    ),
                },
                name: b"A_SHARPDECEL\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CrushstaceanWalk),
                    ),
                },
                name: b"A_CRUSHSTACEANWALK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CrushstaceanPunch),
                    ),
                },
                name: b"A_CRUSHSTACEANPUNCH\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CrushclawAim),
                    ),
                },
                name: b"A_CRUSHCLAWAIM\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CrushclawLaunch),
                    ),
                },
                name: b"A_CRUSHCLAWLAUNCH\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_VultureVtol),
                    ),
                },
                name: b"A_VULTUREVTOL\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_VultureCheck),
                    ),
                },
                name: b"A_VULTURECHECK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_VultureHover),
                    ),
                },
                name: b"A_VULTUREHOVER\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_VultureBlast),
                    ),
                },
                name: b"A_VULTUREBLAST\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_VultureFly),
                    ),
                },
                name: b"A_VULTUREFLY\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SkimChase),
                    ),
                },
                name: b"A_SKIMCHASE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_1upThinker),
                    ),
                },
                name: b"A_1UPTHINKER\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SkullAttack),
                    ),
                },
                name: b"A_SKULLATTACK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_LobShot),
                    ),
                },
                name: b"A_LOBSHOT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FireShot),
                    ),
                },
                name: b"A_FIRESHOT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SuperFireShot),
                    ),
                },
                name: b"A_SUPERFIRESHOT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_BossFireShot),
                    ),
                },
                name: b"A_BOSSFIRESHOT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss7FireMissiles),
                    ),
                },
                name: b"A_BOSS7FIREMISSILES\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss1Laser),
                    ),
                },
                name: b"A_BOSS1LASER\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss4Reverse),
                    ),
                },
                name: b"A_BOSS4REVERSE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss4SpeedUp),
                    ),
                },
                name: b"A_BOSS4SPEEDUP\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss4Raise),
                    ),
                },
                name: b"A_BOSS4RAISE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SparkFollow),
                    ),
                },
                name: b"A_SPARKFOLLOW\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_BuzzFly),
                    ),
                },
                name: b"A_BUZZFLY\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_GuardChase),
                    ),
                },
                name: b"A_GUARDCHASE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_EggShield),
                    ),
                },
                name: b"A_EGGSHIELD\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SetReactionTime),
                    ),
                },
                name: b"A_SETREACTIONTIME\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss1Spikeballs),
                    ),
                },
                name: b"A_BOSS1SPIKEBALLS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss3TakeDamage),
                    ),
                },
                name: b"A_BOSS3TAKEDAMAGE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss3Path),
                    ),
                },
                name: b"A_BOSS3PATH\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss3ShockThink),
                    ),
                },
                name: b"A_BOSS3SHOCKTHINK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Shockwave),
                    ),
                },
                name: b"A_SHOCKWAVE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_LinedefExecute),
                    ),
                },
                name: b"A_LINEDEFEXECUTE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_LinedefExecuteFromArg),
                    ),
                },
                name: b"A_LINEDEFEXECUTEFROMARG\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_PlaySeeSound),
                    ),
                },
                name: b"A_PLAYSEESOUND\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_PlayAttackSound),
                    ),
                },
                name: b"A_PLAYATTACKSOUND\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_PlayActiveSound),
                    ),
                },
                name: b"A_PLAYACTIVESOUND\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SpawnObjectAbsolute),
                    ),
                },
                name: b"A_SPAWNOBJECTABSOLUTE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SpawnObjectRelative),
                    ),
                },
                name: b"A_SPAWNOBJECTRELATIVE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ChangeAngleRelative),
                    ),
                },
                name: b"A_CHANGEANGLERELATIVE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ChangeAngleAbsolute),
                    ),
                },
                name: b"A_CHANGEANGLEABSOLUTE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_RollAngle),
                    ),
                },
                name: b"A_ROLLANGLE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ChangeRollAngleRelative),
                    ),
                },
                name: b"A_CHANGEROLLANGLERELATIVE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ChangeRollAngleAbsolute),
                    ),
                },
                name: b"A_CHANGEROLLANGLEABSOLUTE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_PlaySound),
                    ),
                },
                name: b"A_PLAYSOUND\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FindTarget),
                    ),
                },
                name: b"A_FINDTARGET\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FindTracer),
                    ),
                },
                name: b"A_FINDTRACER\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SetTics),
                    ),
                },
                name: b"A_SETTICS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SetRandomTics),
                    ),
                },
                name: b"A_SETRANDOMTICS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ChangeColorRelative),
                    ),
                },
                name: b"A_CHANGECOLORRELATIVE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ChangeColorAbsolute),
                    ),
                },
                name: b"A_CHANGECOLORABSOLUTE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Dye),
                    ),
                },
                name: b"A_DYE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_MoveRelative),
                    ),
                },
                name: b"A_MOVERELATIVE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_MoveAbsolute),
                    ),
                },
                name: b"A_MOVEABSOLUTE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Thrust),
                    ),
                },
                name: b"A_THRUST\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ZThrust),
                    ),
                },
                name: b"A_ZTHRUST\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SetTargetsTarget),
                    ),
                },
                name: b"A_SETTARGETSTARGET\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SetObjectFlags),
                    ),
                },
                name: b"A_SETOBJECTFLAGS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SetObjectFlags2),
                    ),
                },
                name: b"A_SETOBJECTFLAGS2\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_RandomState),
                    ),
                },
                name: b"A_RANDOMSTATE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_RandomStateRange),
                    ),
                },
                name: b"A_RANDOMSTATERANGE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_StateRangeByAngle),
                    ),
                },
                name: b"A_STATERANGEBYANGLE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_StateRangeByParameter),
                    ),
                },
                name: b"A_STATERANGEBYPARAMETER\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_DualAction),
                    ),
                },
                name: b"A_DUALACTION\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_RemoteAction),
                    ),
                },
                name: b"A_REMOTEACTION\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ToggleFlameJet),
                    ),
                },
                name: b"A_TOGGLEFLAMEJET\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_OrbitNights),
                    ),
                },
                name: b"A_ORBITNIGHTS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_GhostMe),
                    ),
                },
                name: b"A_GHOSTME\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SetObjectState),
                    ),
                },
                name: b"A_SETOBJECTSTATE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SetObjectTypeState),
                    ),
                },
                name: b"A_SETOBJECTTYPESTATE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_KnockBack),
                    ),
                },
                name: b"A_KNOCKBACK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_PushAway),
                    ),
                },
                name: b"A_PUSHAWAY\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_RingDrain),
                    ),
                },
                name: b"A_RINGDRAIN\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SplitShot),
                    ),
                },
                name: b"A_SPLITSHOT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_MissileSplit),
                    ),
                },
                name: b"A_MISSILESPLIT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_MultiShot),
                    ),
                },
                name: b"A_MULTISHOT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_InstaLoop),
                    ),
                },
                name: b"A_INSTALOOP\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Custom3DRotate),
                    ),
                },
                name: b"A_CUSTOM3DROTATE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SearchForPlayers),
                    ),
                },
                name: b"A_SEARCHFORPLAYERS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CheckRandom),
                    ),
                },
                name: b"A_CHECKRANDOM\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CheckTargetRings),
                    ),
                },
                name: b"A_CHECKTARGETRINGS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CheckRings),
                    ),
                },
                name: b"A_CHECKRINGS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CheckTotalRings),
                    ),
                },
                name: b"A_CHECKTOTALRINGS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CheckHealth),
                    ),
                },
                name: b"A_CHECKHEALTH\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CheckRange),
                    ),
                },
                name: b"A_CHECKRANGE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CheckHeight),
                    ),
                },
                name: b"A_CHECKHEIGHT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CheckTrueRange),
                    ),
                },
                name: b"A_CHECKTRUERANGE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CheckThingCount),
                    ),
                },
                name: b"A_CHECKTHINGCOUNT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CheckAmbush),
                    ),
                },
                name: b"A_CHECKAMBUSH\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CheckCustomValue),
                    ),
                },
                name: b"A_CHECKCUSTOMVALUE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CheckCusValMemo),
                    ),
                },
                name: b"A_CHECKCUSVALMEMO\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SetCustomValue),
                    ),
                },
                name: b"A_SETCUSTOMVALUE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_UseCusValMemo),
                    ),
                },
                name: b"A_USECUSVALMEMO\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_RelayCustomValue),
                    ),
                },
                name: b"A_RELAYCUSTOMVALUE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CusValAction),
                    ),
                },
                name: b"A_CUSVALACTION\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ForceStop),
                    ),
                },
                name: b"A_FORCESTOP\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ForceWin),
                    ),
                },
                name: b"A_FORCEWIN\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SpikeRetract),
                    ),
                },
                name: b"A_SPIKERETRACT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_InfoState),
                    ),
                },
                name: b"A_INFOSTATE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Repeat),
                    ),
                },
                name: b"A_REPEAT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SetScale),
                    ),
                },
                name: b"A_SETSCALE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_RemoteDamage),
                    ),
                },
                name: b"A_REMOTEDAMAGE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_HomingChase),
                    ),
                },
                name: b"A_HOMINGCHASE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_TrapShot),
                    ),
                },
                name: b"A_TRAPSHOT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_VileTarget),
                    ),
                },
                name: b"A_VILETARGET\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_VileAttack),
                    ),
                },
                name: b"A_VILEATTACK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_VileFire),
                    ),
                },
                name: b"A_VILEFIRE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_BrakChase),
                    ),
                },
                name: b"A_BRAKCHASE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_BrakFireShot),
                    ),
                },
                name: b"A_BRAKFIRESHOT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_BrakLobShot),
                    ),
                },
                name: b"A_BRAKLOBSHOT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_NapalmScatter),
                    ),
                },
                name: b"A_NAPALMSCATTER\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SpawnFreshCopy),
                    ),
                },
                name: b"A_SPAWNFRESHCOPY\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FlickySpawn),
                    ),
                },
                name: b"A_FLICKYSPAWN\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FlickyCenter),
                    ),
                },
                name: b"A_FLICKYCENTER\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FlickyAim),
                    ),
                },
                name: b"A_FLICKYAIM\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FlickyFly),
                    ),
                },
                name: b"A_FLICKYFLY\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FlickySoar),
                    ),
                },
                name: b"A_FLICKYSOAR\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FlickyCoast),
                    ),
                },
                name: b"A_FLICKYCOAST\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FlickyHop),
                    ),
                },
                name: b"A_FLICKYHOP\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FlickyFlounder),
                    ),
                },
                name: b"A_FLICKYFLOUNDER\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FlickyCheck),
                    ),
                },
                name: b"A_FLICKYCHECK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FlickyHeightCheck),
                    ),
                },
                name: b"A_FLICKYHEIGHTCHECK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FlickyFlutter),
                    ),
                },
                name: b"A_FLICKYFLUTTER\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FlameParticle),
                    ),
                },
                name: b"A_FLAMEPARTICLE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FadeOverlay),
                    ),
                },
                name: b"A_FADEOVERLAY\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss5Jump),
                    ),
                },
                name: b"A_BOSS5JUMP\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_LightBeamReset),
                    ),
                },
                name: b"A_LIGHTBEAMRESET\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_MineExplode),
                    ),
                },
                name: b"A_MINEEXPLODE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_MineRange),
                    ),
                },
                name: b"A_MINERANGE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ConnectToGround),
                    ),
                },
                name: b"A_CONNECTTOGROUND\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SpawnParticleRelative),
                    ),
                },
                name: b"A_SPAWNPARTICLERELATIVE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_MultiShotDist),
                    ),
                },
                name: b"A_MULTISHOTDIST\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_WhoCaresIfYourSonIsABee),
                    ),
                },
                name: b"A_WHOCARESIFYOURSONISABEE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ParentTriesToSleep),
                    ),
                },
                name: b"A_PARENTTRIESTOSLEEP\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CryingToMomma),
                    ),
                },
                name: b"A_CRYINGTOMOMMA\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CheckFlags2),
                    ),
                },
                name: b"A_CHECKFLAGS2\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss5FindWaypoint),
                    ),
                },
                name: b"A_BOSS5FINDWAYPOINT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_DoNPCSkid),
                    ),
                },
                name: b"A_DONPCSKID\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_DoNPCPain),
                    ),
                },
                name: b"A_DONPCPAIN\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_PrepareRepeat),
                    ),
                },
                name: b"A_PREPAREREPEAT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss5ExtraRepeat),
                    ),
                },
                name: b"A_BOSS5EXTRAREPEAT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss5Calm),
                    ),
                },
                name: b"A_BOSS5CALM\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss5CheckOnGround),
                    ),
                },
                name: b"A_BOSS5CHECKONGROUND\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss5CheckFalling),
                    ),
                },
                name: b"A_BOSS5CHECKFALLING\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss5PinchShot),
                    ),
                },
                name: b"A_BOSS5PINCHSHOT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss5MakeItRain),
                    ),
                },
                name: b"A_BOSS5MAKEITRAIN\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss5MakeJunk),
                    ),
                },
                name: b"A_BOSS5MAKEJUNK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_LookForBetter),
                    ),
                },
                name: b"A_LOOKFORBETTER\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_Boss5BombExplode),
                    ),
                },
                name: b"A_BOSS5BOMBEXPLODE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_DustDevilThink),
                    ),
                },
                name: b"A_DUSTDEVILTHINK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_TNTExplode),
                    ),
                },
                name: b"A_TNTEXPLODE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_DebrisRandom),
                    ),
                },
                name: b"A_DEBRISRANDOM\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_TrainCameo),
                    ),
                },
                name: b"A_TRAINCAMEO\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_TrainCameo2),
                    ),
                },
                name: b"A_TRAINCAMEO2\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_CanarivoreGas),
                    ),
                },
                name: b"A_CANARIVOREGAS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_KillSegments),
                    ),
                },
                name: b"A_KILLSEGMENTS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SnapperSpawn),
                    ),
                },
                name: b"A_SNAPPERSPAWN\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SnapperThinker),
                    ),
                },
                name: b"A_SNAPPERTHINKER\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SaloonDoorSpawn),
                    ),
                },
                name: b"A_SALOONDOORSPAWN\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_MinecartSparkThink),
                    ),
                },
                name: b"A_MINECARTSPARKTHINK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ModuloToState),
                    ),
                },
                name: b"A_MODULOTOSTATE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_LavafallRocks),
                    ),
                },
                name: b"A_LAVAFALLROCKS\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_LavafallLava),
                    ),
                },
                name: b"A_LAVAFALLLAVA\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FallingLavaCheck),
                    ),
                },
                name: b"A_FALLINGLAVACHECK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_FireShrink),
                    ),
                },
                name: b"A_FIRESHRINK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_SpawnPterabytes),
                    ),
                },
                name: b"A_SPAWNPTERABYTES\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_PterabyteHover),
                    ),
                },
                name: b"A_PTERABYTEHOVER\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_RolloutSpawn),
                    ),
                },
                name: b"A_ROLLOUTSPAWN\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_RolloutRock),
                    ),
                },
                name: b"A_ROLLOUTROCK\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_DragonbomberSpawn),
                    ),
                },
                name: b"A_DRAGONBOMBERSPAWN\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_DragonWing),
                    ),
                },
                name: b"A_DRAGONWING\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_DragonSegment),
                    ),
                },
                name: b"A_DRAGONSEGMENT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t {
                    acv: Some(
                        ::core::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(A_ChangeHeight),
                    ),
                },
                name: b"A_CHANGEHEIGHT\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t { acv: None },
                name: b"NONE\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = actionpointer_t {
                action: actionf_t { acv: None },
                name: 0 as *const libc::c_char,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut STATE_LIST: [*const libc::c_char; 2639] = [
    b"S_NULL\0" as *const u8 as *const libc::c_char,
    b"S_UNKNOWN\0" as *const u8 as *const libc::c_char,
    b"S_INVISIBLE\0" as *const u8 as *const libc::c_char,
    b"S_SPAWNSTATE\0" as *const u8 as *const libc::c_char,
    b"S_SEESTATE\0" as *const u8 as *const libc::c_char,
    b"S_MELEESTATE\0" as *const u8 as *const libc::c_char,
    b"S_MISSILESTATE\0" as *const u8 as *const libc::c_char,
    b"S_DEATHSTATE\0" as *const u8 as *const libc::c_char,
    b"S_XDEATHSTATE\0" as *const u8 as *const libc::c_char,
    b"S_RAISESTATE\0" as *const u8 as *const libc::c_char,
    b"S_THOK\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_STND\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_WAIT\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_WALK\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_SKID\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_RUN\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_DASH\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_PAIN\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_STUN\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_DEAD\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_DRWN\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_ROLL\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_GASP\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_JUMP\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_SPRING\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_FALL\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_EDGE\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_RIDE\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_SPINDASH\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_FLY\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_SWIM\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_FLY_TIRED\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_GLIDE\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_GLIDE_LANDING\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_CLING\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_CLIMB\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_FLOAT\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_FLOAT_RUN\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_BOUNCE\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_BOUNCE_LANDING\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_FIRE\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_FIRE_FINISH\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_TWINSPIN\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_MELEE\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_MELEE_FINISH\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_MELEE_LANDING\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_SUPER_TRANS1\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_SUPER_TRANS2\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_SUPER_TRANS3\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_SUPER_TRANS4\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_SUPER_TRANS5\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_SUPER_TRANS6\0" as *const u8 as *const libc::c_char,
    b"S_OBJPLACE_DUMMY\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_BOX1\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_BOX2\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_ICON3\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_SIGN\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_NIGHTS_TRANS1\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_NIGHTS_TRANS2\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_NIGHTS_TRANS3\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_NIGHTS_TRANS4\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_NIGHTS_TRANS5\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_NIGHTS_TRANS6\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_NIGHTS_STAND\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_NIGHTS_FLOAT\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_NIGHTS_FLY\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_NIGHTS_DRILL\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_NIGHTS_STUN\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_NIGHTS_PULL\0" as *const u8 as *const libc::c_char,
    b"S_PLAY_NIGHTS_ATTACK\0" as *const u8 as *const libc::c_char,
    b"S_TAILSOVERLAY_STAND\0" as *const u8 as *const libc::c_char,
    b"S_TAILSOVERLAY_0DEGREES\0" as *const u8 as *const libc::c_char,
    b"S_TAILSOVERLAY_PLUS30DEGREES\0" as *const u8 as *const libc::c_char,
    b"S_TAILSOVERLAY_PLUS60DEGREES\0" as *const u8 as *const libc::c_char,
    b"S_TAILSOVERLAY_MINUS30DEGREES\0" as *const u8 as *const libc::c_char,
    b"S_TAILSOVERLAY_MINUS60DEGREES\0" as *const u8 as *const libc::c_char,
    b"S_TAILSOVERLAY_RUN\0" as *const u8 as *const libc::c_char,
    b"S_TAILSOVERLAY_FLY\0" as *const u8 as *const libc::c_char,
    b"S_TAILSOVERLAY_TIRE\0" as *const u8 as *const libc::c_char,
    b"S_TAILSOVERLAY_PAIN\0" as *const u8 as *const libc::c_char,
    b"S_TAILSOVERLAY_GASP\0" as *const u8 as *const libc::c_char,
    b"S_TAILSOVERLAY_EDGE\0" as *const u8 as *const libc::c_char,
    b"S_TAILSOVERLAY_DASH\0" as *const u8 as *const libc::c_char,
    b"S_JETFUMEFLASH\0" as *const u8 as *const libc::c_char,
    b"S_POSS_STND\0" as *const u8 as *const libc::c_char,
    b"S_POSS_RUN1\0" as *const u8 as *const libc::c_char,
    b"S_POSS_RUN2\0" as *const u8 as *const libc::c_char,
    b"S_POSS_RUN3\0" as *const u8 as *const libc::c_char,
    b"S_POSS_RUN4\0" as *const u8 as *const libc::c_char,
    b"S_POSS_RUN5\0" as *const u8 as *const libc::c_char,
    b"S_POSS_RUN6\0" as *const u8 as *const libc::c_char,
    b"S_SPOS_STND\0" as *const u8 as *const libc::c_char,
    b"S_SPOS_RUN1\0" as *const u8 as *const libc::c_char,
    b"S_SPOS_RUN2\0" as *const u8 as *const libc::c_char,
    b"S_SPOS_RUN3\0" as *const u8 as *const libc::c_char,
    b"S_SPOS_RUN4\0" as *const u8 as *const libc::c_char,
    b"S_SPOS_RUN5\0" as *const u8 as *const libc::c_char,
    b"S_SPOS_RUN6\0" as *const u8 as *const libc::c_char,
    b"S_FISH1\0" as *const u8 as *const libc::c_char,
    b"S_FISH2\0" as *const u8 as *const libc::c_char,
    b"S_FISH3\0" as *const u8 as *const libc::c_char,
    b"S_FISH4\0" as *const u8 as *const libc::c_char,
    b"S_BUZZLOOK1\0" as *const u8 as *const libc::c_char,
    b"S_BUZZLOOK2\0" as *const u8 as *const libc::c_char,
    b"S_BUZZFLY1\0" as *const u8 as *const libc::c_char,
    b"S_BUZZFLY2\0" as *const u8 as *const libc::c_char,
    b"S_RBUZZLOOK1\0" as *const u8 as *const libc::c_char,
    b"S_RBUZZLOOK2\0" as *const u8 as *const libc::c_char,
    b"S_RBUZZFLY1\0" as *const u8 as *const libc::c_char,
    b"S_RBUZZFLY2\0" as *const u8 as *const libc::c_char,
    b"S_JETBLOOK1\0" as *const u8 as *const libc::c_char,
    b"S_JETBLOOK2\0" as *const u8 as *const libc::c_char,
    b"S_JETBZOOM1\0" as *const u8 as *const libc::c_char,
    b"S_JETBZOOM2\0" as *const u8 as *const libc::c_char,
    b"S_JETGLOOK1\0" as *const u8 as *const libc::c_char,
    b"S_JETGLOOK2\0" as *const u8 as *const libc::c_char,
    b"S_JETGZOOM1\0" as *const u8 as *const libc::c_char,
    b"S_JETGZOOM2\0" as *const u8 as *const libc::c_char,
    b"S_JETGSHOOT1\0" as *const u8 as *const libc::c_char,
    b"S_JETGSHOOT2\0" as *const u8 as *const libc::c_char,
    b"S_CCOMMAND1\0" as *const u8 as *const libc::c_char,
    b"S_CCOMMAND2\0" as *const u8 as *const libc::c_char,
    b"S_CCOMMAND3\0" as *const u8 as *const libc::c_char,
    b"S_CCOMMAND4\0" as *const u8 as *const libc::c_char,
    b"S_DETON1\0" as *const u8 as *const libc::c_char,
    b"S_DETON2\0" as *const u8 as *const libc::c_char,
    b"S_DETON3\0" as *const u8 as *const libc::c_char,
    b"S_DETON4\0" as *const u8 as *const libc::c_char,
    b"S_DETON5\0" as *const u8 as *const libc::c_char,
    b"S_DETON6\0" as *const u8 as *const libc::c_char,
    b"S_DETON7\0" as *const u8 as *const libc::c_char,
    b"S_DETON8\0" as *const u8 as *const libc::c_char,
    b"S_DETON9\0" as *const u8 as *const libc::c_char,
    b"S_DETON10\0" as *const u8 as *const libc::c_char,
    b"S_DETON11\0" as *const u8 as *const libc::c_char,
    b"S_DETON12\0" as *const u8 as *const libc::c_char,
    b"S_DETON13\0" as *const u8 as *const libc::c_char,
    b"S_DETON14\0" as *const u8 as *const libc::c_char,
    b"S_DETON15\0" as *const u8 as *const libc::c_char,
    b"S_SKIM1\0" as *const u8 as *const libc::c_char,
    b"S_SKIM2\0" as *const u8 as *const libc::c_char,
    b"S_SKIM3\0" as *const u8 as *const libc::c_char,
    b"S_SKIM4\0" as *const u8 as *const libc::c_char,
    b"S_TURRET\0" as *const u8 as *const libc::c_char,
    b"S_TURRETFIRE\0" as *const u8 as *const libc::c_char,
    b"S_TURRETSHOCK1\0" as *const u8 as *const libc::c_char,
    b"S_TURRETSHOCK2\0" as *const u8 as *const libc::c_char,
    b"S_TURRETSHOCK3\0" as *const u8 as *const libc::c_char,
    b"S_TURRETSHOCK4\0" as *const u8 as *const libc::c_char,
    b"S_TURRETSHOCK5\0" as *const u8 as *const libc::c_char,
    b"S_TURRETSHOCK6\0" as *const u8 as *const libc::c_char,
    b"S_TURRETSHOCK7\0" as *const u8 as *const libc::c_char,
    b"S_TURRETSHOCK8\0" as *const u8 as *const libc::c_char,
    b"S_TURRETSHOCK9\0" as *const u8 as *const libc::c_char,
    b"S_TURRETLOOK\0" as *const u8 as *const libc::c_char,
    b"S_TURRETSEE\0" as *const u8 as *const libc::c_char,
    b"S_TURRETPOPUP1\0" as *const u8 as *const libc::c_char,
    b"S_TURRETPOPUP2\0" as *const u8 as *const libc::c_char,
    b"S_TURRETPOPUP3\0" as *const u8 as *const libc::c_char,
    b"S_TURRETPOPUP4\0" as *const u8 as *const libc::c_char,
    b"S_TURRETPOPUP5\0" as *const u8 as *const libc::c_char,
    b"S_TURRETPOPUP6\0" as *const u8 as *const libc::c_char,
    b"S_TURRETPOPUP7\0" as *const u8 as *const libc::c_char,
    b"S_TURRETPOPUP8\0" as *const u8 as *const libc::c_char,
    b"S_TURRETSHOOT\0" as *const u8 as *const libc::c_char,
    b"S_TURRETPOPDOWN1\0" as *const u8 as *const libc::c_char,
    b"S_TURRETPOPDOWN2\0" as *const u8 as *const libc::c_char,
    b"S_TURRETPOPDOWN3\0" as *const u8 as *const libc::c_char,
    b"S_TURRETPOPDOWN4\0" as *const u8 as *const libc::c_char,
    b"S_TURRETPOPDOWN5\0" as *const u8 as *const libc::c_char,
    b"S_TURRETPOPDOWN6\0" as *const u8 as *const libc::c_char,
    b"S_TURRETPOPDOWN7\0" as *const u8 as *const libc::c_char,
    b"S_TURRETPOPDOWN8\0" as *const u8 as *const libc::c_char,
    b"S_SPINCUSHION_LOOK\0" as *const u8 as *const libc::c_char,
    b"S_SPINCUSHION_CHASE1\0" as *const u8 as *const libc::c_char,
    b"S_SPINCUSHION_CHASE2\0" as *const u8 as *const libc::c_char,
    b"S_SPINCUSHION_CHASE3\0" as *const u8 as *const libc::c_char,
    b"S_SPINCUSHION_CHASE4\0" as *const u8 as *const libc::c_char,
    b"S_SPINCUSHION_AIM1\0" as *const u8 as *const libc::c_char,
    b"S_SPINCUSHION_AIM2\0" as *const u8 as *const libc::c_char,
    b"S_SPINCUSHION_AIM3\0" as *const u8 as *const libc::c_char,
    b"S_SPINCUSHION_AIM4\0" as *const u8 as *const libc::c_char,
    b"S_SPINCUSHION_AIM5\0" as *const u8 as *const libc::c_char,
    b"S_SPINCUSHION_SPIN1\0" as *const u8 as *const libc::c_char,
    b"S_SPINCUSHION_SPIN2\0" as *const u8 as *const libc::c_char,
    b"S_SPINCUSHION_SPIN3\0" as *const u8 as *const libc::c_char,
    b"S_SPINCUSHION_SPIN4\0" as *const u8 as *const libc::c_char,
    b"S_SPINCUSHION_STOP1\0" as *const u8 as *const libc::c_char,
    b"S_SPINCUSHION_STOP2\0" as *const u8 as *const libc::c_char,
    b"S_SPINCUSHION_STOP3\0" as *const u8 as *const libc::c_char,
    b"S_SPINCUSHION_STOP4\0" as *const u8 as *const libc::c_char,
    b"S_CRUSHSTACEAN_ROAM1\0" as *const u8 as *const libc::c_char,
    b"S_CRUSHSTACEAN_ROAM2\0" as *const u8 as *const libc::c_char,
    b"S_CRUSHSTACEAN_ROAM3\0" as *const u8 as *const libc::c_char,
    b"S_CRUSHSTACEAN_ROAM4\0" as *const u8 as *const libc::c_char,
    b"S_CRUSHSTACEAN_ROAMPAUSE\0" as *const u8 as *const libc::c_char,
    b"S_CRUSHSTACEAN_PUNCH1\0" as *const u8 as *const libc::c_char,
    b"S_CRUSHSTACEAN_PUNCH2\0" as *const u8 as *const libc::c_char,
    b"S_CRUSHCLAW_AIM\0" as *const u8 as *const libc::c_char,
    b"S_CRUSHCLAW_OUT\0" as *const u8 as *const libc::c_char,
    b"S_CRUSHCLAW_STAY\0" as *const u8 as *const libc::c_char,
    b"S_CRUSHCLAW_IN\0" as *const u8 as *const libc::c_char,
    b"S_CRUSHCLAW_WAIT\0" as *const u8 as *const libc::c_char,
    b"S_CRUSHCHAIN\0" as *const u8 as *const libc::c_char,
    b"S_BANPYURA_ROAM1\0" as *const u8 as *const libc::c_char,
    b"S_BANPYURA_ROAM2\0" as *const u8 as *const libc::c_char,
    b"S_BANPYURA_ROAM3\0" as *const u8 as *const libc::c_char,
    b"S_BANPYURA_ROAM4\0" as *const u8 as *const libc::c_char,
    b"S_BANPYURA_ROAMPAUSE\0" as *const u8 as *const libc::c_char,
    b"S_CDIAG1\0" as *const u8 as *const libc::c_char,
    b"S_CDIAG2\0" as *const u8 as *const libc::c_char,
    b"S_CDIAG3\0" as *const u8 as *const libc::c_char,
    b"S_CDIAG4\0" as *const u8 as *const libc::c_char,
    b"S_CDIAG5\0" as *const u8 as *const libc::c_char,
    b"S_CDIAG6\0" as *const u8 as *const libc::c_char,
    b"S_CDIAG7\0" as *const u8 as *const libc::c_char,
    b"S_CDIAG8\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_ROAM1\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_ROAM2\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_ROAM3\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_ROAM4\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_ROAM5\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_ROAM6\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_ROAM7\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_ROAM8\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_CHOMP1\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_CHOMP2\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_CHOMP3\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_CHOMP4\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_CHOMP5\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_CHOMP6\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_CHOMP7\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_CHOMP8\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_CHOMP9\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_CHOMP10\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_CHOMP11\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_CHOMP12\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_CHOMP13\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_CHOMP14\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_CHOMP15\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_CHOMP16\0" as *const u8 as *const libc::c_char,
    b"S_JETJAW_SOUND\0" as *const u8 as *const libc::c_char,
    b"S_SNAILER1\0" as *const u8 as *const libc::c_char,
    b"S_SNAILER_FLICKY\0" as *const u8 as *const libc::c_char,
    b"S_VULTURE_STND\0" as *const u8 as *const libc::c_char,
    b"S_VULTURE_DRIFT\0" as *const u8 as *const libc::c_char,
    b"S_VULTURE_ZOOM1\0" as *const u8 as *const libc::c_char,
    b"S_VULTURE_ZOOM2\0" as *const u8 as *const libc::c_char,
    b"S_VULTURE_STUNNED\0" as *const u8 as *const libc::c_char,
    b"S_POINTY1\0" as *const u8 as *const libc::c_char,
    b"S_POINTYBALL1\0" as *const u8 as *const libc::c_char,
    b"S_ROBOHOOD_LOOK\0" as *const u8 as *const libc::c_char,
    b"S_ROBOHOOD_STAND\0" as *const u8 as *const libc::c_char,
    b"S_ROBOHOOD_FIRE1\0" as *const u8 as *const libc::c_char,
    b"S_ROBOHOOD_FIRE2\0" as *const u8 as *const libc::c_char,
    b"S_ROBOHOOD_JUMP1\0" as *const u8 as *const libc::c_char,
    b"S_ROBOHOOD_JUMP2\0" as *const u8 as *const libc::c_char,
    b"S_ROBOHOOD_JUMP3\0" as *const u8 as *const libc::c_char,
    b"S_FACESTABBER_STND1\0" as *const u8 as *const libc::c_char,
    b"S_FACESTABBER_STND2\0" as *const u8 as *const libc::c_char,
    b"S_FACESTABBER_STND3\0" as *const u8 as *const libc::c_char,
    b"S_FACESTABBER_STND4\0" as *const u8 as *const libc::c_char,
    b"S_FACESTABBER_STND5\0" as *const u8 as *const libc::c_char,
    b"S_FACESTABBER_STND6\0" as *const u8 as *const libc::c_char,
    b"S_FACESTABBER_CHARGE1\0" as *const u8 as *const libc::c_char,
    b"S_FACESTABBER_CHARGE2\0" as *const u8 as *const libc::c_char,
    b"S_FACESTABBER_CHARGE3\0" as *const u8 as *const libc::c_char,
    b"S_FACESTABBER_CHARGE4\0" as *const u8 as *const libc::c_char,
    b"S_FACESTABBER_PAIN\0" as *const u8 as *const libc::c_char,
    b"S_FACESTABBER_DIE1\0" as *const u8 as *const libc::c_char,
    b"S_FACESTABBER_DIE2\0" as *const u8 as *const libc::c_char,
    b"S_FACESTABBER_DIE3\0" as *const u8 as *const libc::c_char,
    b"S_FACESTABBERSPEAR\0" as *const u8 as *const libc::c_char,
    b"S_EGGGUARD_STND\0" as *const u8 as *const libc::c_char,
    b"S_EGGGUARD_WALK1\0" as *const u8 as *const libc::c_char,
    b"S_EGGGUARD_WALK2\0" as *const u8 as *const libc::c_char,
    b"S_EGGGUARD_WALK3\0" as *const u8 as *const libc::c_char,
    b"S_EGGGUARD_WALK4\0" as *const u8 as *const libc::c_char,
    b"S_EGGGUARD_MAD1\0" as *const u8 as *const libc::c_char,
    b"S_EGGGUARD_MAD2\0" as *const u8 as *const libc::c_char,
    b"S_EGGGUARD_MAD3\0" as *const u8 as *const libc::c_char,
    b"S_EGGGUARD_RUN1\0" as *const u8 as *const libc::c_char,
    b"S_EGGGUARD_RUN2\0" as *const u8 as *const libc::c_char,
    b"S_EGGGUARD_RUN3\0" as *const u8 as *const libc::c_char,
    b"S_EGGGUARD_RUN4\0" as *const u8 as *const libc::c_char,
    b"S_EGGSHIELD\0" as *const u8 as *const libc::c_char,
    b"S_EGGSHIELDBREAK\0" as *const u8 as *const libc::c_char,
    b"S_SNAPPER_SPAWN\0" as *const u8 as *const libc::c_char,
    b"S_SNAPPER_SPAWN2\0" as *const u8 as *const libc::c_char,
    b"S_GSNAPPER_STND\0" as *const u8 as *const libc::c_char,
    b"S_GSNAPPER1\0" as *const u8 as *const libc::c_char,
    b"S_GSNAPPER2\0" as *const u8 as *const libc::c_char,
    b"S_GSNAPPER3\0" as *const u8 as *const libc::c_char,
    b"S_GSNAPPER4\0" as *const u8 as *const libc::c_char,
    b"S_SNAPPER_XPLD\0" as *const u8 as *const libc::c_char,
    b"S_SNAPPER_LEG\0" as *const u8 as *const libc::c_char,
    b"S_SNAPPER_LEGRAISE\0" as *const u8 as *const libc::c_char,
    b"S_SNAPPER_HEAD\0" as *const u8 as *const libc::c_char,
    b"S_MINUS_INIT\0" as *const u8 as *const libc::c_char,
    b"S_MINUS_STND\0" as *const u8 as *const libc::c_char,
    b"S_MINUS_DIGGING1\0" as *const u8 as *const libc::c_char,
    b"S_MINUS_DIGGING2\0" as *const u8 as *const libc::c_char,
    b"S_MINUS_DIGGING3\0" as *const u8 as *const libc::c_char,
    b"S_MINUS_DIGGING4\0" as *const u8 as *const libc::c_char,
    b"S_MINUS_BURST0\0" as *const u8 as *const libc::c_char,
    b"S_MINUS_BURST1\0" as *const u8 as *const libc::c_char,
    b"S_MINUS_BURST2\0" as *const u8 as *const libc::c_char,
    b"S_MINUS_BURST3\0" as *const u8 as *const libc::c_char,
    b"S_MINUS_BURST4\0" as *const u8 as *const libc::c_char,
    b"S_MINUS_BURST5\0" as *const u8 as *const libc::c_char,
    b"S_MINUS_POPUP\0" as *const u8 as *const libc::c_char,
    b"S_MINUS_AERIAL1\0" as *const u8 as *const libc::c_char,
    b"S_MINUS_AERIAL2\0" as *const u8 as *const libc::c_char,
    b"S_MINUS_AERIAL3\0" as *const u8 as *const libc::c_char,
    b"S_MINUS_AERIAL4\0" as *const u8 as *const libc::c_char,
    b"S_MINUSDIRT1\0" as *const u8 as *const libc::c_char,
    b"S_MINUSDIRT2\0" as *const u8 as *const libc::c_char,
    b"S_MINUSDIRT3\0" as *const u8 as *const libc::c_char,
    b"S_MINUSDIRT4\0" as *const u8 as *const libc::c_char,
    b"S_MINUSDIRT5\0" as *const u8 as *const libc::c_char,
    b"S_MINUSDIRT6\0" as *const u8 as *const libc::c_char,
    b"S_MINUSDIRT7\0" as *const u8 as *const libc::c_char,
    b"S_SSHELL_STND\0" as *const u8 as *const libc::c_char,
    b"S_SSHELL_RUN1\0" as *const u8 as *const libc::c_char,
    b"S_SSHELL_RUN2\0" as *const u8 as *const libc::c_char,
    b"S_SSHELL_RUN3\0" as *const u8 as *const libc::c_char,
    b"S_SSHELL_RUN4\0" as *const u8 as *const libc::c_char,
    b"S_SSHELL_SPRING1\0" as *const u8 as *const libc::c_char,
    b"S_SSHELL_SPRING2\0" as *const u8 as *const libc::c_char,
    b"S_SSHELL_SPRING3\0" as *const u8 as *const libc::c_char,
    b"S_SSHELL_SPRING4\0" as *const u8 as *const libc::c_char,
    b"S_YSHELL_STND\0" as *const u8 as *const libc::c_char,
    b"S_YSHELL_RUN1\0" as *const u8 as *const libc::c_char,
    b"S_YSHELL_RUN2\0" as *const u8 as *const libc::c_char,
    b"S_YSHELL_RUN3\0" as *const u8 as *const libc::c_char,
    b"S_YSHELL_RUN4\0" as *const u8 as *const libc::c_char,
    b"S_YSHELL_SPRING1\0" as *const u8 as *const libc::c_char,
    b"S_YSHELL_SPRING2\0" as *const u8 as *const libc::c_char,
    b"S_YSHELL_SPRING3\0" as *const u8 as *const libc::c_char,
    b"S_YSHELL_SPRING4\0" as *const u8 as *const libc::c_char,
    b"S_UNIDUS_STND\0" as *const u8 as *const libc::c_char,
    b"S_UNIDUS_RUN\0" as *const u8 as *const libc::c_char,
    b"S_UNIDUS_BALL\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVORE_LOOK\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVORE_AWAKEN1\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVORE_AWAKEN2\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVORE_AWAKEN3\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVORE_GAS1\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVORE_GAS2\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVORE_GAS3\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVORE_GAS4\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVORE_GAS5\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVORE_GASREPEAT\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVORE_CLOSE1\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVORE_CLOSE2\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVOREGAS_1\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVOREGAS_2\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVOREGAS_3\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVOREGAS_4\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVOREGAS_5\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVOREGAS_6\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVOREGAS_7\0" as *const u8 as *const libc::c_char,
    b"S_CANARIVOREGAS_8\0" as *const u8 as *const libc::c_char,
    b"S_PYREFLY_FLY\0" as *const u8 as *const libc::c_char,
    b"S_PYREFLY_BURN\0" as *const u8 as *const libc::c_char,
    b"S_PYREFIRE1\0" as *const u8 as *const libc::c_char,
    b"S_PYREFIRE2\0" as *const u8 as *const libc::c_char,
    b"S_PTERABYTESPAWNER\0" as *const u8 as *const libc::c_char,
    b"S_PTERABYTEWAYPOINT\0" as *const u8 as *const libc::c_char,
    b"S_PTERABYTE_FLY1\0" as *const u8 as *const libc::c_char,
    b"S_PTERABYTE_FLY2\0" as *const u8 as *const libc::c_char,
    b"S_PTERABYTE_FLY3\0" as *const u8 as *const libc::c_char,
    b"S_PTERABYTE_FLY4\0" as *const u8 as *const libc::c_char,
    b"S_PTERABYTE_SWOOPDOWN\0" as *const u8 as *const libc::c_char,
    b"S_PTERABYTE_SWOOPUP\0" as *const u8 as *const libc::c_char,
    b"S_DRAGONBOMBER\0" as *const u8 as *const libc::c_char,
    b"S_DRAGONWING1\0" as *const u8 as *const libc::c_char,
    b"S_DRAGONWING2\0" as *const u8 as *const libc::c_char,
    b"S_DRAGONWING3\0" as *const u8 as *const libc::c_char,
    b"S_DRAGONWING4\0" as *const u8 as *const libc::c_char,
    b"S_DRAGONTAIL_LOADED\0" as *const u8 as *const libc::c_char,
    b"S_DRAGONTAIL_EMPTY\0" as *const u8 as *const libc::c_char,
    b"S_DRAGONTAIL_EMPTYLOOP\0" as *const u8 as *const libc::c_char,
    b"S_DRAGONTAIL_RELOAD\0" as *const u8 as *const libc::c_char,
    b"S_DRAGONMINE\0" as *const u8 as *const libc::c_char,
    b"S_DRAGONMINE_LAND1\0" as *const u8 as *const libc::c_char,
    b"S_DRAGONMINE_LAND2\0" as *const u8 as *const libc::c_char,
    b"S_DRAGONMINE_SLOWFLASH1\0" as *const u8 as *const libc::c_char,
    b"S_DRAGONMINE_SLOWFLASH2\0" as *const u8 as *const libc::c_char,
    b"S_DRAGONMINE_SLOWLOOP\0" as *const u8 as *const libc::c_char,
    b"S_DRAGONMINE_FASTFLASH1\0" as *const u8 as *const libc::c_char,
    b"S_DRAGONMINE_FASTFLASH2\0" as *const u8 as *const libc::c_char,
    b"S_DRAGONMINE_FASTLOOP\0" as *const u8 as *const libc::c_char,
    b"S_BOSSEXPLODE\0" as *const u8 as *const libc::c_char,
    b"S_SONIC3KBOSSEXPLOSION1\0" as *const u8 as *const libc::c_char,
    b"S_SONIC3KBOSSEXPLOSION2\0" as *const u8 as *const libc::c_char,
    b"S_SONIC3KBOSSEXPLOSION3\0" as *const u8 as *const libc::c_char,
    b"S_SONIC3KBOSSEXPLOSION4\0" as *const u8 as *const libc::c_char,
    b"S_SONIC3KBOSSEXPLOSION5\0" as *const u8 as *const libc::c_char,
    b"S_SONIC3KBOSSEXPLOSION6\0" as *const u8 as *const libc::c_char,
    b"S_JETFUME1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_STND\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_ROFL\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_LATK1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_LATK2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_LATK3\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_LATK4\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_LATK5\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_LATK6\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_LATK7\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_LATK8\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_LATK9\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_RATK1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_RATK2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_RATK3\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_RATK4\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_RATK5\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_RATK6\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_RATK7\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_RATK8\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_RATK9\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_PANIC1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_PANIC2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_PANIC3\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_PANIC4\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_PANIC5\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_PANIC6\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_PANIC7\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_PANIC8\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_PANIC9\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_PANIC10\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_PANIC11\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_PANIC12\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_PANIC13\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_PANIC14\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_PANIC15\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_PAIN\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_PAIN2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_DIE1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_DIE2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_DIE3\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_DIE4\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_FLEE1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_FLEE2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_BALL\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE_TARGET\0" as *const u8 as *const libc::c_char,
    b"S_BOSSEGLZ1\0" as *const u8 as *const libc::c_char,
    b"S_BOSSEGLZ2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE2_STND\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE2_POGO1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE2_POGO2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE2_POGO3\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE2_POGO4\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE2_POGO5\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE2_POGO6\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE2_POGO7\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE2_PAIN\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE2_PAIN2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE2_DIE1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE2_DIE2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE2_DIE3\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE2_DIE4\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE2_FLEE1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE2_FLEE2\0" as *const u8 as *const libc::c_char,
    b"S_BOSSTANK1\0" as *const u8 as *const libc::c_char,
    b"S_BOSSTANK2\0" as *const u8 as *const libc::c_char,
    b"S_BOSSSPIGOT\0" as *const u8 as *const libc::c_char,
    b"S_GOOP1\0" as *const u8 as *const libc::c_char,
    b"S_GOOP2\0" as *const u8 as *const libc::c_char,
    b"S_GOOP3\0" as *const u8 as *const libc::c_char,
    b"S_GOOPTRAIL\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_STND\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_SHOCK\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_ATK1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_ATK2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_ATK3A\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_ATK3B\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_ATK3C\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_ATK3D\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_ATK4\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_ATK5\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_ROFL\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_PAIN\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_PAIN2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_DIE1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_DIE2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_DIE3\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_DIE4\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_FLEE1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE3_FLEE2\0" as *const u8 as *const libc::c_char,
    b"S_FAKEMOBILE_INIT\0" as *const u8 as *const libc::c_char,
    b"S_FAKEMOBILE\0" as *const u8 as *const libc::c_char,
    b"S_FAKEMOBILE_ATK1\0" as *const u8 as *const libc::c_char,
    b"S_FAKEMOBILE_ATK2\0" as *const u8 as *const libc::c_char,
    b"S_FAKEMOBILE_ATK3A\0" as *const u8 as *const libc::c_char,
    b"S_FAKEMOBILE_ATK3B\0" as *const u8 as *const libc::c_char,
    b"S_FAKEMOBILE_ATK3C\0" as *const u8 as *const libc::c_char,
    b"S_FAKEMOBILE_ATK3D\0" as *const u8 as *const libc::c_char,
    b"S_FAKEMOBILE_DIE1\0" as *const u8 as *const libc::c_char,
    b"S_FAKEMOBILE_DIE2\0" as *const u8 as *const libc::c_char,
    b"S_BOSSSEBH1\0" as *const u8 as *const libc::c_char,
    b"S_BOSSSEBH2\0" as *const u8 as *const libc::c_char,
    b"S_SHOCKWAVE1\0" as *const u8 as *const libc::c_char,
    b"S_SHOCKWAVE2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_STND\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_LATK1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_LATK2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_LATK3\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_LATK4\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_LATK5\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_LATK6\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_RATK1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_RATK2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_RATK3\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_RATK4\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_RATK5\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_RATK6\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_RAISE1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_RAISE2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_PAIN1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_PAIN2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_DIE1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_DIE2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_DIE3\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_DIE4\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_FLEE1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_FLEE2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_MACE\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_MACE_DIE1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_MACE_DIE2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMOBILE4_MACE_DIE3\0" as *const u8 as *const libc::c_char,
    b"S_JETFLAME\0" as *const u8 as *const libc::c_char,
    b"S_EGGROBO1_STND\0" as *const u8 as *const libc::c_char,
    b"S_EGGROBO1_BSLAP1\0" as *const u8 as *const libc::c_char,
    b"S_EGGROBO1_BSLAP2\0" as *const u8 as *const libc::c_char,
    b"S_EGGROBO1_PISSED\0" as *const u8 as *const libc::c_char,
    b"S_EGGROBOJET\0" as *const u8 as *const libc::c_char,
    b"S_FANG_SETUP\0" as *const u8 as *const libc::c_char,
    b"S_FANG_INTRO0\0" as *const u8 as *const libc::c_char,
    b"S_FANG_INTRO1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_INTRO2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_INTRO3\0" as *const u8 as *const libc::c_char,
    b"S_FANG_INTRO4\0" as *const u8 as *const libc::c_char,
    b"S_FANG_INTRO5\0" as *const u8 as *const libc::c_char,
    b"S_FANG_INTRO6\0" as *const u8 as *const libc::c_char,
    b"S_FANG_INTRO7\0" as *const u8 as *const libc::c_char,
    b"S_FANG_INTRO8\0" as *const u8 as *const libc::c_char,
    b"S_FANG_INTRO9\0" as *const u8 as *const libc::c_char,
    b"S_FANG_INTRO10\0" as *const u8 as *const libc::c_char,
    b"S_FANG_INTRO11\0" as *const u8 as *const libc::c_char,
    b"S_FANG_INTRO12\0" as *const u8 as *const libc::c_char,
    b"S_FANG_CLONE1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_CLONE2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_CLONE3\0" as *const u8 as *const libc::c_char,
    b"S_FANG_CLONE4\0" as *const u8 as *const libc::c_char,
    b"S_FANG_IDLE0\0" as *const u8 as *const libc::c_char,
    b"S_FANG_IDLE1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_IDLE2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_IDLE3\0" as *const u8 as *const libc::c_char,
    b"S_FANG_IDLE4\0" as *const u8 as *const libc::c_char,
    b"S_FANG_IDLE5\0" as *const u8 as *const libc::c_char,
    b"S_FANG_IDLE6\0" as *const u8 as *const libc::c_char,
    b"S_FANG_IDLE7\0" as *const u8 as *const libc::c_char,
    b"S_FANG_IDLE8\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PAIN1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PAIN2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PATHINGSTART1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PATHINGSTART2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PATHING\0" as *const u8 as *const libc::c_char,
    b"S_FANG_BOUNCE1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_BOUNCE2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_BOUNCE3\0" as *const u8 as *const libc::c_char,
    b"S_FANG_BOUNCE4\0" as *const u8 as *const libc::c_char,
    b"S_FANG_FALL1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_FALL2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_CHECKPATH1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_CHECKPATH2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PATHINGCONT1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PATHINGCONT2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PATHINGCONT3\0" as *const u8 as *const libc::c_char,
    b"S_FANG_SKID1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_SKID2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_SKID3\0" as *const u8 as *const libc::c_char,
    b"S_FANG_CHOOSEATTACK\0" as *const u8 as *const libc::c_char,
    b"S_FANG_FIRESTART1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_FIRESTART2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_FIRE1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_FIRE2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_FIRE3\0" as *const u8 as *const libc::c_char,
    b"S_FANG_FIRE4\0" as *const u8 as *const libc::c_char,
    b"S_FANG_FIREREPEAT\0" as *const u8 as *const libc::c_char,
    b"S_FANG_LOBSHOT0\0" as *const u8 as *const libc::c_char,
    b"S_FANG_LOBSHOT1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_LOBSHOT2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_WAIT1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_WAIT2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_WALLHIT\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PINCHPATHINGSTART1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PINCHPATHINGSTART2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PINCHPATHING\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PINCHBOUNCE0\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PINCHBOUNCE1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PINCHBOUNCE2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PINCHBOUNCE3\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PINCHBOUNCE4\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PINCHFALL0\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PINCHFALL1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PINCHFALL2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PINCHSKID1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PINCHSKID2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PINCHLOBSHOT0\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PINCHLOBSHOT1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PINCHLOBSHOT2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PINCHLOBSHOT3\0" as *const u8 as *const libc::c_char,
    b"S_FANG_PINCHLOBSHOT4\0" as *const u8 as *const libc::c_char,
    b"S_FANG_DIE1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_DIE2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_DIE3\0" as *const u8 as *const libc::c_char,
    b"S_FANG_DIE4\0" as *const u8 as *const libc::c_char,
    b"S_FANG_DIE5\0" as *const u8 as *const libc::c_char,
    b"S_FANG_DIE6\0" as *const u8 as *const libc::c_char,
    b"S_FANG_DIE7\0" as *const u8 as *const libc::c_char,
    b"S_FANG_DIE8\0" as *const u8 as *const libc::c_char,
    b"S_FANG_FLEEPATHING1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_FLEEPATHING2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_FLEEBOUNCE1\0" as *const u8 as *const libc::c_char,
    b"S_FANG_FLEEBOUNCE2\0" as *const u8 as *const libc::c_char,
    b"S_FANG_KO\0" as *const u8 as *const libc::c_char,
    b"S_BROKENROBOTRANDOM\0" as *const u8 as *const libc::c_char,
    b"S_BROKENROBOTA\0" as *const u8 as *const libc::c_char,
    b"S_BROKENROBOTB\0" as *const u8 as *const libc::c_char,
    b"S_BROKENROBOTC\0" as *const u8 as *const libc::c_char,
    b"S_BROKENROBOTD\0" as *const u8 as *const libc::c_char,
    b"S_BROKENROBOTE\0" as *const u8 as *const libc::c_char,
    b"S_BROKENROBOTF\0" as *const u8 as *const libc::c_char,
    b"S_ALART1\0" as *const u8 as *const libc::c_char,
    b"S_ALART2\0" as *const u8 as *const libc::c_char,
    b"S_VWREF\0" as *const u8 as *const libc::c_char,
    b"S_VWREB\0" as *const u8 as *const libc::c_char,
    b"S_PROJECTORLIGHT1\0" as *const u8 as *const libc::c_char,
    b"S_PROJECTORLIGHT2\0" as *const u8 as *const libc::c_char,
    b"S_PROJECTORLIGHT3\0" as *const u8 as *const libc::c_char,
    b"S_PROJECTORLIGHT4\0" as *const u8 as *const libc::c_char,
    b"S_PROJECTORLIGHT5\0" as *const u8 as *const libc::c_char,
    b"S_FBOMB1\0" as *const u8 as *const libc::c_char,
    b"S_FBOMB2\0" as *const u8 as *const libc::c_char,
    b"S_FBOMB_EXPL1\0" as *const u8 as *const libc::c_char,
    b"S_FBOMB_EXPL2\0" as *const u8 as *const libc::c_char,
    b"S_FBOMB_EXPL3\0" as *const u8 as *const libc::c_char,
    b"S_FBOMB_EXPL4\0" as *const u8 as *const libc::c_char,
    b"S_FBOMB_EXPL5\0" as *const u8 as *const libc::c_char,
    b"S_FBOMB_EXPL6\0" as *const u8 as *const libc::c_char,
    b"S_TNTDUST_1\0" as *const u8 as *const libc::c_char,
    b"S_TNTDUST_2\0" as *const u8 as *const libc::c_char,
    b"S_TNTDUST_3\0" as *const u8 as *const libc::c_char,
    b"S_TNTDUST_4\0" as *const u8 as *const libc::c_char,
    b"S_TNTDUST_5\0" as *const u8 as *const libc::c_char,
    b"S_TNTDUST_6\0" as *const u8 as *const libc::c_char,
    b"S_TNTDUST_7\0" as *const u8 as *const libc::c_char,
    b"S_TNTDUST_8\0" as *const u8 as *const libc::c_char,
    b"S_FSGNA\0" as *const u8 as *const libc::c_char,
    b"S_FSGNB\0" as *const u8 as *const libc::c_char,
    b"S_FSGNC\0" as *const u8 as *const libc::c_char,
    b"S_FSGND\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_STND\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_STND2\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_WALK1\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_WALK2\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_WALK3\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_WALK4\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_WALK5\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_WALK6\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_SHOOT1\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_SHOOT2\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN1\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN2\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN3\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN4\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN5\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN6\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN7\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN8\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN9\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN10\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN11\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN12\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN13\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN14\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN15\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN16\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN17\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN18\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN19\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN20\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN21\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN22\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN23\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN24\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN25\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN26\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN27\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN28\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN29\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN30\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN31\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN32\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN33\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN34\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_PAIN35\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_HITFACE1\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_HITFACE2\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_HITFACE3\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_HITFACE4\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_DIE1\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_DIE2\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_DIE3\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_DIE4\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_DIE5\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_MISSILE1\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_MISSILE2\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_MISSILE3\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_GOOP\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_JUMP1\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_JUMP2\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_DESTROYPLAT1\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_DESTROYPLAT2\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_DESTROYPLAT3\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_HELPER\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_GOOP1\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_GOOP2\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_GOOP3\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_GOOP4\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_GOOP5\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_GOOP6\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_GOOP7\0" as *const u8 as *const libc::c_char,
    b"S_BLACKEGG_MISSILE\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_IDLE\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_WALK1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_WALK2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_WALK3\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_WALK4\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_WALK5\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_WALK6\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_CHOOSE_ATTACK1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_MISSILE_ATTACK1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_MISSILE_ATTACK2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_MISSILE_ATTACK3\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_MISSILE_ATTACK4\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_MISSILE_ATTACK5\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_MISSILE_ATTACK6\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_FLAME_ATTACK1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_FLAME_ATTACK2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_FLAME_ATTACK3\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_FLAME_ATTACK4\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_CHOOSE_ATTACK2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_VILE_ATTACK1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_VILE_ATTACK2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_VILE_ATTACK3\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_VILE_ATTACK4\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_VILE_ATTACK5\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_VILE_ATTACK6\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_NAPALM_ATTACK1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_NAPALM_ATTACK2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_NAPALM_ATTACK3\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_FINISH_ATTACK1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_FINISH_ATTACK2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_PAIN1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_PAIN2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_PAIN3\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_DIE1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_DIE2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_DIE3\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_DIE4\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_DIE5\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_DIE6\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_DIE7\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_DIE8\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_DEINVINCIBLERIZE\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMON_INVINCIBLERIZE\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONMISSILE\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONMISSILE_EXPLODE1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONMISSILE_EXPLODE2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONMISSILE_EXPLODE3\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONFLAMESHOT_FLY1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONFLAMESHOT_FLY2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONFLAMESHOT_FLY3\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONFLAMESHOT_DIE\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONFLAMEREST\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_INIT1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_INIT2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_PLAYSOUND\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER3\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER4\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER5\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER6\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER7\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER8\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER9\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER10\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER11\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER12\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER13\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER14\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER15\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER16\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER17\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER18\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER19\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER20\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER21\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER22\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER23\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER24\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_DIE1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_DIE2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_DIE3\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOMCHECK\0" as *const u8
        as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOMSUCCESS\0" as *const u8
        as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOMCHOOSE\0" as *const u8
        as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM3\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM4\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM5\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM6\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM7\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM8\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM9\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM10\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM11\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM12\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOMFAIL\0" as *const u8
        as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOMLOOP\0" as *const u8
        as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_REVIVE1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_REVIVE2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONELECTRICBARRIER_REVIVE3\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONTARGETRETICULE1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONTARGETRETICULE2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONTARGETRETICULE3\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONTARGETRETICULE4\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONTARGETRETICULE5\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONTARGETRETICULE6\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONTARGETRETICULE7\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONTARGETRETICULE8\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONTARGETRETICULE9\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONTARGETRETICULE10\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONTARGETRETICULE11\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONTARGETRETICULE12\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONTARGETRETICULE13\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONTARGETRETICULE14\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONTARGETDOT\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMBOMBLARGE_FLY1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMBOMBLARGE_FLY2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMBOMBLARGE_FLY3\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMBOMBLARGE_FLY4\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMBOMBLARGE_DIE1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMBOMBLARGE_DIE2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMBOMBLARGE_DIE3\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMBOMBLARGE_DIE4\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMBOMBSMALL\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMBOMBSMALL_DIE1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMBOMBSMALL_DIE2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMBOMBSMALL_DIE3\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMBOMBSMALL_DIE4\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMBOMBSMALL_DIE5\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMFLAME_FLY1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMFLAME_FLY2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMFLAME_FLY3\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMFLAME_FLY4\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMFLAME_FLY5\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMFLAME_FLY6\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONNAPALMFLAME_DIE\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONVILEEXPLOSION1\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONVILEEXPLOSION2\0" as *const u8 as *const libc::c_char,
    b"S_CYBRAKDEMONVILEEXPLOSION3\0" as *const u8 as *const libc::c_char,
    b"S_METALSONIC_RACE\0" as *const u8 as *const libc::c_char,
    b"S_METALSONIC_FLOAT\0" as *const u8 as *const libc::c_char,
    b"S_METALSONIC_VECTOR\0" as *const u8 as *const libc::c_char,
    b"S_METALSONIC_STUN\0" as *const u8 as *const libc::c_char,
    b"S_METALSONIC_RAISE\0" as *const u8 as *const libc::c_char,
    b"S_METALSONIC_GATHER\0" as *const u8 as *const libc::c_char,
    b"S_METALSONIC_DASH\0" as *const u8 as *const libc::c_char,
    b"S_METALSONIC_BOUNCE\0" as *const u8 as *const libc::c_char,
    b"S_METALSONIC_BADBOUNCE\0" as *const u8 as *const libc::c_char,
    b"S_METALSONIC_SHOOT\0" as *const u8 as *const libc::c_char,
    b"S_METALSONIC_PAIN\0" as *const u8 as *const libc::c_char,
    b"S_METALSONIC_DEATH1\0" as *const u8 as *const libc::c_char,
    b"S_METALSONIC_DEATH2\0" as *const u8 as *const libc::c_char,
    b"S_METALSONIC_DEATH3\0" as *const u8 as *const libc::c_char,
    b"S_METALSONIC_DEATH4\0" as *const u8 as *const libc::c_char,
    b"S_METALSONIC_FLEE1\0" as *const u8 as *const libc::c_char,
    b"S_METALSONIC_FLEE2\0" as *const u8 as *const libc::c_char,
    b"S_MSSHIELD_F1\0" as *const u8 as *const libc::c_char,
    b"S_MSSHIELD_F2\0" as *const u8 as *const libc::c_char,
    b"S_RING\0" as *const u8 as *const libc::c_char,
    b"S_BLUESPHERE\0" as *const u8 as *const libc::c_char,
    b"S_BLUESPHEREBONUS\0" as *const u8 as *const libc::c_char,
    b"S_BLUESPHERESPARK\0" as *const u8 as *const libc::c_char,
    b"S_BOMBSPHERE1\0" as *const u8 as *const libc::c_char,
    b"S_BOMBSPHERE2\0" as *const u8 as *const libc::c_char,
    b"S_BOMBSPHERE3\0" as *const u8 as *const libc::c_char,
    b"S_BOMBSPHERE4\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCHIP\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCHIPBONUS\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSSTAR\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSSTARXMAS\0" as *const u8 as *const libc::c_char,
    b"S_GRAVWELLGREEN\0" as *const u8 as *const libc::c_char,
    b"S_GRAVWELLRED\0" as *const u8 as *const libc::c_char,
    b"S_TEAMRING\0" as *const u8 as *const libc::c_char,
    b"S_TOKEN\0" as *const u8 as *const libc::c_char,
    b"S_REDFLAG\0" as *const u8 as *const libc::c_char,
    b"S_BLUEFLAG\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM1\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM2\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM3\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM4\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM5\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM6\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM7\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM8\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM9\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM10\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM11\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM12\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM13\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM14\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM15\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM16\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM17\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM18\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM19\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM20\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM21\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM22\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM23\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM24\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM25\0" as *const u8 as *const libc::c_char,
    b"S_EMBLEM26\0" as *const u8 as *const libc::c_char,
    b"S_CEMG1\0" as *const u8 as *const libc::c_char,
    b"S_CEMG2\0" as *const u8 as *const libc::c_char,
    b"S_CEMG3\0" as *const u8 as *const libc::c_char,
    b"S_CEMG4\0" as *const u8 as *const libc::c_char,
    b"S_CEMG5\0" as *const u8 as *const libc::c_char,
    b"S_CEMG6\0" as *const u8 as *const libc::c_char,
    b"S_CEMG7\0" as *const u8 as *const libc::c_char,
    b"S_SHRD1\0" as *const u8 as *const libc::c_char,
    b"S_SHRD2\0" as *const u8 as *const libc::c_char,
    b"S_SHRD3\0" as *const u8 as *const libc::c_char,
    b"S_BUBBLES1\0" as *const u8 as *const libc::c_char,
    b"S_BUBBLES2\0" as *const u8 as *const libc::c_char,
    b"S_BUBBLES3\0" as *const u8 as *const libc::c_char,
    b"S_BUBBLES4\0" as *const u8 as *const libc::c_char,
    b"S_SIGN\0" as *const u8 as *const libc::c_char,
    b"S_SIGNSPIN1\0" as *const u8 as *const libc::c_char,
    b"S_SIGNSPIN2\0" as *const u8 as *const libc::c_char,
    b"S_SIGNSPIN3\0" as *const u8 as *const libc::c_char,
    b"S_SIGNSPIN4\0" as *const u8 as *const libc::c_char,
    b"S_SIGNSPIN5\0" as *const u8 as *const libc::c_char,
    b"S_SIGNSPIN6\0" as *const u8 as *const libc::c_char,
    b"S_SIGNPLAYER\0" as *const u8 as *const libc::c_char,
    b"S_SIGNSLOW\0" as *const u8 as *const libc::c_char,
    b"S_SIGNSTOP\0" as *const u8 as *const libc::c_char,
    b"S_SIGNBOARD\0" as *const u8 as *const libc::c_char,
    b"S_EGGMANSIGN\0" as *const u8 as *const libc::c_char,
    b"S_CLEARSIGN\0" as *const u8 as *const libc::c_char,
    b"S_SPIKEBALL1\0" as *const u8 as *const libc::c_char,
    b"S_SPIKEBALL2\0" as *const u8 as *const libc::c_char,
    b"S_SPIKEBALL3\0" as *const u8 as *const libc::c_char,
    b"S_SPIKEBALL4\0" as *const u8 as *const libc::c_char,
    b"S_SPIKEBALL5\0" as *const u8 as *const libc::c_char,
    b"S_SPIKEBALL6\0" as *const u8 as *const libc::c_char,
    b"S_SPIKEBALL7\0" as *const u8 as *const libc::c_char,
    b"S_SPIKEBALL8\0" as *const u8 as *const libc::c_char,
    b"S_SPINFIRE1\0" as *const u8 as *const libc::c_char,
    b"S_SPINFIRE2\0" as *const u8 as *const libc::c_char,
    b"S_SPINFIRE3\0" as *const u8 as *const libc::c_char,
    b"S_SPINFIRE4\0" as *const u8 as *const libc::c_char,
    b"S_SPINFIRE5\0" as *const u8 as *const libc::c_char,
    b"S_SPINFIRE6\0" as *const u8 as *const libc::c_char,
    b"S_TEAM_SPINFIRE1\0" as *const u8 as *const libc::c_char,
    b"S_TEAM_SPINFIRE2\0" as *const u8 as *const libc::c_char,
    b"S_TEAM_SPINFIRE3\0" as *const u8 as *const libc::c_char,
    b"S_TEAM_SPINFIRE4\0" as *const u8 as *const libc::c_char,
    b"S_TEAM_SPINFIRE5\0" as *const u8 as *const libc::c_char,
    b"S_TEAM_SPINFIRE6\0" as *const u8 as *const libc::c_char,
    b"S_SPIKE1\0" as *const u8 as *const libc::c_char,
    b"S_SPIKE2\0" as *const u8 as *const libc::c_char,
    b"S_SPIKE3\0" as *const u8 as *const libc::c_char,
    b"S_SPIKE4\0" as *const u8 as *const libc::c_char,
    b"S_SPIKE5\0" as *const u8 as *const libc::c_char,
    b"S_SPIKE6\0" as *const u8 as *const libc::c_char,
    b"S_SPIKED1\0" as *const u8 as *const libc::c_char,
    b"S_SPIKED2\0" as *const u8 as *const libc::c_char,
    b"S_WALLSPIKE1\0" as *const u8 as *const libc::c_char,
    b"S_WALLSPIKE2\0" as *const u8 as *const libc::c_char,
    b"S_WALLSPIKE3\0" as *const u8 as *const libc::c_char,
    b"S_WALLSPIKE4\0" as *const u8 as *const libc::c_char,
    b"S_WALLSPIKE5\0" as *const u8 as *const libc::c_char,
    b"S_WALLSPIKE6\0" as *const u8 as *const libc::c_char,
    b"S_WALLSPIKEBASE\0" as *const u8 as *const libc::c_char,
    b"S_WALLSPIKED1\0" as *const u8 as *const libc::c_char,
    b"S_WALLSPIKED2\0" as *const u8 as *const libc::c_char,
    b"S_STARPOST_IDLE\0" as *const u8 as *const libc::c_char,
    b"S_STARPOST_FLASH\0" as *const u8 as *const libc::c_char,
    b"S_STARPOST_STARTSPIN\0" as *const u8 as *const libc::c_char,
    b"S_STARPOST_SPIN\0" as *const u8 as *const libc::c_char,
    b"S_STARPOST_ENDSPIN\0" as *const u8 as *const libc::c_char,
    b"S_BIGMINE_IDLE\0" as *const u8 as *const libc::c_char,
    b"S_BIGMINE_ALERT1\0" as *const u8 as *const libc::c_char,
    b"S_BIGMINE_ALERT2\0" as *const u8 as *const libc::c_char,
    b"S_BIGMINE_ALERT3\0" as *const u8 as *const libc::c_char,
    b"S_BIGMINE_SET1\0" as *const u8 as *const libc::c_char,
    b"S_BIGMINE_SET2\0" as *const u8 as *const libc::c_char,
    b"S_BIGMINE_SET3\0" as *const u8 as *const libc::c_char,
    b"S_BIGMINE_BLAST1\0" as *const u8 as *const libc::c_char,
    b"S_BIGMINE_BLAST2\0" as *const u8 as *const libc::c_char,
    b"S_BIGMINE_BLAST3\0" as *const u8 as *const libc::c_char,
    b"S_BIGMINE_BLAST4\0" as *const u8 as *const libc::c_char,
    b"S_BIGMINE_BLAST5\0" as *const u8 as *const libc::c_char,
    b"S_CANNONLAUNCHER1\0" as *const u8 as *const libc::c_char,
    b"S_CANNONLAUNCHER2\0" as *const u8 as *const libc::c_char,
    b"S_CANNONLAUNCHER3\0" as *const u8 as *const libc::c_char,
    b"S_BOXSPARKLE1\0" as *const u8 as *const libc::c_char,
    b"S_BOXSPARKLE2\0" as *const u8 as *const libc::c_char,
    b"S_BOXSPARKLE3\0" as *const u8 as *const libc::c_char,
    b"S_BOXSPARKLE4\0" as *const u8 as *const libc::c_char,
    b"S_BOX_FLICKER\0" as *const u8 as *const libc::c_char,
    b"S_BOX_POP1\0" as *const u8 as *const libc::c_char,
    b"S_BOX_POP2\0" as *const u8 as *const libc::c_char,
    b"S_GOLDBOX_FLICKER\0" as *const u8 as *const libc::c_char,
    b"S_GOLDBOX_OFF1\0" as *const u8 as *const libc::c_char,
    b"S_GOLDBOX_OFF2\0" as *const u8 as *const libc::c_char,
    b"S_GOLDBOX_OFF3\0" as *const u8 as *const libc::c_char,
    b"S_GOLDBOX_OFF4\0" as *const u8 as *const libc::c_char,
    b"S_GOLDBOX_OFF5\0" as *const u8 as *const libc::c_char,
    b"S_GOLDBOX_OFF6\0" as *const u8 as *const libc::c_char,
    b"S_GOLDBOX_OFF7\0" as *const u8 as *const libc::c_char,
    b"S_MYSTERY_BOX\0" as *const u8 as *const libc::c_char,
    b"S_RING_BOX\0" as *const u8 as *const libc::c_char,
    b"S_PITY_BOX\0" as *const u8 as *const libc::c_char,
    b"S_ATTRACT_BOX\0" as *const u8 as *const libc::c_char,
    b"S_FORCE_BOX\0" as *const u8 as *const libc::c_char,
    b"S_ARMAGEDDON_BOX\0" as *const u8 as *const libc::c_char,
    b"S_WHIRLWIND_BOX\0" as *const u8 as *const libc::c_char,
    b"S_ELEMENTAL_BOX\0" as *const u8 as *const libc::c_char,
    b"S_SNEAKERS_BOX\0" as *const u8 as *const libc::c_char,
    b"S_INVULN_BOX\0" as *const u8 as *const libc::c_char,
    b"S_1UP_BOX\0" as *const u8 as *const libc::c_char,
    b"S_EGGMAN_BOX\0" as *const u8 as *const libc::c_char,
    b"S_MIXUP_BOX\0" as *const u8 as *const libc::c_char,
    b"S_GRAVITY_BOX\0" as *const u8 as *const libc::c_char,
    b"S_RECYCLER_BOX\0" as *const u8 as *const libc::c_char,
    b"S_SCORE1K_BOX\0" as *const u8 as *const libc::c_char,
    b"S_SCORE10K_BOX\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEAURA_BOX\0" as *const u8 as *const libc::c_char,
    b"S_BUBBLEWRAP_BOX\0" as *const u8 as *const libc::c_char,
    b"S_THUNDERCOIN_BOX\0" as *const u8 as *const libc::c_char,
    b"S_PITY_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"S_ATTRACT_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"S_FORCE_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"S_ARMAGEDDON_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"S_WHIRLWIND_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"S_ELEMENTAL_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"S_SNEAKERS_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"S_INVULN_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"S_EGGMAN_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"S_GRAVITY_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEAURA_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"S_BUBBLEWRAP_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"S_THUNDERCOIN_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"S_RING_REDBOX1\0" as *const u8 as *const libc::c_char,
    b"S_RING_REDBOX2\0" as *const u8 as *const libc::c_char,
    b"S_REDBOX_POP1\0" as *const u8 as *const libc::c_char,
    b"S_REDBOX_POP2\0" as *const u8 as *const libc::c_char,
    b"S_RING_BLUEBOX1\0" as *const u8 as *const libc::c_char,
    b"S_RING_BLUEBOX2\0" as *const u8 as *const libc::c_char,
    b"S_BLUEBOX_POP1\0" as *const u8 as *const libc::c_char,
    b"S_BLUEBOX_POP2\0" as *const u8 as *const libc::c_char,
    b"S_RING_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_RING_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_PITY_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_PITY_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_ATTRACT_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_ATTRACT_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_FORCE_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_FORCE_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_ARMAGEDDON_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_ARMAGEDDON_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_WHIRLWIND_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_WHIRLWIND_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_ELEMENTAL_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_ELEMENTAL_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_SNEAKERS_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_SNEAKERS_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_INVULN_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_INVULN_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_1UP_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_1UP_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_EGGMAN_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_EGGMAN_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_MIXUP_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_MIXUP_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_GRAVITY_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_GRAVITY_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_RECYCLER_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_RECYCLER_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_SCORE1K_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_SCORE1K_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_SCORE10K_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_SCORE10K_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEAURA_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEAURA_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_BUBBLEWRAP_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_BUBBLEWRAP_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_THUNDERCOIN_ICON1\0" as *const u8 as *const libc::c_char,
    b"S_THUNDERCOIN_ICON2\0" as *const u8 as *const libc::c_char,
    b"S_ROCKET\0" as *const u8 as *const libc::c_char,
    b"S_LASER\0" as *const u8 as *const libc::c_char,
    b"S_LASER2\0" as *const u8 as *const libc::c_char,
    b"S_LASERFLASH\0" as *const u8 as *const libc::c_char,
    b"S_LASERFLAME1\0" as *const u8 as *const libc::c_char,
    b"S_LASERFLAME2\0" as *const u8 as *const libc::c_char,
    b"S_LASERFLAME3\0" as *const u8 as *const libc::c_char,
    b"S_LASERFLAME4\0" as *const u8 as *const libc::c_char,
    b"S_LASERFLAME5\0" as *const u8 as *const libc::c_char,
    b"S_TORPEDO\0" as *const u8 as *const libc::c_char,
    b"S_ENERGYBALL1\0" as *const u8 as *const libc::c_char,
    b"S_ENERGYBALL2\0" as *const u8 as *const libc::c_char,
    b"S_MINE1\0" as *const u8 as *const libc::c_char,
    b"S_MINE_BOOM1\0" as *const u8 as *const libc::c_char,
    b"S_MINE_BOOM2\0" as *const u8 as *const libc::c_char,
    b"S_MINE_BOOM3\0" as *const u8 as *const libc::c_char,
    b"S_MINE_BOOM4\0" as *const u8 as *const libc::c_char,
    b"S_JETBULLET1\0" as *const u8 as *const libc::c_char,
    b"S_JETBULLET2\0" as *const u8 as *const libc::c_char,
    b"S_TURRETLASER\0" as *const u8 as *const libc::c_char,
    b"S_TURRETLASEREXPLODE1\0" as *const u8 as *const libc::c_char,
    b"S_TURRETLASEREXPLODE2\0" as *const u8 as *const libc::c_char,
    b"S_CANNONBALL1\0" as *const u8 as *const libc::c_char,
    b"S_ARROW\0" as *const u8 as *const libc::c_char,
    b"S_ARROWBONK\0" as *const u8 as *const libc::c_char,
    b"S_DEMONFIRE\0" as *const u8 as *const libc::c_char,
    b"S_LETTER\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALLEAF1\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALLEAF2\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALLEAF3\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALLEAF4\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALLEAF5\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALLEAF6\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALLEAF7\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALLEAF8\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALLEAF9\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALLEAF10\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALLEAF11\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALLEAF12\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALLEAF13\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALLEAF14\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALLEAF15\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALLEAF16\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWER1\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWER2\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWER3\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWER4\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWER5\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWER6\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWER7\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWER8\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWER9\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWER10\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWER11\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWER12\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWER13\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWER14\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWER15\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWER16\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWERF1\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWERF2\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWERF3\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWERF4\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWERF5\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWERF6\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWERF7\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWERF8\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWERF9\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWERF10\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWERF11\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWERF12\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWERF13\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWERF14\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWERF15\0" as *const u8 as *const libc::c_char,
    b"S_TUTORIALFLOWERF16\0" as *const u8 as *const libc::c_char,
    b"S_GFZFLOWERA\0" as *const u8 as *const libc::c_char,
    b"S_GFZFLOWERB\0" as *const u8 as *const libc::c_char,
    b"S_GFZFLOWERC\0" as *const u8 as *const libc::c_char,
    b"S_BLUEBERRYBUSH\0" as *const u8 as *const libc::c_char,
    b"S_BERRYBUSH\0" as *const u8 as *const libc::c_char,
    b"S_BUSH\0" as *const u8 as *const libc::c_char,
    b"S_GFZTREE\0" as *const u8 as *const libc::c_char,
    b"S_GFZBERRYTREE\0" as *const u8 as *const libc::c_char,
    b"S_GFZCHERRYTREE\0" as *const u8 as *const libc::c_char,
    b"S_CHECKERTREE\0" as *const u8 as *const libc::c_char,
    b"S_CHECKERSUNSETTREE\0" as *const u8 as *const libc::c_char,
    b"S_FHZTREE\0" as *const u8 as *const libc::c_char,
    b"S_FHZPINKTREE\0" as *const u8 as *const libc::c_char,
    b"S_POLYGONTREE\0" as *const u8 as *const libc::c_char,
    b"S_BUSHTREE\0" as *const u8 as *const libc::c_char,
    b"S_BUSHREDTREE\0" as *const u8 as *const libc::c_char,
    b"S_SPRINGTREE\0" as *const u8 as *const libc::c_char,
    b"S_THZFLOWERA\0" as *const u8 as *const libc::c_char,
    b"S_THZFLOWERB\0" as *const u8 as *const libc::c_char,
    b"S_THZFLOWERC\0" as *const u8 as *const libc::c_char,
    b"S_THZTREE\0" as *const u8 as *const libc::c_char,
    b"S_THZTREEBRANCH1\0" as *const u8 as *const libc::c_char,
    b"S_THZTREEBRANCH2\0" as *const u8 as *const libc::c_char,
    b"S_THZTREEBRANCH3\0" as *const u8 as *const libc::c_char,
    b"S_THZTREEBRANCH4\0" as *const u8 as *const libc::c_char,
    b"S_THZTREEBRANCH5\0" as *const u8 as *const libc::c_char,
    b"S_THZTREEBRANCH6\0" as *const u8 as *const libc::c_char,
    b"S_THZTREEBRANCH7\0" as *const u8 as *const libc::c_char,
    b"S_THZTREEBRANCH8\0" as *const u8 as *const libc::c_char,
    b"S_THZTREEBRANCH9\0" as *const u8 as *const libc::c_char,
    b"S_THZTREEBRANCH10\0" as *const u8 as *const libc::c_char,
    b"S_THZTREEBRANCH11\0" as *const u8 as *const libc::c_char,
    b"S_THZTREEBRANCH12\0" as *const u8 as *const libc::c_char,
    b"S_THZTREEBRANCH13\0" as *const u8 as *const libc::c_char,
    b"S_ALARM1\0" as *const u8 as *const libc::c_char,
    b"S_GARGOYLE\0" as *const u8 as *const libc::c_char,
    b"S_BIGGARGOYLE\0" as *const u8 as *const libc::c_char,
    b"S_SEAWEED1\0" as *const u8 as *const libc::c_char,
    b"S_SEAWEED2\0" as *const u8 as *const libc::c_char,
    b"S_SEAWEED3\0" as *const u8 as *const libc::c_char,
    b"S_SEAWEED4\0" as *const u8 as *const libc::c_char,
    b"S_SEAWEED5\0" as *const u8 as *const libc::c_char,
    b"S_SEAWEED6\0" as *const u8 as *const libc::c_char,
    b"S_DRIPA1\0" as *const u8 as *const libc::c_char,
    b"S_DRIPA2\0" as *const u8 as *const libc::c_char,
    b"S_DRIPA3\0" as *const u8 as *const libc::c_char,
    b"S_DRIPA4\0" as *const u8 as *const libc::c_char,
    b"S_DRIPB1\0" as *const u8 as *const libc::c_char,
    b"S_DRIPC1\0" as *const u8 as *const libc::c_char,
    b"S_DRIPC2\0" as *const u8 as *const libc::c_char,
    b"S_CORAL1\0" as *const u8 as *const libc::c_char,
    b"S_CORAL2\0" as *const u8 as *const libc::c_char,
    b"S_CORAL3\0" as *const u8 as *const libc::c_char,
    b"S_CORAL4\0" as *const u8 as *const libc::c_char,
    b"S_CORAL5\0" as *const u8 as *const libc::c_char,
    b"S_BLUECRYSTAL1\0" as *const u8 as *const libc::c_char,
    b"S_KELP\0" as *const u8 as *const libc::c_char,
    b"S_ANIMALGAETOP1\0" as *const u8 as *const libc::c_char,
    b"S_ANIMALGAETOP2\0" as *const u8 as *const libc::c_char,
    b"S_ANIMALGAESEG\0" as *const u8 as *const libc::c_char,
    b"S_DSZSTALAGMITE\0" as *const u8 as *const libc::c_char,
    b"S_DSZ2STALAGMITE\0" as *const u8 as *const libc::c_char,
    b"S_LIGHTBEAM1\0" as *const u8 as *const libc::c_char,
    b"S_LIGHTBEAM2\0" as *const u8 as *const libc::c_char,
    b"S_LIGHTBEAM3\0" as *const u8 as *const libc::c_char,
    b"S_LIGHTBEAM4\0" as *const u8 as *const libc::c_char,
    b"S_LIGHTBEAM5\0" as *const u8 as *const libc::c_char,
    b"S_LIGHTBEAM6\0" as *const u8 as *const libc::c_char,
    b"S_LIGHTBEAM7\0" as *const u8 as *const libc::c_char,
    b"S_LIGHTBEAM8\0" as *const u8 as *const libc::c_char,
    b"S_LIGHTBEAM9\0" as *const u8 as *const libc::c_char,
    b"S_LIGHTBEAM10\0" as *const u8 as *const libc::c_char,
    b"S_LIGHTBEAM11\0" as *const u8 as *const libc::c_char,
    b"S_LIGHTBEAM12\0" as *const u8 as *const libc::c_char,
    b"S_CEZCHAIN\0" as *const u8 as *const libc::c_char,
    b"S_FLAME\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEPARTICLE\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEREST\0" as *const u8 as *const libc::c_char,
    b"S_EGGSTATUE1\0" as *const u8 as *const libc::c_char,
    b"S_SLING1\0" as *const u8 as *const libc::c_char,
    b"S_SLING2\0" as *const u8 as *const libc::c_char,
    b"S_SMALLMACECHAIN\0" as *const u8 as *const libc::c_char,
    b"S_BIGMACECHAIN\0" as *const u8 as *const libc::c_char,
    b"S_SMALLMACE\0" as *const u8 as *const libc::c_char,
    b"S_BIGMACE\0" as *const u8 as *const libc::c_char,
    b"S_SMALLGRABCHAIN\0" as *const u8 as *const libc::c_char,
    b"S_BIGGRABCHAIN\0" as *const u8 as *const libc::c_char,
    b"S_YELLOWSPRINGBALL\0" as *const u8 as *const libc::c_char,
    b"S_YELLOWSPRINGBALL2\0" as *const u8 as *const libc::c_char,
    b"S_YELLOWSPRINGBALL3\0" as *const u8 as *const libc::c_char,
    b"S_YELLOWSPRINGBALL4\0" as *const u8 as *const libc::c_char,
    b"S_YELLOWSPRINGBALL5\0" as *const u8 as *const libc::c_char,
    b"S_REDSPRINGBALL\0" as *const u8 as *const libc::c_char,
    b"S_REDSPRINGBALL2\0" as *const u8 as *const libc::c_char,
    b"S_REDSPRINGBALL3\0" as *const u8 as *const libc::c_char,
    b"S_REDSPRINGBALL4\0" as *const u8 as *const libc::c_char,
    b"S_REDSPRINGBALL5\0" as *const u8 as *const libc::c_char,
    b"S_SMALLFIREBAR1\0" as *const u8 as *const libc::c_char,
    b"S_SMALLFIREBAR2\0" as *const u8 as *const libc::c_char,
    b"S_SMALLFIREBAR3\0" as *const u8 as *const libc::c_char,
    b"S_SMALLFIREBAR4\0" as *const u8 as *const libc::c_char,
    b"S_SMALLFIREBAR5\0" as *const u8 as *const libc::c_char,
    b"S_SMALLFIREBAR6\0" as *const u8 as *const libc::c_char,
    b"S_SMALLFIREBAR7\0" as *const u8 as *const libc::c_char,
    b"S_SMALLFIREBAR8\0" as *const u8 as *const libc::c_char,
    b"S_SMALLFIREBAR9\0" as *const u8 as *const libc::c_char,
    b"S_SMALLFIREBAR10\0" as *const u8 as *const libc::c_char,
    b"S_SMALLFIREBAR11\0" as *const u8 as *const libc::c_char,
    b"S_SMALLFIREBAR12\0" as *const u8 as *const libc::c_char,
    b"S_SMALLFIREBAR13\0" as *const u8 as *const libc::c_char,
    b"S_SMALLFIREBAR14\0" as *const u8 as *const libc::c_char,
    b"S_SMALLFIREBAR15\0" as *const u8 as *const libc::c_char,
    b"S_SMALLFIREBAR16\0" as *const u8 as *const libc::c_char,
    b"S_BIGFIREBAR1\0" as *const u8 as *const libc::c_char,
    b"S_BIGFIREBAR2\0" as *const u8 as *const libc::c_char,
    b"S_BIGFIREBAR3\0" as *const u8 as *const libc::c_char,
    b"S_BIGFIREBAR4\0" as *const u8 as *const libc::c_char,
    b"S_BIGFIREBAR5\0" as *const u8 as *const libc::c_char,
    b"S_BIGFIREBAR6\0" as *const u8 as *const libc::c_char,
    b"S_BIGFIREBAR7\0" as *const u8 as *const libc::c_char,
    b"S_BIGFIREBAR8\0" as *const u8 as *const libc::c_char,
    b"S_BIGFIREBAR9\0" as *const u8 as *const libc::c_char,
    b"S_BIGFIREBAR10\0" as *const u8 as *const libc::c_char,
    b"S_BIGFIREBAR11\0" as *const u8 as *const libc::c_char,
    b"S_BIGFIREBAR12\0" as *const u8 as *const libc::c_char,
    b"S_BIGFIREBAR13\0" as *const u8 as *const libc::c_char,
    b"S_BIGFIREBAR14\0" as *const u8 as *const libc::c_char,
    b"S_BIGFIREBAR15\0" as *const u8 as *const libc::c_char,
    b"S_BIGFIREBAR16\0" as *const u8 as *const libc::c_char,
    b"S_CEZFLOWER\0" as *const u8 as *const libc::c_char,
    b"S_CEZPOLE\0" as *const u8 as *const libc::c_char,
    b"S_CEZBANNER1\0" as *const u8 as *const libc::c_char,
    b"S_CEZBANNER2\0" as *const u8 as *const libc::c_char,
    b"S_PINETREE\0" as *const u8 as *const libc::c_char,
    b"S_CEZBUSH1\0" as *const u8 as *const libc::c_char,
    b"S_CEZBUSH2\0" as *const u8 as *const libc::c_char,
    b"S_CANDLE\0" as *const u8 as *const libc::c_char,
    b"S_CANDLEPRICKET\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEHOLDER\0" as *const u8 as *const libc::c_char,
    b"S_FIRETORCH\0" as *const u8 as *const libc::c_char,
    b"S_WAVINGFLAG\0" as *const u8 as *const libc::c_char,
    b"S_WAVINGFLAGSEG1\0" as *const u8 as *const libc::c_char,
    b"S_WAVINGFLAGSEG2\0" as *const u8 as *const libc::c_char,
    b"S_CRAWLASTATUE\0" as *const u8 as *const libc::c_char,
    b"S_FACESTABBERSTATUE\0" as *const u8 as *const libc::c_char,
    b"S_SUSPICIOUSFACESTABBERSTATUE_WAIT\0" as *const u8 as *const libc::c_char,
    b"S_SUSPICIOUSFACESTABBERSTATUE_BURST1\0" as *const u8 as *const libc::c_char,
    b"S_SUSPICIOUSFACESTABBERSTATUE_BURST2\0" as *const u8 as *const libc::c_char,
    b"S_BRAMBLES\0" as *const u8 as *const libc::c_char,
    b"S_BIGTUMBLEWEED\0" as *const u8 as *const libc::c_char,
    b"S_BIGTUMBLEWEED_ROLL1\0" as *const u8 as *const libc::c_char,
    b"S_BIGTUMBLEWEED_ROLL2\0" as *const u8 as *const libc::c_char,
    b"S_BIGTUMBLEWEED_ROLL3\0" as *const u8 as *const libc::c_char,
    b"S_BIGTUMBLEWEED_ROLL4\0" as *const u8 as *const libc::c_char,
    b"S_BIGTUMBLEWEED_ROLL5\0" as *const u8 as *const libc::c_char,
    b"S_BIGTUMBLEWEED_ROLL6\0" as *const u8 as *const libc::c_char,
    b"S_BIGTUMBLEWEED_ROLL7\0" as *const u8 as *const libc::c_char,
    b"S_BIGTUMBLEWEED_ROLL8\0" as *const u8 as *const libc::c_char,
    b"S_LITTLETUMBLEWEED\0" as *const u8 as *const libc::c_char,
    b"S_LITTLETUMBLEWEED_ROLL1\0" as *const u8 as *const libc::c_char,
    b"S_LITTLETUMBLEWEED_ROLL2\0" as *const u8 as *const libc::c_char,
    b"S_LITTLETUMBLEWEED_ROLL3\0" as *const u8 as *const libc::c_char,
    b"S_LITTLETUMBLEWEED_ROLL4\0" as *const u8 as *const libc::c_char,
    b"S_LITTLETUMBLEWEED_ROLL5\0" as *const u8 as *const libc::c_char,
    b"S_LITTLETUMBLEWEED_ROLL6\0" as *const u8 as *const libc::c_char,
    b"S_LITTLETUMBLEWEED_ROLL7\0" as *const u8 as *const libc::c_char,
    b"S_LITTLETUMBLEWEED_ROLL8\0" as *const u8 as *const libc::c_char,
    b"S_CACTI1\0" as *const u8 as *const libc::c_char,
    b"S_CACTI2\0" as *const u8 as *const libc::c_char,
    b"S_CACTI3\0" as *const u8 as *const libc::c_char,
    b"S_CACTI4\0" as *const u8 as *const libc::c_char,
    b"S_CACTI5\0" as *const u8 as *const libc::c_char,
    b"S_CACTI6\0" as *const u8 as *const libc::c_char,
    b"S_CACTI7\0" as *const u8 as *const libc::c_char,
    b"S_CACTI8\0" as *const u8 as *const libc::c_char,
    b"S_CACTI9\0" as *const u8 as *const libc::c_char,
    b"S_CACTI10\0" as *const u8 as *const libc::c_char,
    b"S_CACTI11\0" as *const u8 as *const libc::c_char,
    b"S_CACTITINYSEG\0" as *const u8 as *const libc::c_char,
    b"S_CACTISMALLSEG\0" as *const u8 as *const libc::c_char,
    b"S_ARIDSIGN_CAUTION\0" as *const u8 as *const libc::c_char,
    b"S_ARIDSIGN_CACTI\0" as *const u8 as *const libc::c_char,
    b"S_ARIDSIGN_SHARPTURN\0" as *const u8 as *const libc::c_char,
    b"S_OILLAMP\0" as *const u8 as *const libc::c_char,
    b"S_OILLAMPFLARE\0" as *const u8 as *const libc::c_char,
    b"S_TNTBARREL_STND1\0" as *const u8 as *const libc::c_char,
    b"S_TNTBARREL_EXPL1\0" as *const u8 as *const libc::c_char,
    b"S_TNTBARREL_EXPL2\0" as *const u8 as *const libc::c_char,
    b"S_TNTBARREL_EXPL3\0" as *const u8 as *const libc::c_char,
    b"S_TNTBARREL_EXPL4\0" as *const u8 as *const libc::c_char,
    b"S_TNTBARREL_EXPL5\0" as *const u8 as *const libc::c_char,
    b"S_TNTBARREL_EXPL6\0" as *const u8 as *const libc::c_char,
    b"S_TNTBARREL_EXPL7\0" as *const u8 as *const libc::c_char,
    b"S_TNTBARREL_FLYING\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER1\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER2\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER3\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER4\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER5\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER6\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER7\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER8\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER9\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER10\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER11\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER12\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER13\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER14\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER15\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER16\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER17\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER18\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER19\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER20\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER21\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER22\0" as *const u8 as *const libc::c_char,
    b"S_PROXIMITY_TNT_TRIGGER23\0" as *const u8 as *const libc::c_char,
    b"S_DUSTDEVIL\0" as *const u8 as *const libc::c_char,
    b"S_DUSTLAYER1\0" as *const u8 as *const libc::c_char,
    b"S_DUSTLAYER2\0" as *const u8 as *const libc::c_char,
    b"S_DUSTLAYER3\0" as *const u8 as *const libc::c_char,
    b"S_DUSTLAYER4\0" as *const u8 as *const libc::c_char,
    b"S_DUSTLAYER5\0" as *const u8 as *const libc::c_char,
    b"S_ARIDDUST1\0" as *const u8 as *const libc::c_char,
    b"S_ARIDDUST2\0" as *const u8 as *const libc::c_char,
    b"S_ARIDDUST3\0" as *const u8 as *const libc::c_char,
    b"S_MINECART_IDLE\0" as *const u8 as *const libc::c_char,
    b"S_MINECART_DTH1\0" as *const u8 as *const libc::c_char,
    b"S_MINECARTEND\0" as *const u8 as *const libc::c_char,
    b"S_MINECARTSEG_FRONT\0" as *const u8 as *const libc::c_char,
    b"S_MINECARTSEG_BACK\0" as *const u8 as *const libc::c_char,
    b"S_MINECARTSEG_LEFT\0" as *const u8 as *const libc::c_char,
    b"S_MINECARTSEG_RIGHT\0" as *const u8 as *const libc::c_char,
    b"S_MINECARTSIDEMARK1\0" as *const u8 as *const libc::c_char,
    b"S_MINECARTSIDEMARK2\0" as *const u8 as *const libc::c_char,
    b"S_MINECARTSPARK\0" as *const u8 as *const libc::c_char,
    b"S_SALOONDOOR\0" as *const u8 as *const libc::c_char,
    b"S_SALOONDOORCENTER\0" as *const u8 as *const libc::c_char,
    b"S_TRAINCAMEOSPAWNER_1\0" as *const u8 as *const libc::c_char,
    b"S_TRAINCAMEOSPAWNER_2\0" as *const u8 as *const libc::c_char,
    b"S_TRAINCAMEOSPAWNER_3\0" as *const u8 as *const libc::c_char,
    b"S_TRAINCAMEOSPAWNER_4\0" as *const u8 as *const libc::c_char,
    b"S_TRAINCAMEOSPAWNER_5\0" as *const u8 as *const libc::c_char,
    b"S_TRAINPUFFMAKER\0" as *const u8 as *const libc::c_char,
    b"S_TRAINDUST\0" as *const u8 as *const libc::c_char,
    b"S_TRAINSTEAM\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEJETSTND\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEJETSTART\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEJETSTOP\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEJETFLAME1\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEJETFLAME2\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEJETFLAME3\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEJETFLAME4\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEJETFLAME5\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEJETFLAME6\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEJETFLAME7\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEJETFLAME8\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEJETFLAME9\0" as *const u8 as *const libc::c_char,
    b"S_FJSPINAXISA1\0" as *const u8 as *const libc::c_char,
    b"S_FJSPINAXISA2\0" as *const u8 as *const libc::c_char,
    b"S_FJSPINAXISB1\0" as *const u8 as *const libc::c_char,
    b"S_FJSPINAXISB2\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEJETFLAMEB1\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEJETFLAMEB2\0" as *const u8 as *const libc::c_char,
    b"S_FLAMEJETFLAMEB3\0" as *const u8 as *const libc::c_char,
    b"S_LAVAFALL_DORMANT\0" as *const u8 as *const libc::c_char,
    b"S_LAVAFALL_TELL\0" as *const u8 as *const libc::c_char,
    b"S_LAVAFALL_SHOOT\0" as *const u8 as *const libc::c_char,
    b"S_LAVAFALL_LAVA1\0" as *const u8 as *const libc::c_char,
    b"S_LAVAFALL_LAVA2\0" as *const u8 as *const libc::c_char,
    b"S_LAVAFALL_LAVA3\0" as *const u8 as *const libc::c_char,
    b"S_LAVAFALLROCK\0" as *const u8 as *const libc::c_char,
    b"S_ROLLOUTSPAWN\0" as *const u8 as *const libc::c_char,
    b"S_ROLLOUTROCK\0" as *const u8 as *const libc::c_char,
    b"S_BIGFERNLEAF\0" as *const u8 as *const libc::c_char,
    b"S_BIGFERN1\0" as *const u8 as *const libc::c_char,
    b"S_BIGFERN2\0" as *const u8 as *const libc::c_char,
    b"S_JUNGLEPALM\0" as *const u8 as *const libc::c_char,
    b"S_TORCHFLOWER\0" as *const u8 as *const libc::c_char,
    b"S_WALLVINE_LONG\0" as *const u8 as *const libc::c_char,
    b"S_WALLVINE_SHORT\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLE\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLE_CHARGE\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLE_BLINK\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLE_HOLD\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLE_FIRE\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLE_LOOP\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLE_COOLDOWN\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLEUP\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLEUP_CHARGE\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLEUP_BLINK\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLEUP_HOLD\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLEUP_FIRE\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLEUP_LOOP\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLEUP_COOLDOWN\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLEDOWN\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLEDOWN_CHARGE\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLEDOWN_BLINK\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLEDOWN_HOLD\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLEDOWN_FIRE\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLEDOWN_LOOP\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLEDOWN_COOLDOWN\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLELONG\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLELONG_CHARGE\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLELONG_BLINK\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLELONG_HOLD\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLELONG_FIRE\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLELONG_LOOP\0" as *const u8 as *const libc::c_char,
    b"S_GLAREGOYLELONG_COOLDOWN\0" as *const u8 as *const libc::c_char,
    b"S_TARGET_IDLE\0" as *const u8 as *const libc::c_char,
    b"S_TARGET_HIT1\0" as *const u8 as *const libc::c_char,
    b"S_TARGET_HIT2\0" as *const u8 as *const libc::c_char,
    b"S_TARGET_RESPAWN\0" as *const u8 as *const libc::c_char,
    b"S_TARGET_ALLDONE\0" as *const u8 as *const libc::c_char,
    b"S_GREENFLAME\0" as *const u8 as *const libc::c_char,
    b"S_BLUEGARGOYLE\0" as *const u8 as *const libc::c_char,
    b"S_STG0\0" as *const u8 as *const libc::c_char,
    b"S_STG1\0" as *const u8 as *const libc::c_char,
    b"S_STG2\0" as *const u8 as *const libc::c_char,
    b"S_STG3\0" as *const u8 as *const libc::c_char,
    b"S_STG4\0" as *const u8 as *const libc::c_char,
    b"S_STG5\0" as *const u8 as *const libc::c_char,
    b"S_STG6\0" as *const u8 as *const libc::c_char,
    b"S_STG7\0" as *const u8 as *const libc::c_char,
    b"S_STG8\0" as *const u8 as *const libc::c_char,
    b"S_STG9\0" as *const u8 as *const libc::c_char,
    b"S_XMASPOLE\0" as *const u8 as *const libc::c_char,
    b"S_CANDYCANE\0" as *const u8 as *const libc::c_char,
    b"S_SNOWMAN\0" as *const u8 as *const libc::c_char,
    b"S_SNOWMANHAT\0" as *const u8 as *const libc::c_char,
    b"S_LAMPPOST1\0" as *const u8 as *const libc::c_char,
    b"S_LAMPPOST2\0" as *const u8 as *const libc::c_char,
    b"S_HANGSTAR\0" as *const u8 as *const libc::c_char,
    b"S_MISTLETOE\0" as *const u8 as *const libc::c_char,
    b"S_XMASBLUEBERRYBUSH\0" as *const u8 as *const libc::c_char,
    b"S_XMASBERRYBUSH\0" as *const u8 as *const libc::c_char,
    b"S_XMASBUSH\0" as *const u8 as *const libc::c_char,
    b"S_FHZICE1\0" as *const u8 as *const libc::c_char,
    b"S_FHZICE2\0" as *const u8 as *const libc::c_char,
    b"S_ROSY_IDLE1\0" as *const u8 as *const libc::c_char,
    b"S_ROSY_IDLE2\0" as *const u8 as *const libc::c_char,
    b"S_ROSY_IDLE3\0" as *const u8 as *const libc::c_char,
    b"S_ROSY_IDLE4\0" as *const u8 as *const libc::c_char,
    b"S_ROSY_JUMP\0" as *const u8 as *const libc::c_char,
    b"S_ROSY_WALK\0" as *const u8 as *const libc::c_char,
    b"S_ROSY_HUG\0" as *const u8 as *const libc::c_char,
    b"S_ROSY_PAIN\0" as *const u8 as *const libc::c_char,
    b"S_ROSY_STND\0" as *const u8 as *const libc::c_char,
    b"S_ROSY_UNHAPPY\0" as *const u8 as *const libc::c_char,
    b"S_JACKO1\0" as *const u8 as *const libc::c_char,
    b"S_JACKO1OVERLAY_1\0" as *const u8 as *const libc::c_char,
    b"S_JACKO1OVERLAY_2\0" as *const u8 as *const libc::c_char,
    b"S_JACKO1OVERLAY_3\0" as *const u8 as *const libc::c_char,
    b"S_JACKO1OVERLAY_4\0" as *const u8 as *const libc::c_char,
    b"S_JACKO2\0" as *const u8 as *const libc::c_char,
    b"S_JACKO2OVERLAY_1\0" as *const u8 as *const libc::c_char,
    b"S_JACKO2OVERLAY_2\0" as *const u8 as *const libc::c_char,
    b"S_JACKO2OVERLAY_3\0" as *const u8 as *const libc::c_char,
    b"S_JACKO2OVERLAY_4\0" as *const u8 as *const libc::c_char,
    b"S_JACKO3\0" as *const u8 as *const libc::c_char,
    b"S_JACKO3OVERLAY_1\0" as *const u8 as *const libc::c_char,
    b"S_JACKO3OVERLAY_2\0" as *const u8 as *const libc::c_char,
    b"S_JACKO3OVERLAY_3\0" as *const u8 as *const libc::c_char,
    b"S_JACKO3OVERLAY_4\0" as *const u8 as *const libc::c_char,
    b"S_HHZTREE_TOP\0" as *const u8 as *const libc::c_char,
    b"S_HHZTREE_TRUNK\0" as *const u8 as *const libc::c_char,
    b"S_HHZTREE_LEAF\0" as *const u8 as *const libc::c_char,
    b"S_HHZSHROOM_1\0" as *const u8 as *const libc::c_char,
    b"S_HHZSHROOM_2\0" as *const u8 as *const libc::c_char,
    b"S_HHZSHROOM_3\0" as *const u8 as *const libc::c_char,
    b"S_HHZSHROOM_4\0" as *const u8 as *const libc::c_char,
    b"S_HHZSHROOM_5\0" as *const u8 as *const libc::c_char,
    b"S_HHZSHROOM_6\0" as *const u8 as *const libc::c_char,
    b"S_HHZSHROOM_7\0" as *const u8 as *const libc::c_char,
    b"S_HHZSHROOM_8\0" as *const u8 as *const libc::c_char,
    b"S_HHZSHROOM_9\0" as *const u8 as *const libc::c_char,
    b"S_HHZSHROOM_10\0" as *const u8 as *const libc::c_char,
    b"S_HHZSHROOM_11\0" as *const u8 as *const libc::c_char,
    b"S_HHZSHROOM_12\0" as *const u8 as *const libc::c_char,
    b"S_HHZSHROOM_13\0" as *const u8 as *const libc::c_char,
    b"S_HHZSHROOM_14\0" as *const u8 as *const libc::c_char,
    b"S_HHZSHROOM_15\0" as *const u8 as *const libc::c_char,
    b"S_HHZSHROOM_16\0" as *const u8 as *const libc::c_char,
    b"S_HHZGRASS\0" as *const u8 as *const libc::c_char,
    b"S_HHZTENT1\0" as *const u8 as *const libc::c_char,
    b"S_HHZTENT2\0" as *const u8 as *const libc::c_char,
    b"S_HHZSTALAGMITE_TALL\0" as *const u8 as *const libc::c_char,
    b"S_HHZSTALAGMITE_SHORT\0" as *const u8 as *const libc::c_char,
    b"S_BSZTALLFLOWER_RED\0" as *const u8 as *const libc::c_char,
    b"S_BSZTALLFLOWER_PURPLE\0" as *const u8 as *const libc::c_char,
    b"S_BSZTALLFLOWER_BLUE\0" as *const u8 as *const libc::c_char,
    b"S_BSZTALLFLOWER_CYAN\0" as *const u8 as *const libc::c_char,
    b"S_BSZTALLFLOWER_YELLOW\0" as *const u8 as *const libc::c_char,
    b"S_BSZTALLFLOWER_ORANGE\0" as *const u8 as *const libc::c_char,
    b"S_BSZFLOWER_RED\0" as *const u8 as *const libc::c_char,
    b"S_BSZFLOWER_PURPLE\0" as *const u8 as *const libc::c_char,
    b"S_BSZFLOWER_BLUE\0" as *const u8 as *const libc::c_char,
    b"S_BSZFLOWER_CYAN\0" as *const u8 as *const libc::c_char,
    b"S_BSZFLOWER_YELLOW\0" as *const u8 as *const libc::c_char,
    b"S_BSZFLOWER_ORANGE\0" as *const u8 as *const libc::c_char,
    b"S_BSZSHORTFLOWER_RED\0" as *const u8 as *const libc::c_char,
    b"S_BSZSHORTFLOWER_PURPLE\0" as *const u8 as *const libc::c_char,
    b"S_BSZSHORTFLOWER_BLUE\0" as *const u8 as *const libc::c_char,
    b"S_BSZSHORTFLOWER_CYAN\0" as *const u8 as *const libc::c_char,
    b"S_BSZSHORTFLOWER_YELLOW\0" as *const u8 as *const libc::c_char,
    b"S_BSZSHORTFLOWER_ORANGE\0" as *const u8 as *const libc::c_char,
    b"S_BSZTULIP_RED\0" as *const u8 as *const libc::c_char,
    b"S_BSZTULIP_PURPLE\0" as *const u8 as *const libc::c_char,
    b"S_BSZTULIP_BLUE\0" as *const u8 as *const libc::c_char,
    b"S_BSZTULIP_CYAN\0" as *const u8 as *const libc::c_char,
    b"S_BSZTULIP_YELLOW\0" as *const u8 as *const libc::c_char,
    b"S_BSZTULIP_ORANGE\0" as *const u8 as *const libc::c_char,
    b"S_BSZCLUSTER_RED\0" as *const u8 as *const libc::c_char,
    b"S_BSZCLUSTER_PURPLE\0" as *const u8 as *const libc::c_char,
    b"S_BSZCLUSTER_BLUE\0" as *const u8 as *const libc::c_char,
    b"S_BSZCLUSTER_CYAN\0" as *const u8 as *const libc::c_char,
    b"S_BSZCLUSTER_YELLOW\0" as *const u8 as *const libc::c_char,
    b"S_BSZCLUSTER_ORANGE\0" as *const u8 as *const libc::c_char,
    b"S_BSZBUSH_RED\0" as *const u8 as *const libc::c_char,
    b"S_BSZBUSH_PURPLE\0" as *const u8 as *const libc::c_char,
    b"S_BSZBUSH_BLUE\0" as *const u8 as *const libc::c_char,
    b"S_BSZBUSH_CYAN\0" as *const u8 as *const libc::c_char,
    b"S_BSZBUSH_YELLOW\0" as *const u8 as *const libc::c_char,
    b"S_BSZBUSH_ORANGE\0" as *const u8 as *const libc::c_char,
    b"S_BSZVINE_RED\0" as *const u8 as *const libc::c_char,
    b"S_BSZVINE_PURPLE\0" as *const u8 as *const libc::c_char,
    b"S_BSZVINE_BLUE\0" as *const u8 as *const libc::c_char,
    b"S_BSZVINE_CYAN\0" as *const u8 as *const libc::c_char,
    b"S_BSZVINE_YELLOW\0" as *const u8 as *const libc::c_char,
    b"S_BSZVINE_ORANGE\0" as *const u8 as *const libc::c_char,
    b"S_BSZSHRUB\0" as *const u8 as *const libc::c_char,
    b"S_BSZCLOVER\0" as *const u8 as *const libc::c_char,
    b"S_BIG_PALMTREE_TRUNK\0" as *const u8 as *const libc::c_char,
    b"S_BIG_PALMTREE_TOP\0" as *const u8 as *const libc::c_char,
    b"S_PALMTREE_TRUNK\0" as *const u8 as *const libc::c_char,
    b"S_PALMTREE_TOP\0" as *const u8 as *const libc::c_char,
    b"S_DBALL1\0" as *const u8 as *const libc::c_char,
    b"S_DBALL2\0" as *const u8 as *const libc::c_char,
    b"S_DBALL3\0" as *const u8 as *const libc::c_char,
    b"S_DBALL4\0" as *const u8 as *const libc::c_char,
    b"S_DBALL5\0" as *const u8 as *const libc::c_char,
    b"S_DBALL6\0" as *const u8 as *const libc::c_char,
    b"S_EGGSTATUE2\0" as *const u8 as *const libc::c_char,
    b"S_ARMA1\0" as *const u8 as *const libc::c_char,
    b"S_ARMA2\0" as *const u8 as *const libc::c_char,
    b"S_ARMA3\0" as *const u8 as *const libc::c_char,
    b"S_ARMA4\0" as *const u8 as *const libc::c_char,
    b"S_ARMA5\0" as *const u8 as *const libc::c_char,
    b"S_ARMA6\0" as *const u8 as *const libc::c_char,
    b"S_ARMA7\0" as *const u8 as *const libc::c_char,
    b"S_ARMA8\0" as *const u8 as *const libc::c_char,
    b"S_ARMA9\0" as *const u8 as *const libc::c_char,
    b"S_ARMA10\0" as *const u8 as *const libc::c_char,
    b"S_ARMA11\0" as *const u8 as *const libc::c_char,
    b"S_ARMA12\0" as *const u8 as *const libc::c_char,
    b"S_ARMA13\0" as *const u8 as *const libc::c_char,
    b"S_ARMA14\0" as *const u8 as *const libc::c_char,
    b"S_ARMA15\0" as *const u8 as *const libc::c_char,
    b"S_ARMA16\0" as *const u8 as *const libc::c_char,
    b"S_ARMF1\0" as *const u8 as *const libc::c_char,
    b"S_ARMF2\0" as *const u8 as *const libc::c_char,
    b"S_ARMF3\0" as *const u8 as *const libc::c_char,
    b"S_ARMF4\0" as *const u8 as *const libc::c_char,
    b"S_ARMF5\0" as *const u8 as *const libc::c_char,
    b"S_ARMF6\0" as *const u8 as *const libc::c_char,
    b"S_ARMF7\0" as *const u8 as *const libc::c_char,
    b"S_ARMF8\0" as *const u8 as *const libc::c_char,
    b"S_ARMF9\0" as *const u8 as *const libc::c_char,
    b"S_ARMF10\0" as *const u8 as *const libc::c_char,
    b"S_ARMF11\0" as *const u8 as *const libc::c_char,
    b"S_ARMF12\0" as *const u8 as *const libc::c_char,
    b"S_ARMF13\0" as *const u8 as *const libc::c_char,
    b"S_ARMF14\0" as *const u8 as *const libc::c_char,
    b"S_ARMF15\0" as *const u8 as *const libc::c_char,
    b"S_ARMF16\0" as *const u8 as *const libc::c_char,
    b"S_ARMF17\0" as *const u8 as *const libc::c_char,
    b"S_ARMF18\0" as *const u8 as *const libc::c_char,
    b"S_ARMF19\0" as *const u8 as *const libc::c_char,
    b"S_ARMF20\0" as *const u8 as *const libc::c_char,
    b"S_ARMF21\0" as *const u8 as *const libc::c_char,
    b"S_ARMF22\0" as *const u8 as *const libc::c_char,
    b"S_ARMF23\0" as *const u8 as *const libc::c_char,
    b"S_ARMF24\0" as *const u8 as *const libc::c_char,
    b"S_ARMF25\0" as *const u8 as *const libc::c_char,
    b"S_ARMF26\0" as *const u8 as *const libc::c_char,
    b"S_ARMF27\0" as *const u8 as *const libc::c_char,
    b"S_ARMF28\0" as *const u8 as *const libc::c_char,
    b"S_ARMF29\0" as *const u8 as *const libc::c_char,
    b"S_ARMF30\0" as *const u8 as *const libc::c_char,
    b"S_ARMF31\0" as *const u8 as *const libc::c_char,
    b"S_ARMF32\0" as *const u8 as *const libc::c_char,
    b"S_ARMB1\0" as *const u8 as *const libc::c_char,
    b"S_ARMB2\0" as *const u8 as *const libc::c_char,
    b"S_ARMB3\0" as *const u8 as *const libc::c_char,
    b"S_ARMB4\0" as *const u8 as *const libc::c_char,
    b"S_ARMB5\0" as *const u8 as *const libc::c_char,
    b"S_ARMB6\0" as *const u8 as *const libc::c_char,
    b"S_ARMB7\0" as *const u8 as *const libc::c_char,
    b"S_ARMB8\0" as *const u8 as *const libc::c_char,
    b"S_ARMB9\0" as *const u8 as *const libc::c_char,
    b"S_ARMB10\0" as *const u8 as *const libc::c_char,
    b"S_ARMB11\0" as *const u8 as *const libc::c_char,
    b"S_ARMB12\0" as *const u8 as *const libc::c_char,
    b"S_ARMB13\0" as *const u8 as *const libc::c_char,
    b"S_ARMB14\0" as *const u8 as *const libc::c_char,
    b"S_ARMB15\0" as *const u8 as *const libc::c_char,
    b"S_ARMB16\0" as *const u8 as *const libc::c_char,
    b"S_ARMB17\0" as *const u8 as *const libc::c_char,
    b"S_ARMB18\0" as *const u8 as *const libc::c_char,
    b"S_ARMB19\0" as *const u8 as *const libc::c_char,
    b"S_ARMB20\0" as *const u8 as *const libc::c_char,
    b"S_ARMB21\0" as *const u8 as *const libc::c_char,
    b"S_ARMB22\0" as *const u8 as *const libc::c_char,
    b"S_ARMB23\0" as *const u8 as *const libc::c_char,
    b"S_ARMB24\0" as *const u8 as *const libc::c_char,
    b"S_ARMB25\0" as *const u8 as *const libc::c_char,
    b"S_ARMB26\0" as *const u8 as *const libc::c_char,
    b"S_ARMB27\0" as *const u8 as *const libc::c_char,
    b"S_ARMB28\0" as *const u8 as *const libc::c_char,
    b"S_ARMB29\0" as *const u8 as *const libc::c_char,
    b"S_ARMB30\0" as *const u8 as *const libc::c_char,
    b"S_ARMB31\0" as *const u8 as *const libc::c_char,
    b"S_ARMB32\0" as *const u8 as *const libc::c_char,
    b"S_WIND1\0" as *const u8 as *const libc::c_char,
    b"S_WIND2\0" as *const u8 as *const libc::c_char,
    b"S_WIND3\0" as *const u8 as *const libc::c_char,
    b"S_WIND4\0" as *const u8 as *const libc::c_char,
    b"S_WIND5\0" as *const u8 as *const libc::c_char,
    b"S_WIND6\0" as *const u8 as *const libc::c_char,
    b"S_WIND7\0" as *const u8 as *const libc::c_char,
    b"S_WIND8\0" as *const u8 as *const libc::c_char,
    b"S_MAGN1\0" as *const u8 as *const libc::c_char,
    b"S_MAGN2\0" as *const u8 as *const libc::c_char,
    b"S_MAGN3\0" as *const u8 as *const libc::c_char,
    b"S_MAGN4\0" as *const u8 as *const libc::c_char,
    b"S_MAGN5\0" as *const u8 as *const libc::c_char,
    b"S_MAGN6\0" as *const u8 as *const libc::c_char,
    b"S_MAGN7\0" as *const u8 as *const libc::c_char,
    b"S_MAGN8\0" as *const u8 as *const libc::c_char,
    b"S_MAGN9\0" as *const u8 as *const libc::c_char,
    b"S_MAGN10\0" as *const u8 as *const libc::c_char,
    b"S_MAGN11\0" as *const u8 as *const libc::c_char,
    b"S_MAGN12\0" as *const u8 as *const libc::c_char,
    b"S_MAGN13\0" as *const u8 as *const libc::c_char,
    b"S_FORC1\0" as *const u8 as *const libc::c_char,
    b"S_FORC2\0" as *const u8 as *const libc::c_char,
    b"S_FORC3\0" as *const u8 as *const libc::c_char,
    b"S_FORC4\0" as *const u8 as *const libc::c_char,
    b"S_FORC5\0" as *const u8 as *const libc::c_char,
    b"S_FORC6\0" as *const u8 as *const libc::c_char,
    b"S_FORC7\0" as *const u8 as *const libc::c_char,
    b"S_FORC8\0" as *const u8 as *const libc::c_char,
    b"S_FORC9\0" as *const u8 as *const libc::c_char,
    b"S_FORC10\0" as *const u8 as *const libc::c_char,
    b"S_FORC11\0" as *const u8 as *const libc::c_char,
    b"S_FORC12\0" as *const u8 as *const libc::c_char,
    b"S_FORC13\0" as *const u8 as *const libc::c_char,
    b"S_FORC14\0" as *const u8 as *const libc::c_char,
    b"S_FORC15\0" as *const u8 as *const libc::c_char,
    b"S_FORC16\0" as *const u8 as *const libc::c_char,
    b"S_FORC17\0" as *const u8 as *const libc::c_char,
    b"S_FORC18\0" as *const u8 as *const libc::c_char,
    b"S_FORC19\0" as *const u8 as *const libc::c_char,
    b"S_FORC20\0" as *const u8 as *const libc::c_char,
    b"S_FORC21\0" as *const u8 as *const libc::c_char,
    b"S_ELEM1\0" as *const u8 as *const libc::c_char,
    b"S_ELEM2\0" as *const u8 as *const libc::c_char,
    b"S_ELEM3\0" as *const u8 as *const libc::c_char,
    b"S_ELEM4\0" as *const u8 as *const libc::c_char,
    b"S_ELEM5\0" as *const u8 as *const libc::c_char,
    b"S_ELEM6\0" as *const u8 as *const libc::c_char,
    b"S_ELEM7\0" as *const u8 as *const libc::c_char,
    b"S_ELEM8\0" as *const u8 as *const libc::c_char,
    b"S_ELEM9\0" as *const u8 as *const libc::c_char,
    b"S_ELEM10\0" as *const u8 as *const libc::c_char,
    b"S_ELEM11\0" as *const u8 as *const libc::c_char,
    b"S_ELEM12\0" as *const u8 as *const libc::c_char,
    b"S_ELEM13\0" as *const u8 as *const libc::c_char,
    b"S_ELEM14\0" as *const u8 as *const libc::c_char,
    b"S_ELEMF1\0" as *const u8 as *const libc::c_char,
    b"S_ELEMF2\0" as *const u8 as *const libc::c_char,
    b"S_ELEMF3\0" as *const u8 as *const libc::c_char,
    b"S_ELEMF4\0" as *const u8 as *const libc::c_char,
    b"S_ELEMF5\0" as *const u8 as *const libc::c_char,
    b"S_ELEMF6\0" as *const u8 as *const libc::c_char,
    b"S_ELEMF7\0" as *const u8 as *const libc::c_char,
    b"S_ELEMF8\0" as *const u8 as *const libc::c_char,
    b"S_ELEMF9\0" as *const u8 as *const libc::c_char,
    b"S_ELEMF10\0" as *const u8 as *const libc::c_char,
    b"S_PITY1\0" as *const u8 as *const libc::c_char,
    b"S_PITY2\0" as *const u8 as *const libc::c_char,
    b"S_PITY3\0" as *const u8 as *const libc::c_char,
    b"S_PITY4\0" as *const u8 as *const libc::c_char,
    b"S_PITY5\0" as *const u8 as *const libc::c_char,
    b"S_PITY6\0" as *const u8 as *const libc::c_char,
    b"S_PITY7\0" as *const u8 as *const libc::c_char,
    b"S_PITY8\0" as *const u8 as *const libc::c_char,
    b"S_PITY9\0" as *const u8 as *const libc::c_char,
    b"S_PITY10\0" as *const u8 as *const libc::c_char,
    b"S_PITY11\0" as *const u8 as *const libc::c_char,
    b"S_PITY12\0" as *const u8 as *const libc::c_char,
    b"S_FIRS1\0" as *const u8 as *const libc::c_char,
    b"S_FIRS2\0" as *const u8 as *const libc::c_char,
    b"S_FIRS3\0" as *const u8 as *const libc::c_char,
    b"S_FIRS4\0" as *const u8 as *const libc::c_char,
    b"S_FIRS5\0" as *const u8 as *const libc::c_char,
    b"S_FIRS6\0" as *const u8 as *const libc::c_char,
    b"S_FIRS7\0" as *const u8 as *const libc::c_char,
    b"S_FIRS8\0" as *const u8 as *const libc::c_char,
    b"S_FIRS9\0" as *const u8 as *const libc::c_char,
    b"S_FIRS10\0" as *const u8 as *const libc::c_char,
    b"S_FIRS11\0" as *const u8 as *const libc::c_char,
    b"S_FIRSB1\0" as *const u8 as *const libc::c_char,
    b"S_FIRSB2\0" as *const u8 as *const libc::c_char,
    b"S_FIRSB3\0" as *const u8 as *const libc::c_char,
    b"S_FIRSB4\0" as *const u8 as *const libc::c_char,
    b"S_FIRSB5\0" as *const u8 as *const libc::c_char,
    b"S_FIRSB6\0" as *const u8 as *const libc::c_char,
    b"S_FIRSB7\0" as *const u8 as *const libc::c_char,
    b"S_FIRSB8\0" as *const u8 as *const libc::c_char,
    b"S_FIRSB9\0" as *const u8 as *const libc::c_char,
    b"S_FIRSB10\0" as *const u8 as *const libc::c_char,
    b"S_BUBS1\0" as *const u8 as *const libc::c_char,
    b"S_BUBS2\0" as *const u8 as *const libc::c_char,
    b"S_BUBS3\0" as *const u8 as *const libc::c_char,
    b"S_BUBS4\0" as *const u8 as *const libc::c_char,
    b"S_BUBS5\0" as *const u8 as *const libc::c_char,
    b"S_BUBS6\0" as *const u8 as *const libc::c_char,
    b"S_BUBS7\0" as *const u8 as *const libc::c_char,
    b"S_BUBS8\0" as *const u8 as *const libc::c_char,
    b"S_BUBS9\0" as *const u8 as *const libc::c_char,
    b"S_BUBS10\0" as *const u8 as *const libc::c_char,
    b"S_BUBS11\0" as *const u8 as *const libc::c_char,
    b"S_BUBSB1\0" as *const u8 as *const libc::c_char,
    b"S_BUBSB2\0" as *const u8 as *const libc::c_char,
    b"S_BUBSB3\0" as *const u8 as *const libc::c_char,
    b"S_BUBSB4\0" as *const u8 as *const libc::c_char,
    b"S_BUBSB5\0" as *const u8 as *const libc::c_char,
    b"S_BUBSB6\0" as *const u8 as *const libc::c_char,
    b"S_ZAPS1\0" as *const u8 as *const libc::c_char,
    b"S_ZAPS2\0" as *const u8 as *const libc::c_char,
    b"S_ZAPS3\0" as *const u8 as *const libc::c_char,
    b"S_ZAPS4\0" as *const u8 as *const libc::c_char,
    b"S_ZAPS5\0" as *const u8 as *const libc::c_char,
    b"S_ZAPS6\0" as *const u8 as *const libc::c_char,
    b"S_ZAPS7\0" as *const u8 as *const libc::c_char,
    b"S_ZAPS8\0" as *const u8 as *const libc::c_char,
    b"S_ZAPS9\0" as *const u8 as *const libc::c_char,
    b"S_ZAPS10\0" as *const u8 as *const libc::c_char,
    b"S_ZAPS11\0" as *const u8 as *const libc::c_char,
    b"S_ZAPS12\0" as *const u8 as *const libc::c_char,
    b"S_ZAPS13\0" as *const u8 as *const libc::c_char,
    b"S_ZAPS14\0" as *const u8 as *const libc::c_char,
    b"S_ZAPS15\0" as *const u8 as *const libc::c_char,
    b"S_ZAPS16\0" as *const u8 as *const libc::c_char,
    b"S_ZAPSB1\0" as *const u8 as *const libc::c_char,
    b"S_ZAPSB2\0" as *const u8 as *const libc::c_char,
    b"S_ZAPSB3\0" as *const u8 as *const libc::c_char,
    b"S_ZAPSB4\0" as *const u8 as *const libc::c_char,
    b"S_ZAPSB5\0" as *const u8 as *const libc::c_char,
    b"S_ZAPSB6\0" as *const u8 as *const libc::c_char,
    b"S_ZAPSB7\0" as *const u8 as *const libc::c_char,
    b"S_ZAPSB8\0" as *const u8 as *const libc::c_char,
    b"S_ZAPSB9\0" as *const u8 as *const libc::c_char,
    b"S_ZAPSB10\0" as *const u8 as *const libc::c_char,
    b"S_ZAPSB11\0" as *const u8 as *const libc::c_char,
    b"S_THUNDERCOIN_SPARK\0" as *const u8 as *const libc::c_char,
    b"S_IVSP\0" as *const u8 as *const libc::c_char,
    b"S_SSPK1\0" as *const u8 as *const libc::c_char,
    b"S_SSPK2\0" as *const u8 as *const libc::c_char,
    b"S_SSPK3\0" as *const u8 as *const libc::c_char,
    b"S_SSPK4\0" as *const u8 as *const libc::c_char,
    b"S_SSPK5\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_BUBBLE\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_01_OUT\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_01_FLAP1\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_01_FLAP2\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_01_FLAP3\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_01_STAND\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_01_CENTER\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_02_OUT\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_02_AIM\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_02_HOP\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_02_UP\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_02_DOWN\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_02_STAND\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_02_CENTER\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_03_OUT\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_03_AIM\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_03_HOP\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_03_UP\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_03_FLAP1\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_03_FLAP2\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_03_STAND\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_03_CENTER\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_04_OUT\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_04_AIM\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_04_HOP\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_04_UP\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_04_DOWN\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_04_SWIM1\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_04_SWIM2\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_04_SWIM3\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_04_SWIM4\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_04_STAND\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_04_CENTER\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_05_OUT\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_05_AIM\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_05_HOP\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_05_UP\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_05_DOWN\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_05_STAND\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_05_CENTER\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_06_OUT\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_06_AIM\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_06_HOP\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_06_UP\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_06_DOWN\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_06_STAND\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_06_CENTER\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_07_OUT\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_07_AIML\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_07_HOPL\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_07_UPL\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_07_DOWNL\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_07_AIMR\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_07_HOPR\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_07_UPR\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_07_DOWNR\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_07_SWIM1\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_07_SWIM2\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_07_SWIM3\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_07_STAND\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_07_CENTER\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_08_OUT\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_08_AIM\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_08_HOP\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_08_FLAP1\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_08_FLAP2\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_08_FLAP3\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_08_FLAP4\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_08_SWIM1\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_08_SWIM2\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_08_SWIM3\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_08_SWIM4\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_08_STAND\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_08_CENTER\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_09_OUT\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_09_AIM\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_09_HOP\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_09_UP\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_09_DOWN\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_09_STAND\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_09_CENTER\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_10_OUT\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_10_FLAP1\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_10_FLAP2\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_10_STAND\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_10_CENTER\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_11_OUT\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_11_AIM\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_11_RUN1\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_11_RUN2\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_11_RUN3\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_11_STAND\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_11_CENTER\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_12_OUT\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_12_AIM\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_12_RUN1\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_12_RUN2\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_12_RUN3\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_12_STAND\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_12_CENTER\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_13_OUT\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_13_AIM\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_13_HOP\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_13_UP\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_13_DOWN\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_13_STAND\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_13_CENTER\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_14_OUT\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_14_FLAP1\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_14_FLAP2\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_14_FLAP3\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_14_STAND\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_14_CENTER\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_15_OUT\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_15_AIM\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_15_HOP\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_15_UP\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_15_DOWN\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_15_STAND\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_15_CENTER\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_16_OUT\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_16_FLAP1\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_16_FLAP2\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_16_FLAP3\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_16_STAND\0" as *const u8 as *const libc::c_char,
    b"S_FLICKY_16_CENTER\0" as *const u8 as *const libc::c_char,
    b"S_SECRETFLICKY_01_OUT\0" as *const u8 as *const libc::c_char,
    b"S_SECRETFLICKY_01_AIM\0" as *const u8 as *const libc::c_char,
    b"S_SECRETFLICKY_01_HOP\0" as *const u8 as *const libc::c_char,
    b"S_SECRETFLICKY_01_UP\0" as *const u8 as *const libc::c_char,
    b"S_SECRETFLICKY_01_DOWN\0" as *const u8 as *const libc::c_char,
    b"S_SECRETFLICKY_01_STAND\0" as *const u8 as *const libc::c_char,
    b"S_SECRETFLICKY_01_CENTER\0" as *const u8 as *const libc::c_char,
    b"S_SECRETFLICKY_02_OUT\0" as *const u8 as *const libc::c_char,
    b"S_SECRETFLICKY_02_FLAP1\0" as *const u8 as *const libc::c_char,
    b"S_SECRETFLICKY_02_FLAP2\0" as *const u8 as *const libc::c_char,
    b"S_SECRETFLICKY_02_FLAP3\0" as *const u8 as *const libc::c_char,
    b"S_SECRETFLICKY_02_STAND\0" as *const u8 as *const libc::c_char,
    b"S_SECRETFLICKY_02_CENTER\0" as *const u8 as *const libc::c_char,
    b"S_FAN\0" as *const u8 as *const libc::c_char,
    b"S_FAN2\0" as *const u8 as *const libc::c_char,
    b"S_FAN3\0" as *const u8 as *const libc::c_char,
    b"S_FAN4\0" as *const u8 as *const libc::c_char,
    b"S_FAN5\0" as *const u8 as *const libc::c_char,
    b"S_STEAM1\0" as *const u8 as *const libc::c_char,
    b"S_STEAM2\0" as *const u8 as *const libc::c_char,
    b"S_STEAM3\0" as *const u8 as *const libc::c_char,
    b"S_STEAM4\0" as *const u8 as *const libc::c_char,
    b"S_STEAM5\0" as *const u8 as *const libc::c_char,
    b"S_STEAM6\0" as *const u8 as *const libc::c_char,
    b"S_STEAM7\0" as *const u8 as *const libc::c_char,
    b"S_STEAM8\0" as *const u8 as *const libc::c_char,
    b"S_BUMPER\0" as *const u8 as *const libc::c_char,
    b"S_BUMPERHIT\0" as *const u8 as *const libc::c_char,
    b"S_BALLOON\0" as *const u8 as *const libc::c_char,
    b"S_BALLOONPOP1\0" as *const u8 as *const libc::c_char,
    b"S_BALLOONPOP2\0" as *const u8 as *const libc::c_char,
    b"S_BALLOONPOP3\0" as *const u8 as *const libc::c_char,
    b"S_BALLOONPOP4\0" as *const u8 as *const libc::c_char,
    b"S_BALLOONPOP5\0" as *const u8 as *const libc::c_char,
    b"S_BALLOONPOP6\0" as *const u8 as *const libc::c_char,
    b"S_YELLOWSPRING\0" as *const u8 as *const libc::c_char,
    b"S_YELLOWSPRING2\0" as *const u8 as *const libc::c_char,
    b"S_YELLOWSPRING3\0" as *const u8 as *const libc::c_char,
    b"S_YELLOWSPRING4\0" as *const u8 as *const libc::c_char,
    b"S_YELLOWSPRING5\0" as *const u8 as *const libc::c_char,
    b"S_REDSPRING\0" as *const u8 as *const libc::c_char,
    b"S_REDSPRING2\0" as *const u8 as *const libc::c_char,
    b"S_REDSPRING3\0" as *const u8 as *const libc::c_char,
    b"S_REDSPRING4\0" as *const u8 as *const libc::c_char,
    b"S_REDSPRING5\0" as *const u8 as *const libc::c_char,
    b"S_BLUESPRING\0" as *const u8 as *const libc::c_char,
    b"S_BLUESPRING2\0" as *const u8 as *const libc::c_char,
    b"S_BLUESPRING3\0" as *const u8 as *const libc::c_char,
    b"S_BLUESPRING4\0" as *const u8 as *const libc::c_char,
    b"S_BLUESPRING5\0" as *const u8 as *const libc::c_char,
    b"S_YDIAG1\0" as *const u8 as *const libc::c_char,
    b"S_YDIAG2\0" as *const u8 as *const libc::c_char,
    b"S_YDIAG3\0" as *const u8 as *const libc::c_char,
    b"S_YDIAG4\0" as *const u8 as *const libc::c_char,
    b"S_YDIAG5\0" as *const u8 as *const libc::c_char,
    b"S_YDIAG6\0" as *const u8 as *const libc::c_char,
    b"S_YDIAG7\0" as *const u8 as *const libc::c_char,
    b"S_YDIAG8\0" as *const u8 as *const libc::c_char,
    b"S_RDIAG1\0" as *const u8 as *const libc::c_char,
    b"S_RDIAG2\0" as *const u8 as *const libc::c_char,
    b"S_RDIAG3\0" as *const u8 as *const libc::c_char,
    b"S_RDIAG4\0" as *const u8 as *const libc::c_char,
    b"S_RDIAG5\0" as *const u8 as *const libc::c_char,
    b"S_RDIAG6\0" as *const u8 as *const libc::c_char,
    b"S_RDIAG7\0" as *const u8 as *const libc::c_char,
    b"S_RDIAG8\0" as *const u8 as *const libc::c_char,
    b"S_BDIAG1\0" as *const u8 as *const libc::c_char,
    b"S_BDIAG2\0" as *const u8 as *const libc::c_char,
    b"S_BDIAG3\0" as *const u8 as *const libc::c_char,
    b"S_BDIAG4\0" as *const u8 as *const libc::c_char,
    b"S_BDIAG5\0" as *const u8 as *const libc::c_char,
    b"S_BDIAG6\0" as *const u8 as *const libc::c_char,
    b"S_BDIAG7\0" as *const u8 as *const libc::c_char,
    b"S_BDIAG8\0" as *const u8 as *const libc::c_char,
    b"S_YHORIZ1\0" as *const u8 as *const libc::c_char,
    b"S_YHORIZ2\0" as *const u8 as *const libc::c_char,
    b"S_YHORIZ3\0" as *const u8 as *const libc::c_char,
    b"S_YHORIZ4\0" as *const u8 as *const libc::c_char,
    b"S_YHORIZ5\0" as *const u8 as *const libc::c_char,
    b"S_YHORIZ6\0" as *const u8 as *const libc::c_char,
    b"S_YHORIZ7\0" as *const u8 as *const libc::c_char,
    b"S_YHORIZ8\0" as *const u8 as *const libc::c_char,
    b"S_RHORIZ1\0" as *const u8 as *const libc::c_char,
    b"S_RHORIZ2\0" as *const u8 as *const libc::c_char,
    b"S_RHORIZ3\0" as *const u8 as *const libc::c_char,
    b"S_RHORIZ4\0" as *const u8 as *const libc::c_char,
    b"S_RHORIZ5\0" as *const u8 as *const libc::c_char,
    b"S_RHORIZ6\0" as *const u8 as *const libc::c_char,
    b"S_RHORIZ7\0" as *const u8 as *const libc::c_char,
    b"S_RHORIZ8\0" as *const u8 as *const libc::c_char,
    b"S_BHORIZ1\0" as *const u8 as *const libc::c_char,
    b"S_BHORIZ2\0" as *const u8 as *const libc::c_char,
    b"S_BHORIZ3\0" as *const u8 as *const libc::c_char,
    b"S_BHORIZ4\0" as *const u8 as *const libc::c_char,
    b"S_BHORIZ5\0" as *const u8 as *const libc::c_char,
    b"S_BHORIZ6\0" as *const u8 as *const libc::c_char,
    b"S_BHORIZ7\0" as *const u8 as *const libc::c_char,
    b"S_BHORIZ8\0" as *const u8 as *const libc::c_char,
    b"S_BOOSTERSOUND\0" as *const u8 as *const libc::c_char,
    b"S_YELLOWBOOSTERROLLER\0" as *const u8 as *const libc::c_char,
    b"S_YELLOWBOOSTERSEG_LEFT\0" as *const u8 as *const libc::c_char,
    b"S_YELLOWBOOSTERSEG_RIGHT\0" as *const u8 as *const libc::c_char,
    b"S_YELLOWBOOSTERSEG_FACE\0" as *const u8 as *const libc::c_char,
    b"S_REDBOOSTERROLLER\0" as *const u8 as *const libc::c_char,
    b"S_REDBOOSTERSEG_LEFT\0" as *const u8 as *const libc::c_char,
    b"S_REDBOOSTERSEG_RIGHT\0" as *const u8 as *const libc::c_char,
    b"S_REDBOOSTERSEG_FACE\0" as *const u8 as *const libc::c_char,
    b"S_RAIN1\0" as *const u8 as *const libc::c_char,
    b"S_RAINRETURN\0" as *const u8 as *const libc::c_char,
    b"S_SNOW1\0" as *const u8 as *const libc::c_char,
    b"S_SNOW2\0" as *const u8 as *const libc::c_char,
    b"S_SNOW3\0" as *const u8 as *const libc::c_char,
    b"S_SPLISH1\0" as *const u8 as *const libc::c_char,
    b"S_SPLISH2\0" as *const u8 as *const libc::c_char,
    b"S_SPLISH3\0" as *const u8 as *const libc::c_char,
    b"S_SPLISH4\0" as *const u8 as *const libc::c_char,
    b"S_SPLISH5\0" as *const u8 as *const libc::c_char,
    b"S_SPLISH6\0" as *const u8 as *const libc::c_char,
    b"S_SPLISH7\0" as *const u8 as *const libc::c_char,
    b"S_SPLISH8\0" as *const u8 as *const libc::c_char,
    b"S_SPLISH9\0" as *const u8 as *const libc::c_char,
    b"S_LAVASPLISH\0" as *const u8 as *const libc::c_char,
    b"S_SPLASH1\0" as *const u8 as *const libc::c_char,
    b"S_SPLASH2\0" as *const u8 as *const libc::c_char,
    b"S_SPLASH3\0" as *const u8 as *const libc::c_char,
    b"S_SMOKE1\0" as *const u8 as *const libc::c_char,
    b"S_SMOKE2\0" as *const u8 as *const libc::c_char,
    b"S_SMOKE3\0" as *const u8 as *const libc::c_char,
    b"S_SMOKE4\0" as *const u8 as *const libc::c_char,
    b"S_SMOKE5\0" as *const u8 as *const libc::c_char,
    b"S_SMALLBUBBLE\0" as *const u8 as *const libc::c_char,
    b"S_MEDIUMBUBBLE\0" as *const u8 as *const libc::c_char,
    b"S_LARGEBUBBLE1\0" as *const u8 as *const libc::c_char,
    b"S_LARGEBUBBLE2\0" as *const u8 as *const libc::c_char,
    b"S_EXTRALARGEBUBBLE\0" as *const u8 as *const libc::c_char,
    b"S_POP1\0" as *const u8 as *const libc::c_char,
    b"S_WATERZAP\0" as *const u8 as *const libc::c_char,
    b"S_SPINDUST1\0" as *const u8 as *const libc::c_char,
    b"S_SPINDUST2\0" as *const u8 as *const libc::c_char,
    b"S_SPINDUST3\0" as *const u8 as *const libc::c_char,
    b"S_SPINDUST4\0" as *const u8 as *const libc::c_char,
    b"S_SPINDUST_BUBBLE1\0" as *const u8 as *const libc::c_char,
    b"S_SPINDUST_BUBBLE2\0" as *const u8 as *const libc::c_char,
    b"S_SPINDUST_BUBBLE3\0" as *const u8 as *const libc::c_char,
    b"S_SPINDUST_BUBBLE4\0" as *const u8 as *const libc::c_char,
    b"S_SPINDUST_FIRE1\0" as *const u8 as *const libc::c_char,
    b"S_SPINDUST_FIRE2\0" as *const u8 as *const libc::c_char,
    b"S_SPINDUST_FIRE3\0" as *const u8 as *const libc::c_char,
    b"S_SPINDUST_FIRE4\0" as *const u8 as *const libc::c_char,
    b"S_FOG1\0" as *const u8 as *const libc::c_char,
    b"S_FOG2\0" as *const u8 as *const libc::c_char,
    b"S_FOG3\0" as *const u8 as *const libc::c_char,
    b"S_FOG4\0" as *const u8 as *const libc::c_char,
    b"S_FOG5\0" as *const u8 as *const libc::c_char,
    b"S_FOG6\0" as *const u8 as *const libc::c_char,
    b"S_FOG7\0" as *const u8 as *const libc::c_char,
    b"S_FOG8\0" as *const u8 as *const libc::c_char,
    b"S_FOG9\0" as *const u8 as *const libc::c_char,
    b"S_FOG10\0" as *const u8 as *const libc::c_char,
    b"S_FOG11\0" as *const u8 as *const libc::c_char,
    b"S_FOG12\0" as *const u8 as *const libc::c_char,
    b"S_FOG13\0" as *const u8 as *const libc::c_char,
    b"S_FOG14\0" as *const u8 as *const libc::c_char,
    b"S_SEED\0" as *const u8 as *const libc::c_char,
    b"S_PARTICLE\0" as *const u8 as *const libc::c_char,
    b"S_SCRA\0" as *const u8 as *const libc::c_char,
    b"S_SCRB\0" as *const u8 as *const libc::c_char,
    b"S_SCRC\0" as *const u8 as *const libc::c_char,
    b"S_SCRD\0" as *const u8 as *const libc::c_char,
    b"S_SCRE\0" as *const u8 as *const libc::c_char,
    b"S_SCRF\0" as *const u8 as *const libc::c_char,
    b"S_SCRG\0" as *const u8 as *const libc::c_char,
    b"S_SCRH\0" as *const u8 as *const libc::c_char,
    b"S_SCRI\0" as *const u8 as *const libc::c_char,
    b"S_SCRJ\0" as *const u8 as *const libc::c_char,
    b"S_SCRK\0" as *const u8 as *const libc::c_char,
    b"S_SCRL\0" as *const u8 as *const libc::c_char,
    b"S_ZERO1\0" as *const u8 as *const libc::c_char,
    b"S_ONE1\0" as *const u8 as *const libc::c_char,
    b"S_TWO1\0" as *const u8 as *const libc::c_char,
    b"S_THREE1\0" as *const u8 as *const libc::c_char,
    b"S_FOUR1\0" as *const u8 as *const libc::c_char,
    b"S_FIVE1\0" as *const u8 as *const libc::c_char,
    b"S_ZERO2\0" as *const u8 as *const libc::c_char,
    b"S_ONE2\0" as *const u8 as *const libc::c_char,
    b"S_TWO2\0" as *const u8 as *const libc::c_char,
    b"S_THREE2\0" as *const u8 as *const libc::c_char,
    b"S_FOUR2\0" as *const u8 as *const libc::c_char,
    b"S_FIVE2\0" as *const u8 as *const libc::c_char,
    b"S_FLIGHTINDICATOR\0" as *const u8 as *const libc::c_char,
    b"S_LOCKON1\0" as *const u8 as *const libc::c_char,
    b"S_LOCKON2\0" as *const u8 as *const libc::c_char,
    b"S_LOCKON3\0" as *const u8 as *const libc::c_char,
    b"S_LOCKON4\0" as *const u8 as *const libc::c_char,
    b"S_LOCKONINF1\0" as *const u8 as *const libc::c_char,
    b"S_LOCKONINF2\0" as *const u8 as *const libc::c_char,
    b"S_LOCKONINF3\0" as *const u8 as *const libc::c_char,
    b"S_LOCKONINF4\0" as *const u8 as *const libc::c_char,
    b"S_TTAG\0" as *const u8 as *const libc::c_char,
    b"S_GOTFLAG\0" as *const u8 as *const libc::c_char,
    b"S_FINISHFLAG\0" as *const u8 as *const libc::c_char,
    b"S_CORK\0" as *const u8 as *const libc::c_char,
    b"S_LHRT\0" as *const u8 as *const libc::c_char,
    b"S_RRNG1\0" as *const u8 as *const libc::c_char,
    b"S_RRNG2\0" as *const u8 as *const libc::c_char,
    b"S_RRNG3\0" as *const u8 as *const libc::c_char,
    b"S_RRNG4\0" as *const u8 as *const libc::c_char,
    b"S_RRNG5\0" as *const u8 as *const libc::c_char,
    b"S_RRNG6\0" as *const u8 as *const libc::c_char,
    b"S_RRNG7\0" as *const u8 as *const libc::c_char,
    b"S_BOUNCERINGAMMO\0" as *const u8 as *const libc::c_char,
    b"S_RAILRINGAMMO\0" as *const u8 as *const libc::c_char,
    b"S_INFINITYRINGAMMO\0" as *const u8 as *const libc::c_char,
    b"S_AUTOMATICRINGAMMO\0" as *const u8 as *const libc::c_char,
    b"S_EXPLOSIONRINGAMMO\0" as *const u8 as *const libc::c_char,
    b"S_SCATTERRINGAMMO\0" as *const u8 as *const libc::c_char,
    b"S_GRENADERINGAMMO\0" as *const u8 as *const libc::c_char,
    b"S_BOUNCEPICKUP\0" as *const u8 as *const libc::c_char,
    b"S_BOUNCEPICKUPFADE1\0" as *const u8 as *const libc::c_char,
    b"S_BOUNCEPICKUPFADE2\0" as *const u8 as *const libc::c_char,
    b"S_BOUNCEPICKUPFADE3\0" as *const u8 as *const libc::c_char,
    b"S_BOUNCEPICKUPFADE4\0" as *const u8 as *const libc::c_char,
    b"S_BOUNCEPICKUPFADE5\0" as *const u8 as *const libc::c_char,
    b"S_BOUNCEPICKUPFADE6\0" as *const u8 as *const libc::c_char,
    b"S_BOUNCEPICKUPFADE7\0" as *const u8 as *const libc::c_char,
    b"S_BOUNCEPICKUPFADE8\0" as *const u8 as *const libc::c_char,
    b"S_RAILPICKUP\0" as *const u8 as *const libc::c_char,
    b"S_RAILPICKUPFADE1\0" as *const u8 as *const libc::c_char,
    b"S_RAILPICKUPFADE2\0" as *const u8 as *const libc::c_char,
    b"S_RAILPICKUPFADE3\0" as *const u8 as *const libc::c_char,
    b"S_RAILPICKUPFADE4\0" as *const u8 as *const libc::c_char,
    b"S_RAILPICKUPFADE5\0" as *const u8 as *const libc::c_char,
    b"S_RAILPICKUPFADE6\0" as *const u8 as *const libc::c_char,
    b"S_RAILPICKUPFADE7\0" as *const u8 as *const libc::c_char,
    b"S_RAILPICKUPFADE8\0" as *const u8 as *const libc::c_char,
    b"S_AUTOPICKUP\0" as *const u8 as *const libc::c_char,
    b"S_AUTOPICKUPFADE1\0" as *const u8 as *const libc::c_char,
    b"S_AUTOPICKUPFADE2\0" as *const u8 as *const libc::c_char,
    b"S_AUTOPICKUPFADE3\0" as *const u8 as *const libc::c_char,
    b"S_AUTOPICKUPFADE4\0" as *const u8 as *const libc::c_char,
    b"S_AUTOPICKUPFADE5\0" as *const u8 as *const libc::c_char,
    b"S_AUTOPICKUPFADE6\0" as *const u8 as *const libc::c_char,
    b"S_AUTOPICKUPFADE7\0" as *const u8 as *const libc::c_char,
    b"S_AUTOPICKUPFADE8\0" as *const u8 as *const libc::c_char,
    b"S_EXPLODEPICKUP\0" as *const u8 as *const libc::c_char,
    b"S_EXPLODEPICKUPFADE1\0" as *const u8 as *const libc::c_char,
    b"S_EXPLODEPICKUPFADE2\0" as *const u8 as *const libc::c_char,
    b"S_EXPLODEPICKUPFADE3\0" as *const u8 as *const libc::c_char,
    b"S_EXPLODEPICKUPFADE4\0" as *const u8 as *const libc::c_char,
    b"S_EXPLODEPICKUPFADE5\0" as *const u8 as *const libc::c_char,
    b"S_EXPLODEPICKUPFADE6\0" as *const u8 as *const libc::c_char,
    b"S_EXPLODEPICKUPFADE7\0" as *const u8 as *const libc::c_char,
    b"S_EXPLODEPICKUPFADE8\0" as *const u8 as *const libc::c_char,
    b"S_SCATTERPICKUP\0" as *const u8 as *const libc::c_char,
    b"S_SCATTERPICKUPFADE1\0" as *const u8 as *const libc::c_char,
    b"S_SCATTERPICKUPFADE2\0" as *const u8 as *const libc::c_char,
    b"S_SCATTERPICKUPFADE3\0" as *const u8 as *const libc::c_char,
    b"S_SCATTERPICKUPFADE4\0" as *const u8 as *const libc::c_char,
    b"S_SCATTERPICKUPFADE5\0" as *const u8 as *const libc::c_char,
    b"S_SCATTERPICKUPFADE6\0" as *const u8 as *const libc::c_char,
    b"S_SCATTERPICKUPFADE7\0" as *const u8 as *const libc::c_char,
    b"S_SCATTERPICKUPFADE8\0" as *const u8 as *const libc::c_char,
    b"S_GRENADEPICKUP\0" as *const u8 as *const libc::c_char,
    b"S_GRENADEPICKUPFADE1\0" as *const u8 as *const libc::c_char,
    b"S_GRENADEPICKUPFADE2\0" as *const u8 as *const libc::c_char,
    b"S_GRENADEPICKUPFADE3\0" as *const u8 as *const libc::c_char,
    b"S_GRENADEPICKUPFADE4\0" as *const u8 as *const libc::c_char,
    b"S_GRENADEPICKUPFADE5\0" as *const u8 as *const libc::c_char,
    b"S_GRENADEPICKUPFADE6\0" as *const u8 as *const libc::c_char,
    b"S_GRENADEPICKUPFADE7\0" as *const u8 as *const libc::c_char,
    b"S_GRENADEPICKUPFADE8\0" as *const u8 as *const libc::c_char,
    b"S_THROWNBOUNCE1\0" as *const u8 as *const libc::c_char,
    b"S_THROWNBOUNCE2\0" as *const u8 as *const libc::c_char,
    b"S_THROWNBOUNCE3\0" as *const u8 as *const libc::c_char,
    b"S_THROWNBOUNCE4\0" as *const u8 as *const libc::c_char,
    b"S_THROWNBOUNCE5\0" as *const u8 as *const libc::c_char,
    b"S_THROWNBOUNCE6\0" as *const u8 as *const libc::c_char,
    b"S_THROWNBOUNCE7\0" as *const u8 as *const libc::c_char,
    b"S_THROWNINFINITY1\0" as *const u8 as *const libc::c_char,
    b"S_THROWNINFINITY2\0" as *const u8 as *const libc::c_char,
    b"S_THROWNINFINITY3\0" as *const u8 as *const libc::c_char,
    b"S_THROWNINFINITY4\0" as *const u8 as *const libc::c_char,
    b"S_THROWNINFINITY5\0" as *const u8 as *const libc::c_char,
    b"S_THROWNINFINITY6\0" as *const u8 as *const libc::c_char,
    b"S_THROWNINFINITY7\0" as *const u8 as *const libc::c_char,
    b"S_THROWNAUTOMATIC1\0" as *const u8 as *const libc::c_char,
    b"S_THROWNAUTOMATIC2\0" as *const u8 as *const libc::c_char,
    b"S_THROWNAUTOMATIC3\0" as *const u8 as *const libc::c_char,
    b"S_THROWNAUTOMATIC4\0" as *const u8 as *const libc::c_char,
    b"S_THROWNAUTOMATIC5\0" as *const u8 as *const libc::c_char,
    b"S_THROWNAUTOMATIC6\0" as *const u8 as *const libc::c_char,
    b"S_THROWNAUTOMATIC7\0" as *const u8 as *const libc::c_char,
    b"S_THROWNEXPLOSION1\0" as *const u8 as *const libc::c_char,
    b"S_THROWNEXPLOSION2\0" as *const u8 as *const libc::c_char,
    b"S_THROWNEXPLOSION3\0" as *const u8 as *const libc::c_char,
    b"S_THROWNEXPLOSION4\0" as *const u8 as *const libc::c_char,
    b"S_THROWNEXPLOSION5\0" as *const u8 as *const libc::c_char,
    b"S_THROWNEXPLOSION6\0" as *const u8 as *const libc::c_char,
    b"S_THROWNEXPLOSION7\0" as *const u8 as *const libc::c_char,
    b"S_THROWNGRENADE1\0" as *const u8 as *const libc::c_char,
    b"S_THROWNGRENADE2\0" as *const u8 as *const libc::c_char,
    b"S_THROWNGRENADE3\0" as *const u8 as *const libc::c_char,
    b"S_THROWNGRENADE4\0" as *const u8 as *const libc::c_char,
    b"S_THROWNGRENADE5\0" as *const u8 as *const libc::c_char,
    b"S_THROWNGRENADE6\0" as *const u8 as *const libc::c_char,
    b"S_THROWNGRENADE7\0" as *const u8 as *const libc::c_char,
    b"S_THROWNGRENADE8\0" as *const u8 as *const libc::c_char,
    b"S_THROWNGRENADE9\0" as *const u8 as *const libc::c_char,
    b"S_THROWNGRENADE10\0" as *const u8 as *const libc::c_char,
    b"S_THROWNGRENADE11\0" as *const u8 as *const libc::c_char,
    b"S_THROWNGRENADE12\0" as *const u8 as *const libc::c_char,
    b"S_THROWNGRENADE13\0" as *const u8 as *const libc::c_char,
    b"S_THROWNGRENADE14\0" as *const u8 as *const libc::c_char,
    b"S_THROWNGRENADE15\0" as *const u8 as *const libc::c_char,
    b"S_THROWNGRENADE16\0" as *const u8 as *const libc::c_char,
    b"S_THROWNGRENADE17\0" as *const u8 as *const libc::c_char,
    b"S_THROWNGRENADE18\0" as *const u8 as *const libc::c_char,
    b"S_THROWNSCATTER\0" as *const u8 as *const libc::c_char,
    b"S_RINGEXPLODE\0" as *const u8 as *const libc::c_char,
    b"S_COIN1\0" as *const u8 as *const libc::c_char,
    b"S_COIN2\0" as *const u8 as *const libc::c_char,
    b"S_COIN3\0" as *const u8 as *const libc::c_char,
    b"S_COINSPARKLE1\0" as *const u8 as *const libc::c_char,
    b"S_COINSPARKLE2\0" as *const u8 as *const libc::c_char,
    b"S_COINSPARKLE3\0" as *const u8 as *const libc::c_char,
    b"S_COINSPARKLE4\0" as *const u8 as *const libc::c_char,
    b"S_GOOMBA1\0" as *const u8 as *const libc::c_char,
    b"S_GOOMBA1B\0" as *const u8 as *const libc::c_char,
    b"S_GOOMBA2\0" as *const u8 as *const libc::c_char,
    b"S_GOOMBA3\0" as *const u8 as *const libc::c_char,
    b"S_GOOMBA4\0" as *const u8 as *const libc::c_char,
    b"S_GOOMBA5\0" as *const u8 as *const libc::c_char,
    b"S_GOOMBA6\0" as *const u8 as *const libc::c_char,
    b"S_GOOMBA7\0" as *const u8 as *const libc::c_char,
    b"S_GOOMBA8\0" as *const u8 as *const libc::c_char,
    b"S_GOOMBA9\0" as *const u8 as *const libc::c_char,
    b"S_GOOMBA_DEAD\0" as *const u8 as *const libc::c_char,
    b"S_BLUEGOOMBA1\0" as *const u8 as *const libc::c_char,
    b"S_BLUEGOOMBA1B\0" as *const u8 as *const libc::c_char,
    b"S_BLUEGOOMBA2\0" as *const u8 as *const libc::c_char,
    b"S_BLUEGOOMBA3\0" as *const u8 as *const libc::c_char,
    b"S_BLUEGOOMBA4\0" as *const u8 as *const libc::c_char,
    b"S_BLUEGOOMBA5\0" as *const u8 as *const libc::c_char,
    b"S_BLUEGOOMBA6\0" as *const u8 as *const libc::c_char,
    b"S_BLUEGOOMBA7\0" as *const u8 as *const libc::c_char,
    b"S_BLUEGOOMBA8\0" as *const u8 as *const libc::c_char,
    b"S_BLUEGOOMBA9\0" as *const u8 as *const libc::c_char,
    b"S_BLUEGOOMBA_DEAD\0" as *const u8 as *const libc::c_char,
    b"S_FIREFLOWER1\0" as *const u8 as *const libc::c_char,
    b"S_FIREFLOWER2\0" as *const u8 as *const libc::c_char,
    b"S_FIREFLOWER3\0" as *const u8 as *const libc::c_char,
    b"S_FIREFLOWER4\0" as *const u8 as *const libc::c_char,
    b"S_FIREBALL\0" as *const u8 as *const libc::c_char,
    b"S_FIREBALLTRAIL1\0" as *const u8 as *const libc::c_char,
    b"S_FIREBALLTRAIL2\0" as *const u8 as *const libc::c_char,
    b"S_SHELL\0" as *const u8 as *const libc::c_char,
    b"S_PUMA_START1\0" as *const u8 as *const libc::c_char,
    b"S_PUMA_START2\0" as *const u8 as *const libc::c_char,
    b"S_PUMA_UP1\0" as *const u8 as *const libc::c_char,
    b"S_PUMA_UP2\0" as *const u8 as *const libc::c_char,
    b"S_PUMA_UP3\0" as *const u8 as *const libc::c_char,
    b"S_PUMA_DOWN1\0" as *const u8 as *const libc::c_char,
    b"S_PUMA_DOWN2\0" as *const u8 as *const libc::c_char,
    b"S_PUMA_DOWN3\0" as *const u8 as *const libc::c_char,
    b"S_PUMATRAIL1\0" as *const u8 as *const libc::c_char,
    b"S_PUMATRAIL2\0" as *const u8 as *const libc::c_char,
    b"S_PUMATRAIL3\0" as *const u8 as *const libc::c_char,
    b"S_PUMATRAIL4\0" as *const u8 as *const libc::c_char,
    b"S_HAMMER\0" as *const u8 as *const libc::c_char,
    b"S_KOOPA1\0" as *const u8 as *const libc::c_char,
    b"S_KOOPA2\0" as *const u8 as *const libc::c_char,
    b"S_KOOPAFLAME1\0" as *const u8 as *const libc::c_char,
    b"S_KOOPAFLAME2\0" as *const u8 as *const libc::c_char,
    b"S_KOOPAFLAME3\0" as *const u8 as *const libc::c_char,
    b"S_AXE1\0" as *const u8 as *const libc::c_char,
    b"S_AXE2\0" as *const u8 as *const libc::c_char,
    b"S_AXE3\0" as *const u8 as *const libc::c_char,
    b"S_MARIOBUSH1\0" as *const u8 as *const libc::c_char,
    b"S_MARIOBUSH2\0" as *const u8 as *const libc::c_char,
    b"S_TOAD\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_MAN1\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_MAN2\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_SPARKLING1\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_SPARKLING2\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_SPARKLING3\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_SPARKLING4\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_SPARKLING5\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_SPARKLING6\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_SPARKLING7\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_SPARKLING8\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_SPARKLING9\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_SPARKLING10\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_SPARKLING11\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_SPARKLING12\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_SPARKLING13\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_SPARKLING14\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_SPARKLING15\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_SPARKLING16\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_GOAL1\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_GOAL2\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_GOAL3\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRONE_GOAL4\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSPARKLE1\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSPARKLE2\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSPARKLE3\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSPARKLE4\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSPARKLESUPER1\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSPARKLESUPER2\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSPARKLESUPER3\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSPARKLESUPER4\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSLOOPHELPER\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSBUMPER1\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSBUMPER2\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSBUMPER3\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSBUMPER4\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSBUMPER5\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSBUMPER6\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSBUMPER7\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSBUMPER8\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSBUMPER9\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSBUMPER10\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSBUMPER11\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSBUMPER12\0" as *const u8 as *const libc::c_char,
    b"S_HOOP\0" as *const u8 as *const libc::c_char,
    b"S_HOOP_XMASA\0" as *const u8 as *const libc::c_char,
    b"S_HOOP_XMASB\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE10\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE20\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE30\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE40\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE50\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE60\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE70\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE80\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE90\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE100\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE10_2\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE20_2\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE30_2\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE40_2\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE50_2\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE60_2\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE70_2\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE80_2\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE90_2\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSCORE100_2\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSSUPERLOOP\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSDRILLREFILL\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSHELPER\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSEXTRATIME\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTSLINKFREEZE\0" as *const u8 as *const libc::c_char,
    b"S_EGGCAPSULE\0" as *const u8 as *const libc::c_char,
    b"S_ORBITEM1\0" as *const u8 as *const libc::c_char,
    b"S_ORBITEM2\0" as *const u8 as *const libc::c_char,
    b"S_ORBITEM3\0" as *const u8 as *const libc::c_char,
    b"S_ORBITEM4\0" as *const u8 as *const libc::c_char,
    b"S_ORBITEM5\0" as *const u8 as *const libc::c_char,
    b"S_ORBITEM6\0" as *const u8 as *const libc::c_char,
    b"S_ORBITEM7\0" as *const u8 as *const libc::c_char,
    b"S_ORBITEM8\0" as *const u8 as *const libc::c_char,
    b"S_ORBIDYA1\0" as *const u8 as *const libc::c_char,
    b"S_ORBIDYA2\0" as *const u8 as *const libc::c_char,
    b"S_ORBIDYA3\0" as *const u8 as *const libc::c_char,
    b"S_ORBIDYA4\0" as *const u8 as *const libc::c_char,
    b"S_ORBIDYA5\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTOPIANHELPER1\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTOPIANHELPER2\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTOPIANHELPER3\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTOPIANHELPER4\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTOPIANHELPER5\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTOPIANHELPER6\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTOPIANHELPER7\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTOPIANHELPER8\0" as *const u8 as *const libc::c_char,
    b"S_NIGHTOPIANHELPER9\0" as *const u8 as *const libc::c_char,
    b"S_PIAN_LOOK1\0" as *const u8 as *const libc::c_char,
    b"S_PIAN_LOOK2\0" as *const u8 as *const libc::c_char,
    b"S_PIAN_LOOK3\0" as *const u8 as *const libc::c_char,
    b"S_PIAN_FLY1\0" as *const u8 as *const libc::c_char,
    b"S_PIAN_FLY2\0" as *const u8 as *const libc::c_char,
    b"S_PIAN_FLY3\0" as *const u8 as *const libc::c_char,
    b"S_PIAN_SING\0" as *const u8 as *const libc::c_char,
    b"S_SHLEEP1\0" as *const u8 as *const libc::c_char,
    b"S_SHLEEP2\0" as *const u8 as *const libc::c_char,
    b"S_SHLEEP3\0" as *const u8 as *const libc::c_char,
    b"S_SHLEEP4\0" as *const u8 as *const libc::c_char,
    b"S_SHLEEPBOUNCE1\0" as *const u8 as *const libc::c_char,
    b"S_SHLEEPBOUNCE2\0" as *const u8 as *const libc::c_char,
    b"S_SHLEEPBOUNCE3\0" as *const u8 as *const libc::c_char,
    b"S_PENGUINATOR_LOOK\0" as *const u8 as *const libc::c_char,
    b"S_PENGUINATOR_WADDLE1\0" as *const u8 as *const libc::c_char,
    b"S_PENGUINATOR_WADDLE2\0" as *const u8 as *const libc::c_char,
    b"S_PENGUINATOR_WADDLE3\0" as *const u8 as *const libc::c_char,
    b"S_PENGUINATOR_WADDLE4\0" as *const u8 as *const libc::c_char,
    b"S_PENGUINATOR_SLIDE1\0" as *const u8 as *const libc::c_char,
    b"S_PENGUINATOR_SLIDE2\0" as *const u8 as *const libc::c_char,
    b"S_PENGUINATOR_SLIDE3\0" as *const u8 as *const libc::c_char,
    b"S_PENGUINATOR_SLIDE4\0" as *const u8 as *const libc::c_char,
    b"S_PENGUINATOR_SLIDE5\0" as *const u8 as *const libc::c_char,
    b"S_POPHAT_LOOK\0" as *const u8 as *const libc::c_char,
    b"S_POPHAT_SHOOT1\0" as *const u8 as *const libc::c_char,
    b"S_POPHAT_SHOOT2\0" as *const u8 as *const libc::c_char,
    b"S_POPHAT_SHOOT3\0" as *const u8 as *const libc::c_char,
    b"S_POPHAT_SHOOT4\0" as *const u8 as *const libc::c_char,
    b"S_POPSHOT\0" as *const u8 as *const libc::c_char,
    b"S_POPSHOT_TRAIL\0" as *const u8 as *const libc::c_char,
    b"S_HIVEELEMENTAL_LOOK\0" as *const u8 as *const libc::c_char,
    b"S_HIVEELEMENTAL_PREPARE1\0" as *const u8 as *const libc::c_char,
    b"S_HIVEELEMENTAL_PREPARE2\0" as *const u8 as *const libc::c_char,
    b"S_HIVEELEMENTAL_SHOOT1\0" as *const u8 as *const libc::c_char,
    b"S_HIVEELEMENTAL_SHOOT2\0" as *const u8 as *const libc::c_char,
    b"S_HIVEELEMENTAL_DORMANT\0" as *const u8 as *const libc::c_char,
    b"S_HIVEELEMENTAL_PAIN\0" as *const u8 as *const libc::c_char,
    b"S_HIVEELEMENTAL_DIE1\0" as *const u8 as *const libc::c_char,
    b"S_HIVEELEMENTAL_DIE2\0" as *const u8 as *const libc::c_char,
    b"S_HIVEELEMENTAL_DIE3\0" as *const u8 as *const libc::c_char,
    b"S_BUMBLEBORE_SPAWN\0" as *const u8 as *const libc::c_char,
    b"S_BUMBLEBORE_LOOK1\0" as *const u8 as *const libc::c_char,
    b"S_BUMBLEBORE_LOOK2\0" as *const u8 as *const libc::c_char,
    b"S_BUMBLEBORE_FLY1\0" as *const u8 as *const libc::c_char,
    b"S_BUMBLEBORE_FLY2\0" as *const u8 as *const libc::c_char,
    b"S_BUMBLEBORE_RAISE\0" as *const u8 as *const libc::c_char,
    b"S_BUMBLEBORE_FALL1\0" as *const u8 as *const libc::c_char,
    b"S_BUMBLEBORE_FALL2\0" as *const u8 as *const libc::c_char,
    b"S_BUMBLEBORE_STUCK1\0" as *const u8 as *const libc::c_char,
    b"S_BUMBLEBORE_STUCK2\0" as *const u8 as *const libc::c_char,
    b"S_BUMBLEBORE_DIE\0" as *const u8 as *const libc::c_char,
    b"S_BUGGLEIDLE\0" as *const u8 as *const libc::c_char,
    b"S_BUGGLEFLY\0" as *const u8 as *const libc::c_char,
    b"S_SMASHSPIKE_FLOAT\0" as *const u8 as *const libc::c_char,
    b"S_SMASHSPIKE_EASE1\0" as *const u8 as *const libc::c_char,
    b"S_SMASHSPIKE_EASE2\0" as *const u8 as *const libc::c_char,
    b"S_SMASHSPIKE_FALL\0" as *const u8 as *const libc::c_char,
    b"S_SMASHSPIKE_STOMP1\0" as *const u8 as *const libc::c_char,
    b"S_SMASHSPIKE_STOMP2\0" as *const u8 as *const libc::c_char,
    b"S_SMASHSPIKE_RISE1\0" as *const u8 as *const libc::c_char,
    b"S_SMASHSPIKE_RISE2\0" as *const u8 as *const libc::c_char,
    b"S_CACO_LOOK\0" as *const u8 as *const libc::c_char,
    b"S_CACO_WAKE1\0" as *const u8 as *const libc::c_char,
    b"S_CACO_WAKE2\0" as *const u8 as *const libc::c_char,
    b"S_CACO_WAKE3\0" as *const u8 as *const libc::c_char,
    b"S_CACO_WAKE4\0" as *const u8 as *const libc::c_char,
    b"S_CACO_ROAR\0" as *const u8 as *const libc::c_char,
    b"S_CACO_CHASE\0" as *const u8 as *const libc::c_char,
    b"S_CACO_CHASE_REPEAT\0" as *const u8 as *const libc::c_char,
    b"S_CACO_RANDOM\0" as *const u8 as *const libc::c_char,
    b"S_CACO_PREPARE_SOUND\0" as *const u8 as *const libc::c_char,
    b"S_CACO_PREPARE1\0" as *const u8 as *const libc::c_char,
    b"S_CACO_PREPARE2\0" as *const u8 as *const libc::c_char,
    b"S_CACO_PREPARE3\0" as *const u8 as *const libc::c_char,
    b"S_CACO_SHOOT_SOUND\0" as *const u8 as *const libc::c_char,
    b"S_CACO_SHOOT1\0" as *const u8 as *const libc::c_char,
    b"S_CACO_SHOOT2\0" as *const u8 as *const libc::c_char,
    b"S_CACO_CLOSE\0" as *const u8 as *const libc::c_char,
    b"S_CACO_DIE_FLAGS\0" as *const u8 as *const libc::c_char,
    b"S_CACO_DIE_GIB1\0" as *const u8 as *const libc::c_char,
    b"S_CACO_DIE_GIB2\0" as *const u8 as *const libc::c_char,
    b"S_CACO_DIE_SCREAM\0" as *const u8 as *const libc::c_char,
    b"S_CACO_DIE_SHATTER\0" as *const u8 as *const libc::c_char,
    b"S_CACO_DIE_FALL\0" as *const u8 as *const libc::c_char,
    b"S_CACOSHARD_RANDOMIZE\0" as *const u8 as *const libc::c_char,
    b"S_CACOSHARD1_1\0" as *const u8 as *const libc::c_char,
    b"S_CACOSHARD1_2\0" as *const u8 as *const libc::c_char,
    b"S_CACOSHARD2_1\0" as *const u8 as *const libc::c_char,
    b"S_CACOSHARD2_2\0" as *const u8 as *const libc::c_char,
    b"S_CACOFIRE1\0" as *const u8 as *const libc::c_char,
    b"S_CACOFIRE2\0" as *const u8 as *const libc::c_char,
    b"S_CACOFIRE3\0" as *const u8 as *const libc::c_char,
    b"S_CACOFIRE_EXPLODE1\0" as *const u8 as *const libc::c_char,
    b"S_CACOFIRE_EXPLODE2\0" as *const u8 as *const libc::c_char,
    b"S_CACOFIRE_EXPLODE3\0" as *const u8 as *const libc::c_char,
    b"S_CACOFIRE_EXPLODE4\0" as *const u8 as *const libc::c_char,
    b"S_SPINBOBERT_MOVE_FLIPUP\0" as *const u8 as *const libc::c_char,
    b"S_SPINBOBERT_MOVE_UP\0" as *const u8 as *const libc::c_char,
    b"S_SPINBOBERT_MOVE_FLIPDOWN\0" as *const u8 as *const libc::c_char,
    b"S_SPINBOBERT_MOVE_DOWN\0" as *const u8 as *const libc::c_char,
    b"S_SPINBOBERT_FIRE_MOVE\0" as *const u8 as *const libc::c_char,
    b"S_SPINBOBERT_FIRE_GHOST\0" as *const u8 as *const libc::c_char,
    b"S_SPINBOBERT_FIRE_TRAIL1\0" as *const u8 as *const libc::c_char,
    b"S_SPINBOBERT_FIRE_TRAIL2\0" as *const u8 as *const libc::c_char,
    b"S_SPINBOBERT_FIRE_TRAIL3\0" as *const u8 as *const libc::c_char,
    b"S_HANGSTER_LOOK\0" as *const u8 as *const libc::c_char,
    b"S_HANGSTER_SWOOP1\0" as *const u8 as *const libc::c_char,
    b"S_HANGSTER_SWOOP2\0" as *const u8 as *const libc::c_char,
    b"S_HANGSTER_ARC1\0" as *const u8 as *const libc::c_char,
    b"S_HANGSTER_ARC2\0" as *const u8 as *const libc::c_char,
    b"S_HANGSTER_ARC3\0" as *const u8 as *const libc::c_char,
    b"S_HANGSTER_FLY1\0" as *const u8 as *const libc::c_char,
    b"S_HANGSTER_FLY2\0" as *const u8 as *const libc::c_char,
    b"S_HANGSTER_FLY3\0" as *const u8 as *const libc::c_char,
    b"S_HANGSTER_FLY4\0" as *const u8 as *const libc::c_char,
    b"S_HANGSTER_FLYREPEAT\0" as *const u8 as *const libc::c_char,
    b"S_HANGSTER_ARCUP1\0" as *const u8 as *const libc::c_char,
    b"S_HANGSTER_ARCUP2\0" as *const u8 as *const libc::c_char,
    b"S_HANGSTER_ARCUP3\0" as *const u8 as *const libc::c_char,
    b"S_HANGSTER_RETURN1\0" as *const u8 as *const libc::c_char,
    b"S_HANGSTER_RETURN2\0" as *const u8 as *const libc::c_char,
    b"S_HANGSTER_RETURN3\0" as *const u8 as *const libc::c_char,
    b"S_CRUMBLE1\0" as *const u8 as *const libc::c_char,
    b"S_CRUMBLE2\0" as *const u8 as *const libc::c_char,
    b"S_SPRK1\0" as *const u8 as *const libc::c_char,
    b"S_SPRK2\0" as *const u8 as *const libc::c_char,
    b"S_SPRK3\0" as *const u8 as *const libc::c_char,
    b"S_XPLD_FLICKY\0" as *const u8 as *const libc::c_char,
    b"S_XPLD1\0" as *const u8 as *const libc::c_char,
    b"S_XPLD2\0" as *const u8 as *const libc::c_char,
    b"S_XPLD3\0" as *const u8 as *const libc::c_char,
    b"S_XPLD4\0" as *const u8 as *const libc::c_char,
    b"S_XPLD5\0" as *const u8 as *const libc::c_char,
    b"S_XPLD6\0" as *const u8 as *const libc::c_char,
    b"S_XPLD_EGGTRAP\0" as *const u8 as *const libc::c_char,
    b"S_WPLD1\0" as *const u8 as *const libc::c_char,
    b"S_WPLD2\0" as *const u8 as *const libc::c_char,
    b"S_WPLD3\0" as *const u8 as *const libc::c_char,
    b"S_WPLD4\0" as *const u8 as *const libc::c_char,
    b"S_WPLD5\0" as *const u8 as *const libc::c_char,
    b"S_WPLD6\0" as *const u8 as *const libc::c_char,
    b"S_DUST1\0" as *const u8 as *const libc::c_char,
    b"S_DUST2\0" as *const u8 as *const libc::c_char,
    b"S_DUST3\0" as *const u8 as *const libc::c_char,
    b"S_DUST4\0" as *const u8 as *const libc::c_char,
    b"S_ROCKSPAWN\0" as *const u8 as *const libc::c_char,
    b"S_ROCKCRUMBLEA\0" as *const u8 as *const libc::c_char,
    b"S_ROCKCRUMBLEB\0" as *const u8 as *const libc::c_char,
    b"S_ROCKCRUMBLEC\0" as *const u8 as *const libc::c_char,
    b"S_ROCKCRUMBLED\0" as *const u8 as *const libc::c_char,
    b"S_ROCKCRUMBLEE\0" as *const u8 as *const libc::c_char,
    b"S_ROCKCRUMBLEF\0" as *const u8 as *const libc::c_char,
    b"S_ROCKCRUMBLEG\0" as *const u8 as *const libc::c_char,
    b"S_ROCKCRUMBLEH\0" as *const u8 as *const libc::c_char,
    b"S_ROCKCRUMBLEI\0" as *const u8 as *const libc::c_char,
    b"S_ROCKCRUMBLEJ\0" as *const u8 as *const libc::c_char,
    b"S_ROCKCRUMBLEK\0" as *const u8 as *const libc::c_char,
    b"S_ROCKCRUMBLEL\0" as *const u8 as *const libc::c_char,
    b"S_ROCKCRUMBLEM\0" as *const u8 as *const libc::c_char,
    b"S_ROCKCRUMBLEN\0" as *const u8 as *const libc::c_char,
    b"S_ROCKCRUMBLEO\0" as *const u8 as *const libc::c_char,
    b"S_ROCKCRUMBLEP\0" as *const u8 as *const libc::c_char,
    b"S_GFZDEBRIS\0" as *const u8 as *const libc::c_char,
    b"S_BRICKDEBRIS\0" as *const u8 as *const libc::c_char,
    b"S_WOODDEBRIS\0" as *const u8 as *const libc::c_char,
    b"S_REDBRICKDEBRIS\0" as *const u8 as *const libc::c_char,
    b"S_BLUEBRICKDEBRIS\0" as *const u8 as *const libc::c_char,
    b"S_YELLOWBRICKDEBRIS\0" as *const u8 as *const libc::c_char,
    b"S_NAMECHECK\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut MOBJTYPE_LIST: [*const libc::c_char; 651] = [
    b"MT_NULL\0" as *const u8 as *const libc::c_char,
    b"MT_UNKNOWN\0" as *const u8 as *const libc::c_char,
    b"MT_THOK\0" as *const u8 as *const libc::c_char,
    b"MT_PLAYER\0" as *const u8 as *const libc::c_char,
    b"MT_TAILSOVERLAY\0" as *const u8 as *const libc::c_char,
    b"MT_METALJETFUME\0" as *const u8 as *const libc::c_char,
    b"MT_BLUECRAWLA\0" as *const u8 as *const libc::c_char,
    b"MT_REDCRAWLA\0" as *const u8 as *const libc::c_char,
    b"MT_GFZFISH\0" as *const u8 as *const libc::c_char,
    b"MT_GOLDBUZZ\0" as *const u8 as *const libc::c_char,
    b"MT_REDBUZZ\0" as *const u8 as *const libc::c_char,
    b"MT_JETTBOMBER\0" as *const u8 as *const libc::c_char,
    b"MT_JETTGUNNER\0" as *const u8 as *const libc::c_char,
    b"MT_CRAWLACOMMANDER\0" as *const u8 as *const libc::c_char,
    b"MT_DETON\0" as *const u8 as *const libc::c_char,
    b"MT_SKIM\0" as *const u8 as *const libc::c_char,
    b"MT_TURRET\0" as *const u8 as *const libc::c_char,
    b"MT_POPUPTURRET\0" as *const u8 as *const libc::c_char,
    b"MT_SPINCUSHION\0" as *const u8 as *const libc::c_char,
    b"MT_CRUSHSTACEAN\0" as *const u8 as *const libc::c_char,
    b"MT_CRUSHCLAW\0" as *const u8 as *const libc::c_char,
    b"MT_CRUSHCHAIN\0" as *const u8 as *const libc::c_char,
    b"MT_BANPYURA\0" as *const u8 as *const libc::c_char,
    b"MT_BANPSPRING\0" as *const u8 as *const libc::c_char,
    b"MT_JETJAW\0" as *const u8 as *const libc::c_char,
    b"MT_SNAILER\0" as *const u8 as *const libc::c_char,
    b"MT_VULTURE\0" as *const u8 as *const libc::c_char,
    b"MT_POINTY\0" as *const u8 as *const libc::c_char,
    b"MT_POINTYBALL\0" as *const u8 as *const libc::c_char,
    b"MT_ROBOHOOD\0" as *const u8 as *const libc::c_char,
    b"MT_FACESTABBER\0" as *const u8 as *const libc::c_char,
    b"MT_FACESTABBERSPEAR\0" as *const u8 as *const libc::c_char,
    b"MT_EGGGUARD\0" as *const u8 as *const libc::c_char,
    b"MT_EGGSHIELD\0" as *const u8 as *const libc::c_char,
    b"MT_GSNAPPER\0" as *const u8 as *const libc::c_char,
    b"MT_SNAPPER_LEG\0" as *const u8 as *const libc::c_char,
    b"MT_SNAPPER_HEAD\0" as *const u8 as *const libc::c_char,
    b"MT_MINUS\0" as *const u8 as *const libc::c_char,
    b"MT_MINUSDIRT\0" as *const u8 as *const libc::c_char,
    b"MT_SPRINGSHELL\0" as *const u8 as *const libc::c_char,
    b"MT_YELLOWSHELL\0" as *const u8 as *const libc::c_char,
    b"MT_UNIDUS\0" as *const u8 as *const libc::c_char,
    b"MT_UNIBALL\0" as *const u8 as *const libc::c_char,
    b"MT_CANARIVORE\0" as *const u8 as *const libc::c_char,
    b"MT_CANARIVORE_GAS\0" as *const u8 as *const libc::c_char,
    b"MT_PYREFLY\0" as *const u8 as *const libc::c_char,
    b"MT_PYREFLY_FIRE\0" as *const u8 as *const libc::c_char,
    b"MT_PTERABYTESPAWNER\0" as *const u8 as *const libc::c_char,
    b"MT_PTERABYTEWAYPOINT\0" as *const u8 as *const libc::c_char,
    b"MT_PTERABYTE\0" as *const u8 as *const libc::c_char,
    b"MT_DRAGONBOMBER\0" as *const u8 as *const libc::c_char,
    b"MT_DRAGONWING\0" as *const u8 as *const libc::c_char,
    b"MT_DRAGONTAIL\0" as *const u8 as *const libc::c_char,
    b"MT_DRAGONMINE\0" as *const u8 as *const libc::c_char,
    b"MT_BOSSEXPLODE\0" as *const u8 as *const libc::c_char,
    b"MT_SONIC3KBOSSEXPLODE\0" as *const u8 as *const libc::c_char,
    b"MT_BOSSFLYPOINT\0" as *const u8 as *const libc::c_char,
    b"MT_EGGTRAP\0" as *const u8 as *const libc::c_char,
    b"MT_BOSS3WAYPOINT\0" as *const u8 as *const libc::c_char,
    b"MT_BOSS9GATHERPOINT\0" as *const u8 as *const libc::c_char,
    b"MT_BOSSJUNK\0" as *const u8 as *const libc::c_char,
    b"MT_EGGMOBILE\0" as *const u8 as *const libc::c_char,
    b"MT_JETFUME1\0" as *const u8 as *const libc::c_char,
    b"MT_EGGMOBILE_BALL\0" as *const u8 as *const libc::c_char,
    b"MT_EGGMOBILE_TARGET\0" as *const u8 as *const libc::c_char,
    b"MT_EGGMOBILE_FIRE\0" as *const u8 as *const libc::c_char,
    b"MT_EGGMOBILE2\0" as *const u8 as *const libc::c_char,
    b"MT_EGGMOBILE2_POGO\0" as *const u8 as *const libc::c_char,
    b"MT_GOOP\0" as *const u8 as *const libc::c_char,
    b"MT_GOOPTRAIL\0" as *const u8 as *const libc::c_char,
    b"MT_EGGMOBILE3\0" as *const u8 as *const libc::c_char,
    b"MT_FAKEMOBILE\0" as *const u8 as *const libc::c_char,
    b"MT_SHOCKWAVE\0" as *const u8 as *const libc::c_char,
    b"MT_EGGMOBILE4\0" as *const u8 as *const libc::c_char,
    b"MT_EGGMOBILE4_MACE\0" as *const u8 as *const libc::c_char,
    b"MT_JETFLAME\0" as *const u8 as *const libc::c_char,
    b"MT_EGGROBO1\0" as *const u8 as *const libc::c_char,
    b"MT_EGGROBO1JET\0" as *const u8 as *const libc::c_char,
    b"MT_FANG\0" as *const u8 as *const libc::c_char,
    b"MT_BROKENROBOT\0" as *const u8 as *const libc::c_char,
    b"MT_VWREF\0" as *const u8 as *const libc::c_char,
    b"MT_VWREB\0" as *const u8 as *const libc::c_char,
    b"MT_PROJECTORLIGHT\0" as *const u8 as *const libc::c_char,
    b"MT_FBOMB\0" as *const u8 as *const libc::c_char,
    b"MT_TNTDUST\0" as *const u8 as *const libc::c_char,
    b"MT_FSGNA\0" as *const u8 as *const libc::c_char,
    b"MT_FSGNB\0" as *const u8 as *const libc::c_char,
    b"MT_FANGWAYPOINT\0" as *const u8 as *const libc::c_char,
    b"MT_BLACKEGGMAN\0" as *const u8 as *const libc::c_char,
    b"MT_BLACKEGGMAN_HELPER\0" as *const u8 as *const libc::c_char,
    b"MT_BLACKEGGMAN_GOOPFIRE\0" as *const u8 as *const libc::c_char,
    b"MT_BLACKEGGMAN_MISSILE\0" as *const u8 as *const libc::c_char,
    b"MT_CYBRAKDEMON\0" as *const u8 as *const libc::c_char,
    b"MT_CYBRAKDEMON_ELECTRIC_BARRIER\0" as *const u8 as *const libc::c_char,
    b"MT_CYBRAKDEMON_MISSILE\0" as *const u8 as *const libc::c_char,
    b"MT_CYBRAKDEMON_FLAMESHOT\0" as *const u8 as *const libc::c_char,
    b"MT_CYBRAKDEMON_FLAMEREST\0" as *const u8 as *const libc::c_char,
    b"MT_CYBRAKDEMON_TARGET_RETICULE\0" as *const u8 as *const libc::c_char,
    b"MT_CYBRAKDEMON_TARGET_DOT\0" as *const u8 as *const libc::c_char,
    b"MT_CYBRAKDEMON_NAPALM_BOMB_LARGE\0" as *const u8 as *const libc::c_char,
    b"MT_CYBRAKDEMON_NAPALM_BOMB_SMALL\0" as *const u8 as *const libc::c_char,
    b"MT_CYBRAKDEMON_NAPALM_FLAMES\0" as *const u8 as *const libc::c_char,
    b"MT_CYBRAKDEMON_VILE_EXPLOSION\0" as *const u8 as *const libc::c_char,
    b"MT_METALSONIC_RACE\0" as *const u8 as *const libc::c_char,
    b"MT_METALSONIC_BATTLE\0" as *const u8 as *const libc::c_char,
    b"MT_MSSHIELD_FRONT\0" as *const u8 as *const libc::c_char,
    b"MT_MSGATHER\0" as *const u8 as *const libc::c_char,
    b"MT_RING\0" as *const u8 as *const libc::c_char,
    b"MT_FLINGRING\0" as *const u8 as *const libc::c_char,
    b"MT_BLUESPHERE\0" as *const u8 as *const libc::c_char,
    b"MT_FLINGBLUESPHERE\0" as *const u8 as *const libc::c_char,
    b"MT_BOMBSPHERE\0" as *const u8 as *const libc::c_char,
    b"MT_REDTEAMRING\0" as *const u8 as *const libc::c_char,
    b"MT_BLUETEAMRING\0" as *const u8 as *const libc::c_char,
    b"MT_TOKEN\0" as *const u8 as *const libc::c_char,
    b"MT_REDFLAG\0" as *const u8 as *const libc::c_char,
    b"MT_BLUEFLAG\0" as *const u8 as *const libc::c_char,
    b"MT_EMBLEM\0" as *const u8 as *const libc::c_char,
    b"MT_EMERALD1\0" as *const u8 as *const libc::c_char,
    b"MT_EMERALD2\0" as *const u8 as *const libc::c_char,
    b"MT_EMERALD3\0" as *const u8 as *const libc::c_char,
    b"MT_EMERALD4\0" as *const u8 as *const libc::c_char,
    b"MT_EMERALD5\0" as *const u8 as *const libc::c_char,
    b"MT_EMERALD6\0" as *const u8 as *const libc::c_char,
    b"MT_EMERALD7\0" as *const u8 as *const libc::c_char,
    b"MT_EMERHUNT\0" as *const u8 as *const libc::c_char,
    b"MT_EMERALDSPAWN\0" as *const u8 as *const libc::c_char,
    b"MT_FLINGEMERALD\0" as *const u8 as *const libc::c_char,
    b"MT_FAN\0" as *const u8 as *const libc::c_char,
    b"MT_STEAM\0" as *const u8 as *const libc::c_char,
    b"MT_BUMPER\0" as *const u8 as *const libc::c_char,
    b"MT_BALLOON\0" as *const u8 as *const libc::c_char,
    b"MT_YELLOWSPRING\0" as *const u8 as *const libc::c_char,
    b"MT_REDSPRING\0" as *const u8 as *const libc::c_char,
    b"MT_BLUESPRING\0" as *const u8 as *const libc::c_char,
    b"MT_YELLOWDIAG\0" as *const u8 as *const libc::c_char,
    b"MT_REDDIAG\0" as *const u8 as *const libc::c_char,
    b"MT_BLUEDIAG\0" as *const u8 as *const libc::c_char,
    b"MT_YELLOWHORIZ\0" as *const u8 as *const libc::c_char,
    b"MT_REDHORIZ\0" as *const u8 as *const libc::c_char,
    b"MT_BLUEHORIZ\0" as *const u8 as *const libc::c_char,
    b"MT_BOOSTERSEG\0" as *const u8 as *const libc::c_char,
    b"MT_BOOSTERROLLER\0" as *const u8 as *const libc::c_char,
    b"MT_YELLOWBOOSTER\0" as *const u8 as *const libc::c_char,
    b"MT_REDBOOSTER\0" as *const u8 as *const libc::c_char,
    b"MT_BUBBLES\0" as *const u8 as *const libc::c_char,
    b"MT_SIGN\0" as *const u8 as *const libc::c_char,
    b"MT_SPIKEBALL\0" as *const u8 as *const libc::c_char,
    b"MT_SPINFIRE\0" as *const u8 as *const libc::c_char,
    b"MT_SPIKE\0" as *const u8 as *const libc::c_char,
    b"MT_WALLSPIKE\0" as *const u8 as *const libc::c_char,
    b"MT_WALLSPIKEBASE\0" as *const u8 as *const libc::c_char,
    b"MT_STARPOST\0" as *const u8 as *const libc::c_char,
    b"MT_BIGMINE\0" as *const u8 as *const libc::c_char,
    b"MT_BLASTEXECUTOR\0" as *const u8 as *const libc::c_char,
    b"MT_CANNONLAUNCHER\0" as *const u8 as *const libc::c_char,
    b"MT_BOXSPARKLE\0" as *const u8 as *const libc::c_char,
    b"MT_RING_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_PITY_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_ATTRACT_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_FORCE_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_ARMAGEDDON_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_WHIRLWIND_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_ELEMENTAL_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_SNEAKERS_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_INVULN_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_1UP_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_EGGMAN_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_MIXUP_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_MYSTERY_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_GRAVITY_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_RECYCLER_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_SCORE1K_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_SCORE10K_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_FLAMEAURA_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_BUBBLEWRAP_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_THUNDERCOIN_BOX\0" as *const u8 as *const libc::c_char,
    b"MT_PITY_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"MT_ATTRACT_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"MT_FORCE_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"MT_ARMAGEDDON_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"MT_WHIRLWIND_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"MT_ELEMENTAL_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"MT_SNEAKERS_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"MT_INVULN_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"MT_EGGMAN_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"MT_GRAVITY_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"MT_FLAMEAURA_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"MT_BUBBLEWRAP_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"MT_THUNDERCOIN_GOLDBOX\0" as *const u8 as *const libc::c_char,
    b"MT_RING_REDBOX\0" as *const u8 as *const libc::c_char,
    b"MT_RING_BLUEBOX\0" as *const u8 as *const libc::c_char,
    b"MT_RING_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_PITY_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_ATTRACT_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_FORCE_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_ARMAGEDDON_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_WHIRLWIND_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_ELEMENTAL_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_SNEAKERS_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_INVULN_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_1UP_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_EGGMAN_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_MIXUP_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_GRAVITY_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_RECYCLER_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_SCORE1K_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_SCORE10K_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_FLAMEAURA_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_BUBBLEWRAP_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_THUNDERCOIN_ICON\0" as *const u8 as *const libc::c_char,
    b"MT_ROCKET\0" as *const u8 as *const libc::c_char,
    b"MT_LASER\0" as *const u8 as *const libc::c_char,
    b"MT_TORPEDO\0" as *const u8 as *const libc::c_char,
    b"MT_TORPEDO2\0" as *const u8 as *const libc::c_char,
    b"MT_ENERGYBALL\0" as *const u8 as *const libc::c_char,
    b"MT_MINE\0" as *const u8 as *const libc::c_char,
    b"MT_JETTBULLET\0" as *const u8 as *const libc::c_char,
    b"MT_TURRETLASER\0" as *const u8 as *const libc::c_char,
    b"MT_CANNONBALL\0" as *const u8 as *const libc::c_char,
    b"MT_CANNONBALLDECOR\0" as *const u8 as *const libc::c_char,
    b"MT_ARROW\0" as *const u8 as *const libc::c_char,
    b"MT_DEMONFIRE\0" as *const u8 as *const libc::c_char,
    b"MT_LETTER\0" as *const u8 as *const libc::c_char,
    b"MT_TUTORIALPLANT\0" as *const u8 as *const libc::c_char,
    b"MT_TUTORIALLEAF\0" as *const u8 as *const libc::c_char,
    b"MT_TUTORIALFLOWER\0" as *const u8 as *const libc::c_char,
    b"MT_TUTORIALFLOWERF\0" as *const u8 as *const libc::c_char,
    b"MT_GFZFLOWER1\0" as *const u8 as *const libc::c_char,
    b"MT_GFZFLOWER2\0" as *const u8 as *const libc::c_char,
    b"MT_GFZFLOWER3\0" as *const u8 as *const libc::c_char,
    b"MT_BLUEBERRYBUSH\0" as *const u8 as *const libc::c_char,
    b"MT_BERRYBUSH\0" as *const u8 as *const libc::c_char,
    b"MT_BUSH\0" as *const u8 as *const libc::c_char,
    b"MT_GFZTREE\0" as *const u8 as *const libc::c_char,
    b"MT_GFZBERRYTREE\0" as *const u8 as *const libc::c_char,
    b"MT_GFZCHERRYTREE\0" as *const u8 as *const libc::c_char,
    b"MT_CHECKERTREE\0" as *const u8 as *const libc::c_char,
    b"MT_CHECKERSUNSETTREE\0" as *const u8 as *const libc::c_char,
    b"MT_FHZTREE\0" as *const u8 as *const libc::c_char,
    b"MT_FHZPINKTREE\0" as *const u8 as *const libc::c_char,
    b"MT_POLYGONTREE\0" as *const u8 as *const libc::c_char,
    b"MT_BUSHTREE\0" as *const u8 as *const libc::c_char,
    b"MT_BUSHREDTREE\0" as *const u8 as *const libc::c_char,
    b"MT_SPRINGTREE\0" as *const u8 as *const libc::c_char,
    b"MT_THZFLOWER1\0" as *const u8 as *const libc::c_char,
    b"MT_THZFLOWER2\0" as *const u8 as *const libc::c_char,
    b"MT_THZFLOWER3\0" as *const u8 as *const libc::c_char,
    b"MT_THZTREE\0" as *const u8 as *const libc::c_char,
    b"MT_THZTREEBRANCH\0" as *const u8 as *const libc::c_char,
    b"MT_ALARM\0" as *const u8 as *const libc::c_char,
    b"MT_GARGOYLE\0" as *const u8 as *const libc::c_char,
    b"MT_BIGGARGOYLE\0" as *const u8 as *const libc::c_char,
    b"MT_SEAWEED\0" as *const u8 as *const libc::c_char,
    b"MT_WATERDRIP\0" as *const u8 as *const libc::c_char,
    b"MT_WATERDROP\0" as *const u8 as *const libc::c_char,
    b"MT_CORAL1\0" as *const u8 as *const libc::c_char,
    b"MT_CORAL2\0" as *const u8 as *const libc::c_char,
    b"MT_CORAL3\0" as *const u8 as *const libc::c_char,
    b"MT_CORAL4\0" as *const u8 as *const libc::c_char,
    b"MT_CORAL5\0" as *const u8 as *const libc::c_char,
    b"MT_BLUECRYSTAL\0" as *const u8 as *const libc::c_char,
    b"MT_KELP\0" as *const u8 as *const libc::c_char,
    b"MT_ANIMALGAETOP\0" as *const u8 as *const libc::c_char,
    b"MT_ANIMALGAESEG\0" as *const u8 as *const libc::c_char,
    b"MT_DSZSTALAGMITE\0" as *const u8 as *const libc::c_char,
    b"MT_DSZ2STALAGMITE\0" as *const u8 as *const libc::c_char,
    b"MT_LIGHTBEAM\0" as *const u8 as *const libc::c_char,
    b"MT_CHAIN\0" as *const u8 as *const libc::c_char,
    b"MT_FLAME\0" as *const u8 as *const libc::c_char,
    b"MT_FLAMEPARTICLE\0" as *const u8 as *const libc::c_char,
    b"MT_EGGSTATUE\0" as *const u8 as *const libc::c_char,
    b"MT_MACEPOINT\0" as *const u8 as *const libc::c_char,
    b"MT_CHAINMACEPOINT\0" as *const u8 as *const libc::c_char,
    b"MT_SPRINGBALLPOINT\0" as *const u8 as *const libc::c_char,
    b"MT_CHAINPOINT\0" as *const u8 as *const libc::c_char,
    b"MT_HIDDEN_SLING\0" as *const u8 as *const libc::c_char,
    b"MT_FIREBARPOINT\0" as *const u8 as *const libc::c_char,
    b"MT_CUSTOMMACEPOINT\0" as *const u8 as *const libc::c_char,
    b"MT_SMALLMACECHAIN\0" as *const u8 as *const libc::c_char,
    b"MT_BIGMACECHAIN\0" as *const u8 as *const libc::c_char,
    b"MT_SMALLMACE\0" as *const u8 as *const libc::c_char,
    b"MT_BIGMACE\0" as *const u8 as *const libc::c_char,
    b"MT_SMALLGRABCHAIN\0" as *const u8 as *const libc::c_char,
    b"MT_BIGGRABCHAIN\0" as *const u8 as *const libc::c_char,
    b"MT_YELLOWSPRINGBALL\0" as *const u8 as *const libc::c_char,
    b"MT_REDSPRINGBALL\0" as *const u8 as *const libc::c_char,
    b"MT_SMALLFIREBAR\0" as *const u8 as *const libc::c_char,
    b"MT_BIGFIREBAR\0" as *const u8 as *const libc::c_char,
    b"MT_CEZFLOWER\0" as *const u8 as *const libc::c_char,
    b"MT_CEZPOLE1\0" as *const u8 as *const libc::c_char,
    b"MT_CEZPOLE2\0" as *const u8 as *const libc::c_char,
    b"MT_CEZBANNER1\0" as *const u8 as *const libc::c_char,
    b"MT_CEZBANNER2\0" as *const u8 as *const libc::c_char,
    b"MT_PINETREE\0" as *const u8 as *const libc::c_char,
    b"MT_CEZBUSH1\0" as *const u8 as *const libc::c_char,
    b"MT_CEZBUSH2\0" as *const u8 as *const libc::c_char,
    b"MT_CANDLE\0" as *const u8 as *const libc::c_char,
    b"MT_CANDLEPRICKET\0" as *const u8 as *const libc::c_char,
    b"MT_FLAMEHOLDER\0" as *const u8 as *const libc::c_char,
    b"MT_FIRETORCH\0" as *const u8 as *const libc::c_char,
    b"MT_WAVINGFLAG1\0" as *const u8 as *const libc::c_char,
    b"MT_WAVINGFLAG2\0" as *const u8 as *const libc::c_char,
    b"MT_WAVINGFLAGSEG1\0" as *const u8 as *const libc::c_char,
    b"MT_WAVINGFLAGSEG2\0" as *const u8 as *const libc::c_char,
    b"MT_CRAWLASTATUE\0" as *const u8 as *const libc::c_char,
    b"MT_FACESTABBERSTATUE\0" as *const u8 as *const libc::c_char,
    b"MT_SUSPICIOUSFACESTABBERSTATUE\0" as *const u8 as *const libc::c_char,
    b"MT_BRAMBLES\0" as *const u8 as *const libc::c_char,
    b"MT_BIGTUMBLEWEED\0" as *const u8 as *const libc::c_char,
    b"MT_LITTLETUMBLEWEED\0" as *const u8 as *const libc::c_char,
    b"MT_CACTI1\0" as *const u8 as *const libc::c_char,
    b"MT_CACTI2\0" as *const u8 as *const libc::c_char,
    b"MT_CACTI3\0" as *const u8 as *const libc::c_char,
    b"MT_CACTI4\0" as *const u8 as *const libc::c_char,
    b"MT_CACTI5\0" as *const u8 as *const libc::c_char,
    b"MT_CACTI6\0" as *const u8 as *const libc::c_char,
    b"MT_CACTI7\0" as *const u8 as *const libc::c_char,
    b"MT_CACTI8\0" as *const u8 as *const libc::c_char,
    b"MT_CACTI9\0" as *const u8 as *const libc::c_char,
    b"MT_CACTI10\0" as *const u8 as *const libc::c_char,
    b"MT_CACTI11\0" as *const u8 as *const libc::c_char,
    b"MT_CACTITINYSEG\0" as *const u8 as *const libc::c_char,
    b"MT_CACTISMALLSEG\0" as *const u8 as *const libc::c_char,
    b"MT_ARIDSIGN_CAUTION\0" as *const u8 as *const libc::c_char,
    b"MT_ARIDSIGN_CACTI\0" as *const u8 as *const libc::c_char,
    b"MT_ARIDSIGN_SHARPTURN\0" as *const u8 as *const libc::c_char,
    b"MT_OILLAMP\0" as *const u8 as *const libc::c_char,
    b"MT_TNTBARREL\0" as *const u8 as *const libc::c_char,
    b"MT_PROXIMITYTNT\0" as *const u8 as *const libc::c_char,
    b"MT_DUSTDEVIL\0" as *const u8 as *const libc::c_char,
    b"MT_DUSTLAYER\0" as *const u8 as *const libc::c_char,
    b"MT_ARIDDUST\0" as *const u8 as *const libc::c_char,
    b"MT_MINECART\0" as *const u8 as *const libc::c_char,
    b"MT_MINECARTSEG\0" as *const u8 as *const libc::c_char,
    b"MT_MINECARTSPAWNER\0" as *const u8 as *const libc::c_char,
    b"MT_MINECARTEND\0" as *const u8 as *const libc::c_char,
    b"MT_MINECARTENDSOLID\0" as *const u8 as *const libc::c_char,
    b"MT_MINECARTSIDEMARK\0" as *const u8 as *const libc::c_char,
    b"MT_MINECARTSPARK\0" as *const u8 as *const libc::c_char,
    b"MT_SALOONDOOR\0" as *const u8 as *const libc::c_char,
    b"MT_SALOONDOORCENTER\0" as *const u8 as *const libc::c_char,
    b"MT_TRAINCAMEOSPAWNER\0" as *const u8 as *const libc::c_char,
    b"MT_TRAINSEG\0" as *const u8 as *const libc::c_char,
    b"MT_TRAINDUSTSPAWNER\0" as *const u8 as *const libc::c_char,
    b"MT_TRAINSTEAMSPAWNER\0" as *const u8 as *const libc::c_char,
    b"MT_MINECARTSWITCHPOINT\0" as *const u8 as *const libc::c_char,
    b"MT_FLAMEJET\0" as *const u8 as *const libc::c_char,
    b"MT_VERTICALFLAMEJET\0" as *const u8 as *const libc::c_char,
    b"MT_FLAMEJETFLAME\0" as *const u8 as *const libc::c_char,
    b"MT_FJSPINAXISA\0" as *const u8 as *const libc::c_char,
    b"MT_FJSPINAXISB\0" as *const u8 as *const libc::c_char,
    b"MT_FLAMEJETFLAMEB\0" as *const u8 as *const libc::c_char,
    b"MT_LAVAFALL\0" as *const u8 as *const libc::c_char,
    b"MT_LAVAFALL_LAVA\0" as *const u8 as *const libc::c_char,
    b"MT_LAVAFALLROCK\0" as *const u8 as *const libc::c_char,
    b"MT_ROLLOUTSPAWN\0" as *const u8 as *const libc::c_char,
    b"MT_ROLLOUTROCK\0" as *const u8 as *const libc::c_char,
    b"MT_BIGFERNLEAF\0" as *const u8 as *const libc::c_char,
    b"MT_BIGFERN\0" as *const u8 as *const libc::c_char,
    b"MT_JUNGLEPALM\0" as *const u8 as *const libc::c_char,
    b"MT_TORCHFLOWER\0" as *const u8 as *const libc::c_char,
    b"MT_WALLVINE_LONG\0" as *const u8 as *const libc::c_char,
    b"MT_WALLVINE_SHORT\0" as *const u8 as *const libc::c_char,
    b"MT_GLAREGOYLE\0" as *const u8 as *const libc::c_char,
    b"MT_GLAREGOYLEUP\0" as *const u8 as *const libc::c_char,
    b"MT_GLAREGOYLEDOWN\0" as *const u8 as *const libc::c_char,
    b"MT_GLAREGOYLELONG\0" as *const u8 as *const libc::c_char,
    b"MT_TARGET\0" as *const u8 as *const libc::c_char,
    b"MT_GREENFLAME\0" as *const u8 as *const libc::c_char,
    b"MT_BLUEGARGOYLE\0" as *const u8 as *const libc::c_char,
    b"MT_STALAGMITE0\0" as *const u8 as *const libc::c_char,
    b"MT_STALAGMITE1\0" as *const u8 as *const libc::c_char,
    b"MT_STALAGMITE2\0" as *const u8 as *const libc::c_char,
    b"MT_STALAGMITE3\0" as *const u8 as *const libc::c_char,
    b"MT_STALAGMITE4\0" as *const u8 as *const libc::c_char,
    b"MT_STALAGMITE5\0" as *const u8 as *const libc::c_char,
    b"MT_STALAGMITE6\0" as *const u8 as *const libc::c_char,
    b"MT_STALAGMITE7\0" as *const u8 as *const libc::c_char,
    b"MT_STALAGMITE8\0" as *const u8 as *const libc::c_char,
    b"MT_STALAGMITE9\0" as *const u8 as *const libc::c_char,
    b"MT_XMASPOLE\0" as *const u8 as *const libc::c_char,
    b"MT_CANDYCANE\0" as *const u8 as *const libc::c_char,
    b"MT_SNOWMAN\0" as *const u8 as *const libc::c_char,
    b"MT_SNOWMANHAT\0" as *const u8 as *const libc::c_char,
    b"MT_LAMPPOST1\0" as *const u8 as *const libc::c_char,
    b"MT_LAMPPOST2\0" as *const u8 as *const libc::c_char,
    b"MT_HANGSTAR\0" as *const u8 as *const libc::c_char,
    b"MT_MISTLETOE\0" as *const u8 as *const libc::c_char,
    b"MT_XMASBLUEBERRYBUSH\0" as *const u8 as *const libc::c_char,
    b"MT_XMASBERRYBUSH\0" as *const u8 as *const libc::c_char,
    b"MT_XMASBUSH\0" as *const u8 as *const libc::c_char,
    b"MT_FHZICE1\0" as *const u8 as *const libc::c_char,
    b"MT_FHZICE2\0" as *const u8 as *const libc::c_char,
    b"MT_ROSY\0" as *const u8 as *const libc::c_char,
    b"MT_CDLHRT\0" as *const u8 as *const libc::c_char,
    b"MT_JACKO1\0" as *const u8 as *const libc::c_char,
    b"MT_JACKO2\0" as *const u8 as *const libc::c_char,
    b"MT_JACKO3\0" as *const u8 as *const libc::c_char,
    b"MT_HHZTREE_TOP\0" as *const u8 as *const libc::c_char,
    b"MT_HHZTREE_PART\0" as *const u8 as *const libc::c_char,
    b"MT_HHZSHROOM\0" as *const u8 as *const libc::c_char,
    b"MT_HHZGRASS\0" as *const u8 as *const libc::c_char,
    b"MT_HHZTENTACLE1\0" as *const u8 as *const libc::c_char,
    b"MT_HHZTENTACLE2\0" as *const u8 as *const libc::c_char,
    b"MT_HHZSTALAGMITE_TALL\0" as *const u8 as *const libc::c_char,
    b"MT_HHZSTALAGMITE_SHORT\0" as *const u8 as *const libc::c_char,
    b"MT_BSZTALLFLOWER_RED\0" as *const u8 as *const libc::c_char,
    b"MT_BSZTALLFLOWER_PURPLE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZTALLFLOWER_BLUE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZTALLFLOWER_CYAN\0" as *const u8 as *const libc::c_char,
    b"MT_BSZTALLFLOWER_YELLOW\0" as *const u8 as *const libc::c_char,
    b"MT_BSZTALLFLOWER_ORANGE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZFLOWER_RED\0" as *const u8 as *const libc::c_char,
    b"MT_BSZFLOWER_PURPLE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZFLOWER_BLUE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZFLOWER_CYAN\0" as *const u8 as *const libc::c_char,
    b"MT_BSZFLOWER_YELLOW\0" as *const u8 as *const libc::c_char,
    b"MT_BSZFLOWER_ORANGE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZSHORTFLOWER_RED\0" as *const u8 as *const libc::c_char,
    b"MT_BSZSHORTFLOWER_PURPLE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZSHORTFLOWER_BLUE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZSHORTFLOWER_CYAN\0" as *const u8 as *const libc::c_char,
    b"MT_BSZSHORTFLOWER_YELLOW\0" as *const u8 as *const libc::c_char,
    b"MT_BSZSHORTFLOWER_ORANGE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZTULIP_RED\0" as *const u8 as *const libc::c_char,
    b"MT_BSZTULIP_PURPLE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZTULIP_BLUE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZTULIP_CYAN\0" as *const u8 as *const libc::c_char,
    b"MT_BSZTULIP_YELLOW\0" as *const u8 as *const libc::c_char,
    b"MT_BSZTULIP_ORANGE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZCLUSTER_RED\0" as *const u8 as *const libc::c_char,
    b"MT_BSZCLUSTER_PURPLE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZCLUSTER_BLUE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZCLUSTER_CYAN\0" as *const u8 as *const libc::c_char,
    b"MT_BSZCLUSTER_YELLOW\0" as *const u8 as *const libc::c_char,
    b"MT_BSZCLUSTER_ORANGE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZBUSH_RED\0" as *const u8 as *const libc::c_char,
    b"MT_BSZBUSH_PURPLE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZBUSH_BLUE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZBUSH_CYAN\0" as *const u8 as *const libc::c_char,
    b"MT_BSZBUSH_YELLOW\0" as *const u8 as *const libc::c_char,
    b"MT_BSZBUSH_ORANGE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZVINE_RED\0" as *const u8 as *const libc::c_char,
    b"MT_BSZVINE_PURPLE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZVINE_BLUE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZVINE_CYAN\0" as *const u8 as *const libc::c_char,
    b"MT_BSZVINE_YELLOW\0" as *const u8 as *const libc::c_char,
    b"MT_BSZVINE_ORANGE\0" as *const u8 as *const libc::c_char,
    b"MT_BSZSHRUB\0" as *const u8 as *const libc::c_char,
    b"MT_BSZCLOVER\0" as *const u8 as *const libc::c_char,
    b"MT_BIG_PALMTREE_TRUNK\0" as *const u8 as *const libc::c_char,
    b"MT_BIG_PALMTREE_TOP\0" as *const u8 as *const libc::c_char,
    b"MT_PALMTREE_TRUNK\0" as *const u8 as *const libc::c_char,
    b"MT_PALMTREE_TOP\0" as *const u8 as *const libc::c_char,
    b"MT_DBALL\0" as *const u8 as *const libc::c_char,
    b"MT_EGGSTATUE2\0" as *const u8 as *const libc::c_char,
    b"MT_ELEMENTAL_ORB\0" as *const u8 as *const libc::c_char,
    b"MT_ATTRACT_ORB\0" as *const u8 as *const libc::c_char,
    b"MT_FORCE_ORB\0" as *const u8 as *const libc::c_char,
    b"MT_ARMAGEDDON_ORB\0" as *const u8 as *const libc::c_char,
    b"MT_WHIRLWIND_ORB\0" as *const u8 as *const libc::c_char,
    b"MT_PITY_ORB\0" as *const u8 as *const libc::c_char,
    b"MT_FLAMEAURA_ORB\0" as *const u8 as *const libc::c_char,
    b"MT_BUBBLEWRAP_ORB\0" as *const u8 as *const libc::c_char,
    b"MT_THUNDERCOIN_ORB\0" as *const u8 as *const libc::c_char,
    b"MT_THUNDERCOIN_SPARK\0" as *const u8 as *const libc::c_char,
    b"MT_IVSP\0" as *const u8 as *const libc::c_char,
    b"MT_SUPERSPARK\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_01\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_01_CENTER\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_02\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_02_CENTER\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_03\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_03_CENTER\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_04\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_04_CENTER\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_05\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_05_CENTER\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_06\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_06_CENTER\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_07\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_07_CENTER\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_08\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_08_CENTER\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_09\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_09_CENTER\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_10\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_10_CENTER\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_11\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_11_CENTER\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_12\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_12_CENTER\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_13\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_13_CENTER\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_14\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_14_CENTER\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_15\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_15_CENTER\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_16\0" as *const u8 as *const libc::c_char,
    b"MT_FLICKY_16_CENTER\0" as *const u8 as *const libc::c_char,
    b"MT_SECRETFLICKY_01\0" as *const u8 as *const libc::c_char,
    b"MT_SECRETFLICKY_01_CENTER\0" as *const u8 as *const libc::c_char,
    b"MT_SECRETFLICKY_02\0" as *const u8 as *const libc::c_char,
    b"MT_SECRETFLICKY_02_CENTER\0" as *const u8 as *const libc::c_char,
    b"MT_SEED\0" as *const u8 as *const libc::c_char,
    b"MT_RAIN\0" as *const u8 as *const libc::c_char,
    b"MT_SNOWFLAKE\0" as *const u8 as *const libc::c_char,
    b"MT_SPLISH\0" as *const u8 as *const libc::c_char,
    b"MT_LAVASPLISH\0" as *const u8 as *const libc::c_char,
    b"MT_SMOKE\0" as *const u8 as *const libc::c_char,
    b"MT_SMALLBUBBLE\0" as *const u8 as *const libc::c_char,
    b"MT_MEDIUMBUBBLE\0" as *const u8 as *const libc::c_char,
    b"MT_EXTRALARGEBUBBLE\0" as *const u8 as *const libc::c_char,
    b"MT_WATERZAP\0" as *const u8 as *const libc::c_char,
    b"MT_SPINDUST\0" as *const u8 as *const libc::c_char,
    b"MT_TFOG\0" as *const u8 as *const libc::c_char,
    b"MT_PARTICLE\0" as *const u8 as *const libc::c_char,
    b"MT_PARTICLEGEN\0" as *const u8 as *const libc::c_char,
    b"MT_SCORE\0" as *const u8 as *const libc::c_char,
    b"MT_DROWNNUMBERS\0" as *const u8 as *const libc::c_char,
    b"MT_GOTEMERALD\0" as *const u8 as *const libc::c_char,
    b"MT_LOCKON\0" as *const u8 as *const libc::c_char,
    b"MT_LOCKONINF\0" as *const u8 as *const libc::c_char,
    b"MT_TAG\0" as *const u8 as *const libc::c_char,
    b"MT_GOTFLAG\0" as *const u8 as *const libc::c_char,
    b"MT_FINISHFLAG\0" as *const u8 as *const libc::c_char,
    b"MT_AMBIENT\0" as *const u8 as *const libc::c_char,
    b"MT_CORK\0" as *const u8 as *const libc::c_char,
    b"MT_LHRT\0" as *const u8 as *const libc::c_char,
    b"MT_REDRING\0" as *const u8 as *const libc::c_char,
    b"MT_BOUNCERING\0" as *const u8 as *const libc::c_char,
    b"MT_RAILRING\0" as *const u8 as *const libc::c_char,
    b"MT_INFINITYRING\0" as *const u8 as *const libc::c_char,
    b"MT_AUTOMATICRING\0" as *const u8 as *const libc::c_char,
    b"MT_EXPLOSIONRING\0" as *const u8 as *const libc::c_char,
    b"MT_SCATTERRING\0" as *const u8 as *const libc::c_char,
    b"MT_GRENADERING\0" as *const u8 as *const libc::c_char,
    b"MT_BOUNCEPICKUP\0" as *const u8 as *const libc::c_char,
    b"MT_RAILPICKUP\0" as *const u8 as *const libc::c_char,
    b"MT_AUTOPICKUP\0" as *const u8 as *const libc::c_char,
    b"MT_EXPLODEPICKUP\0" as *const u8 as *const libc::c_char,
    b"MT_SCATTERPICKUP\0" as *const u8 as *const libc::c_char,
    b"MT_GRENADEPICKUP\0" as *const u8 as *const libc::c_char,
    b"MT_THROWNBOUNCE\0" as *const u8 as *const libc::c_char,
    b"MT_THROWNINFINITY\0" as *const u8 as *const libc::c_char,
    b"MT_THROWNAUTOMATIC\0" as *const u8 as *const libc::c_char,
    b"MT_THROWNSCATTER\0" as *const u8 as *const libc::c_char,
    b"MT_THROWNEXPLOSION\0" as *const u8 as *const libc::c_char,
    b"MT_THROWNGRENADE\0" as *const u8 as *const libc::c_char,
    b"MT_COIN\0" as *const u8 as *const libc::c_char,
    b"MT_FLINGCOIN\0" as *const u8 as *const libc::c_char,
    b"MT_GOOMBA\0" as *const u8 as *const libc::c_char,
    b"MT_BLUEGOOMBA\0" as *const u8 as *const libc::c_char,
    b"MT_FIREFLOWER\0" as *const u8 as *const libc::c_char,
    b"MT_FIREBALL\0" as *const u8 as *const libc::c_char,
    b"MT_FIREBALLTRAIL\0" as *const u8 as *const libc::c_char,
    b"MT_SHELL\0" as *const u8 as *const libc::c_char,
    b"MT_PUMA\0" as *const u8 as *const libc::c_char,
    b"MT_PUMATRAIL\0" as *const u8 as *const libc::c_char,
    b"MT_HAMMER\0" as *const u8 as *const libc::c_char,
    b"MT_KOOPA\0" as *const u8 as *const libc::c_char,
    b"MT_KOOPAFLAME\0" as *const u8 as *const libc::c_char,
    b"MT_AXE\0" as *const u8 as *const libc::c_char,
    b"MT_MARIOBUSH1\0" as *const u8 as *const libc::c_char,
    b"MT_MARIOBUSH2\0" as *const u8 as *const libc::c_char,
    b"MT_TOAD\0" as *const u8 as *const libc::c_char,
    b"MT_AXIS\0" as *const u8 as *const libc::c_char,
    b"MT_AXISTRANSFER\0" as *const u8 as *const libc::c_char,
    b"MT_AXISTRANSFERLINE\0" as *const u8 as *const libc::c_char,
    b"MT_NIGHTSDRONE\0" as *const u8 as *const libc::c_char,
    b"MT_NIGHTSDRONE_MAN\0" as *const u8 as *const libc::c_char,
    b"MT_NIGHTSDRONE_SPARKLING\0" as *const u8 as *const libc::c_char,
    b"MT_NIGHTSDRONE_GOAL\0" as *const u8 as *const libc::c_char,
    b"MT_NIGHTSPARKLE\0" as *const u8 as *const libc::c_char,
    b"MT_NIGHTSLOOPHELPER\0" as *const u8 as *const libc::c_char,
    b"MT_NIGHTSBUMPER\0" as *const u8 as *const libc::c_char,
    b"MT_HOOP\0" as *const u8 as *const libc::c_char,
    b"MT_HOOPCOLLIDE\0" as *const u8 as *const libc::c_char,
    b"MT_HOOPCENTER\0" as *const u8 as *const libc::c_char,
    b"MT_NIGHTSCORE\0" as *const u8 as *const libc::c_char,
    b"MT_NIGHTSCHIP\0" as *const u8 as *const libc::c_char,
    b"MT_FLINGNIGHTSCHIP\0" as *const u8 as *const libc::c_char,
    b"MT_NIGHTSSTAR\0" as *const u8 as *const libc::c_char,
    b"MT_FLINGNIGHTSSTAR\0" as *const u8 as *const libc::c_char,
    b"MT_NIGHTSSUPERLOOP\0" as *const u8 as *const libc::c_char,
    b"MT_NIGHTSDRILLREFILL\0" as *const u8 as *const libc::c_char,
    b"MT_NIGHTSHELPER\0" as *const u8 as *const libc::c_char,
    b"MT_NIGHTSEXTRATIME\0" as *const u8 as *const libc::c_char,
    b"MT_NIGHTSLINKFREEZE\0" as *const u8 as *const libc::c_char,
    b"MT_EGGCAPSULE\0" as *const u8 as *const libc::c_char,
    b"MT_IDEYAANCHOR\0" as *const u8 as *const libc::c_char,
    b"MT_NIGHTOPIANHELPER\0" as *const u8 as *const libc::c_char,
    b"MT_PIAN\0" as *const u8 as *const libc::c_char,
    b"MT_SHLEEP\0" as *const u8 as *const libc::c_char,
    b"MT_PENGUINATOR\0" as *const u8 as *const libc::c_char,
    b"MT_POPHAT\0" as *const u8 as *const libc::c_char,
    b"MT_POPSHOT\0" as *const u8 as *const libc::c_char,
    b"MT_POPSHOT_TRAIL\0" as *const u8 as *const libc::c_char,
    b"MT_HIVEELEMENTAL\0" as *const u8 as *const libc::c_char,
    b"MT_BUMBLEBORE\0" as *const u8 as *const libc::c_char,
    b"MT_BUGGLE\0" as *const u8 as *const libc::c_char,
    b"MT_SMASHINGSPIKEBALL\0" as *const u8 as *const libc::c_char,
    b"MT_CACOLANTERN\0" as *const u8 as *const libc::c_char,
    b"MT_CACOSHARD\0" as *const u8 as *const libc::c_char,
    b"MT_CACOFIRE\0" as *const u8 as *const libc::c_char,
    b"MT_SPINBOBERT\0" as *const u8 as *const libc::c_char,
    b"MT_SPINBOBERT_FIRE1\0" as *const u8 as *const libc::c_char,
    b"MT_SPINBOBERT_FIRE2\0" as *const u8 as *const libc::c_char,
    b"MT_HANGSTER\0" as *const u8 as *const libc::c_char,
    b"MT_TELEPORTMAN\0" as *const u8 as *const libc::c_char,
    b"MT_ALTVIEWMAN\0" as *const u8 as *const libc::c_char,
    b"MT_CRUMBLEOBJ\0" as *const u8 as *const libc::c_char,
    b"MT_TUBEWAYPOINT\0" as *const u8 as *const libc::c_char,
    b"MT_PUSH\0" as *const u8 as *const libc::c_char,
    b"MT_GHOST\0" as *const u8 as *const libc::c_char,
    b"MT_OVERLAY\0" as *const u8 as *const libc::c_char,
    b"MT_ANGLEMAN\0" as *const u8 as *const libc::c_char,
    b"MT_POLYANCHOR\0" as *const u8 as *const libc::c_char,
    b"MT_POLYSPAWN\0" as *const u8 as *const libc::c_char,
    b"MT_SKYBOX\0" as *const u8 as *const libc::c_char,
    b"MT_SPARK\0" as *const u8 as *const libc::c_char,
    b"MT_EXPLODE\0" as *const u8 as *const libc::c_char,
    b"MT_UWEXPLODE\0" as *const u8 as *const libc::c_char,
    b"MT_DUST\0" as *const u8 as *const libc::c_char,
    b"MT_ROCKSPAWNER\0" as *const u8 as *const libc::c_char,
    b"MT_FALLINGROCK\0" as *const u8 as *const libc::c_char,
    b"MT_ROCKCRUMBLE1\0" as *const u8 as *const libc::c_char,
    b"MT_ROCKCRUMBLE2\0" as *const u8 as *const libc::c_char,
    b"MT_ROCKCRUMBLE3\0" as *const u8 as *const libc::c_char,
    b"MT_ROCKCRUMBLE4\0" as *const u8 as *const libc::c_char,
    b"MT_ROCKCRUMBLE5\0" as *const u8 as *const libc::c_char,
    b"MT_ROCKCRUMBLE6\0" as *const u8 as *const libc::c_char,
    b"MT_ROCKCRUMBLE7\0" as *const u8 as *const libc::c_char,
    b"MT_ROCKCRUMBLE8\0" as *const u8 as *const libc::c_char,
    b"MT_ROCKCRUMBLE9\0" as *const u8 as *const libc::c_char,
    b"MT_ROCKCRUMBLE10\0" as *const u8 as *const libc::c_char,
    b"MT_ROCKCRUMBLE11\0" as *const u8 as *const libc::c_char,
    b"MT_ROCKCRUMBLE12\0" as *const u8 as *const libc::c_char,
    b"MT_ROCKCRUMBLE13\0" as *const u8 as *const libc::c_char,
    b"MT_ROCKCRUMBLE14\0" as *const u8 as *const libc::c_char,
    b"MT_ROCKCRUMBLE15\0" as *const u8 as *const libc::c_char,
    b"MT_ROCKCRUMBLE16\0" as *const u8 as *const libc::c_char,
    b"MT_GFZDEBRIS\0" as *const u8 as *const libc::c_char,
    b"MT_BRICKDEBRIS\0" as *const u8 as *const libc::c_char,
    b"MT_WOODDEBRIS\0" as *const u8 as *const libc::c_char,
    b"MT_REDBRICKDEBRIS\0" as *const u8 as *const libc::c_char,
    b"MT_BLUEBRICKDEBRIS\0" as *const u8 as *const libc::c_char,
    b"MT_YELLOWBRICKDEBRIS\0" as *const u8 as *const libc::c_char,
    b"MT_NAMECHECK\0" as *const u8 as *const libc::c_char,
    b"MT_RAY\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut MOBJFLAG_LIST: [*const libc::c_char; 31] = [
    b"SPECIAL\0" as *const u8 as *const libc::c_char,
    b"SOLID\0" as *const u8 as *const libc::c_char,
    b"SHOOTABLE\0" as *const u8 as *const libc::c_char,
    b"NOSECTOR\0" as *const u8 as *const libc::c_char,
    b"NOBLOCKMAP\0" as *const u8 as *const libc::c_char,
    b"PAPERCOLLISION\0" as *const u8 as *const libc::c_char,
    b"PUSHABLE\0" as *const u8 as *const libc::c_char,
    b"BOSS\0" as *const u8 as *const libc::c_char,
    b"SPAWNCEILING\0" as *const u8 as *const libc::c_char,
    b"NOGRAVITY\0" as *const u8 as *const libc::c_char,
    b"AMBIENT\0" as *const u8 as *const libc::c_char,
    b"SLIDEME\0" as *const u8 as *const libc::c_char,
    b"NOCLIP\0" as *const u8 as *const libc::c_char,
    b"FLOAT\0" as *const u8 as *const libc::c_char,
    b"BOXICON\0" as *const u8 as *const libc::c_char,
    b"MISSILE\0" as *const u8 as *const libc::c_char,
    b"SPRING\0" as *const u8 as *const libc::c_char,
    b"BOUNCE\0" as *const u8 as *const libc::c_char,
    b"MONITOR\0" as *const u8 as *const libc::c_char,
    b"NOTHINK\0" as *const u8 as *const libc::c_char,
    b"FIRE\0" as *const u8 as *const libc::c_char,
    b"NOCLIPHEIGHT\0" as *const u8 as *const libc::c_char,
    b"ENEMY\0" as *const u8 as *const libc::c_char,
    b"SCENERY\0" as *const u8 as *const libc::c_char,
    b"PAIN\0" as *const u8 as *const libc::c_char,
    b"STICKY\0" as *const u8 as *const libc::c_char,
    b"NIGHTSITEM\0" as *const u8 as *const libc::c_char,
    b"NOCLIPTHING\0" as *const u8 as *const libc::c_char,
    b"GRENADEBOUNCE\0" as *const u8 as *const libc::c_char,
    b"RUNSPAWNFUNC\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut MOBJFLAG2_LIST: [*const libc::c_char; 32] = [
    b"AXIS\0" as *const u8 as *const libc::c_char,
    b"TWOD\0" as *const u8 as *const libc::c_char,
    b"DONTRESPAWN\0" as *const u8 as *const libc::c_char,
    b"DONTDRAW\0" as *const u8 as *const libc::c_char,
    b"AUTOMATIC\0" as *const u8 as *const libc::c_char,
    b"RAILRING\0" as *const u8 as *const libc::c_char,
    b"BOUNCERING\0" as *const u8 as *const libc::c_char,
    b"EXPLOSION\0" as *const u8 as *const libc::c_char,
    b"SCATTER\0" as *const u8 as *const libc::c_char,
    b"BEYONDTHEGRAVE\0" as *const u8 as *const libc::c_char,
    b"SLIDEPUSH\0" as *const u8 as *const libc::c_char,
    b"CLASSICPUSH\0" as *const u8 as *const libc::c_char,
    b"INVERTAIMABLE\0" as *const u8 as *const libc::c_char,
    b"INFLOAT\0" as *const u8 as *const libc::c_char,
    b"DEBRIS\0" as *const u8 as *const libc::c_char,
    b"NIGHTSPULL\0" as *const u8 as *const libc::c_char,
    b"JUSTATTACKED\0" as *const u8 as *const libc::c_char,
    b"FIRING\0" as *const u8 as *const libc::c_char,
    b"SUPERFIRE\0" as *const u8 as *const libc::c_char,
    b"SHADOW\0" as *const u8 as *const libc::c_char,
    b"STRONGBOX\0" as *const u8 as *const libc::c_char,
    b"OBJECTFLIP\0" as *const u8 as *const libc::c_char,
    b"SKULLFLY\0" as *const u8 as *const libc::c_char,
    b"FRET\0" as *const u8 as *const libc::c_char,
    b"BOSSNOTRAP\0" as *const u8 as *const libc::c_char,
    b"BOSSFLEE\0" as *const u8 as *const libc::c_char,
    b"BOSSDEAD\0" as *const u8 as *const libc::c_char,
    b"AMBUSH\0" as *const u8 as *const libc::c_char,
    b"LINKDRAW\0" as *const u8 as *const libc::c_char,
    b"SHIELD\0" as *const u8 as *const libc::c_char,
    b"SPLAT\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut MOBJEFLAG_LIST: [*const libc::c_char; 15] = [
    b"ONGROUND\0" as *const u8 as *const libc::c_char,
    b"JUSTHITFLOOR\0" as *const u8 as *const libc::c_char,
    b"TOUCHWATER\0" as *const u8 as *const libc::c_char,
    b"UNDERWATER\0" as *const u8 as *const libc::c_char,
    b"JUSTSTEPPEDDOWN\0" as *const u8 as *const libc::c_char,
    b"VERTICALFLIP\0" as *const u8 as *const libc::c_char,
    b"GOOWATER\0" as *const u8 as *const libc::c_char,
    b"TOUCHLAVA\0" as *const u8 as *const libc::c_char,
    b"PUSHED\0" as *const u8 as *const libc::c_char,
    b"SPRUNG\0" as *const u8 as *const libc::c_char,
    b"APPLYPMOMZ\0" as *const u8 as *const libc::c_char,
    b"TRACERANGLE\0" as *const u8 as *const libc::c_char,
    b"FORCESUPER\0" as *const u8 as *const libc::c_char,
    b"FORCENOSUPER\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut MAPTHINGFLAG_LIST: [*const libc::c_char; 5] = [
    b"EXTRA\0" as *const u8 as *const libc::c_char,
    b"OBJECTFLIP\0" as *const u8 as *const libc::c_char,
    b"OBJECTSPECIAL\0" as *const u8 as *const libc::c_char,
    b"AMBUSH\0" as *const u8 as *const libc::c_char,
    b"ABSOLUTEZ\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut PLAYERFLAG_LIST: [*const libc::c_char; 32] = [
    b"FLIPCAM\0" as *const u8 as *const libc::c_char,
    b"ANALOGMODE\0" as *const u8 as *const libc::c_char,
    b"DIRECTIONCHAR\0" as *const u8 as *const libc::c_char,
    b"AUTOBRAKE\0" as *const u8 as *const libc::c_char,
    b"GODMODE\0" as *const u8 as *const libc::c_char,
    b"NOCLIP\0" as *const u8 as *const libc::c_char,
    b"INVIS\0" as *const u8 as *const libc::c_char,
    b"ATTACKDOWN\0" as *const u8 as *const libc::c_char,
    b"SPINDOWN\0" as *const u8 as *const libc::c_char,
    b"JUMPDOWN\0" as *const u8 as *const libc::c_char,
    b"WPNDOWN\0" as *const u8 as *const libc::c_char,
    b"STASIS\0" as *const u8 as *const libc::c_char,
    b"JUMPSTASIS\0" as *const u8 as *const libc::c_char,
    b"APPLYAUTOBRAKE\0" as *const u8 as *const libc::c_char,
    b"STARTJUMP\0" as *const u8 as *const libc::c_char,
    b"JUMPED\0" as *const u8 as *const libc::c_char,
    b"NOJUMPDAMAGE\0" as *const u8 as *const libc::c_char,
    b"SPINNING\0" as *const u8 as *const libc::c_char,
    b"STARTDASH\0" as *const u8 as *const libc::c_char,
    b"THOKKED\0" as *const u8 as *const libc::c_char,
    b"SHIELDABILITY\0" as *const u8 as *const libc::c_char,
    b"GLIDING\0" as *const u8 as *const libc::c_char,
    b"BOUNCING\0" as *const u8 as *const libc::c_char,
    b"SLIDING\0" as *const u8 as *const libc::c_char,
    b"TRANSFERTOCLOSEST\0" as *const u8 as *const libc::c_char,
    b"DRILLING\0" as *const u8 as *const libc::c_char,
    b"GAMETYPEOVER\0" as *const u8 as *const libc::c_char,
    b"TAGIT\0" as *const u8 as *const libc::c_char,
    b"FORCESTRAFE\0" as *const u8 as *const libc::c_char,
    b"CANCARRY\0" as *const u8 as *const libc::c_char,
    b"FINISHED\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut GAMETYPERULE_LIST: [*const libc::c_char; 33] = [
    b"CAMPAIGN\0" as *const u8 as *const libc::c_char,
    b"RINGSLINGER\0" as *const u8 as *const libc::c_char,
    b"SPECTATORS\0" as *const u8 as *const libc::c_char,
    b"LIVES\0" as *const u8 as *const libc::c_char,
    b"TEAMS\0" as *const u8 as *const libc::c_char,
    b"FIRSTPERSON\0" as *const u8 as *const libc::c_char,
    b"POWERSTONES\0" as *const u8 as *const libc::c_char,
    b"TEAMFLAGS\0" as *const u8 as *const libc::c_char,
    b"FRIENDLY\0" as *const u8 as *const libc::c_char,
    b"SPECIALSTAGES\0" as *const u8 as *const libc::c_char,
    b"EMERALDTOKENS\0" as *const u8 as *const libc::c_char,
    b"EMERALDHUNT\0" as *const u8 as *const libc::c_char,
    b"RACE\0" as *const u8 as *const libc::c_char,
    b"TAG\0" as *const u8 as *const libc::c_char,
    b"POINTLIMIT\0" as *const u8 as *const libc::c_char,
    b"TIMELIMIT\0" as *const u8 as *const libc::c_char,
    b"OVERTIME\0" as *const u8 as *const libc::c_char,
    b"HURTMESSAGES\0" as *const u8 as *const libc::c_char,
    b"FRIENDLYFIRE\0" as *const u8 as *const libc::c_char,
    b"STARTCOUNTDOWN\0" as *const u8 as *const libc::c_char,
    b"HIDEFROZEN\0" as *const u8 as *const libc::c_char,
    b"BLINDFOLDED\0" as *const u8 as *const libc::c_char,
    b"RESPAWNDELAY\0" as *const u8 as *const libc::c_char,
    b"PITYSHIELD\0" as *const u8 as *const libc::c_char,
    b"DEATHPENALTY\0" as *const u8 as *const libc::c_char,
    b"NOSPECTATORSPAWN\0" as *const u8 as *const libc::c_char,
    b"DEATHMATCHSTARTS\0" as *const u8 as *const libc::c_char,
    b"SPAWNINVUL\0" as *const u8 as *const libc::c_char,
    b"SPAWNENEMIES\0" as *const u8 as *const libc::c_char,
    b"ALLOWEXIT\0" as *const u8 as *const libc::c_char,
    b"NOTITLECARD\0" as *const u8 as *const libc::c_char,
    b"CUTSCENES\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut ML_LIST: [*const libc::c_char; 17] = [
    b"IMPASSIBLE\0" as *const u8 as *const libc::c_char,
    b"BLOCKMONSTERS\0" as *const u8 as *const libc::c_char,
    b"TWOSIDED\0" as *const u8 as *const libc::c_char,
    b"DONTPEGTOP\0" as *const u8 as *const libc::c_char,
    b"DONTPEGBOTTOM\0" as *const u8 as *const libc::c_char,
    b"SKEWTD\0" as *const u8 as *const libc::c_char,
    b"NOCLIMB\0" as *const u8 as *const libc::c_char,
    b"NOSKEW\0" as *const u8 as *const libc::c_char,
    b"MIDPEG\0" as *const u8 as *const libc::c_char,
    b"MIDSOLID\0" as *const u8 as *const libc::c_char,
    b"WRAPMIDTEX\0" as *const u8 as *const libc::c_char,
    b"NETONLY\0" as *const u8 as *const libc::c_char,
    b"NONET\0" as *const u8 as *const libc::c_char,
    b"EFFECT6\0" as *const u8 as *const libc::c_char,
    b"BOUNCY\0" as *const u8 as *const libc::c_char,
    b"TFERLINE\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut MSF_LIST: [*const libc::c_char; 10] = [
    b"FLIPSPECIAL_FLOOR\0" as *const u8 as *const libc::c_char,
    b"FLIPSPECIAL_CEILING\0" as *const u8 as *const libc::c_char,
    b"TRIGGERSPECIAL_TOUCH\0" as *const u8 as *const libc::c_char,
    b"TRIGGERSPECIAL_HEADBUMP\0" as *const u8 as *const libc::c_char,
    b"TRIGGERLINE_PLANE\0" as *const u8 as *const libc::c_char,
    b"TRIGGERLINE_MOBJ\0" as *const u8 as *const libc::c_char,
    b"GRAVITYFLIP\0" as *const u8 as *const libc::c_char,
    b"HEATWAVE\0" as *const u8 as *const libc::c_char,
    b"NOCLIPCAMERA\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut SSF_LIST: [*const libc::c_char; 22] = [
    b"OUTERSPACE\0" as *const u8 as *const libc::c_char,
    b"DOUBLESTEPUP\0" as *const u8 as *const libc::c_char,
    b"NOSTEPDOWN\0" as *const u8 as *const libc::c_char,
    b"WINDCURRENT\0" as *const u8 as *const libc::c_char,
    b"CONVEYOR\0" as *const u8 as *const libc::c_char,
    b"SPEEDPAD\0" as *const u8 as *const libc::c_char,
    b"STARPOSTACTIVATOR\0" as *const u8 as *const libc::c_char,
    b"EXIT\0" as *const u8 as *const libc::c_char,
    b"SPECIALSTAGEPIT\0" as *const u8 as *const libc::c_char,
    b"RETURNFLAG\0" as *const u8 as *const libc::c_char,
    b"REDTEAMBASE\0" as *const u8 as *const libc::c_char,
    b"BLUETEAMBASE\0" as *const u8 as *const libc::c_char,
    b"FAN\0" as *const u8 as *const libc::c_char,
    b"SUPERTRANSFORM\0" as *const u8 as *const libc::c_char,
    b"FORCESPIN\0" as *const u8 as *const libc::c_char,
    b"ZOOMTUBESTART\0" as *const u8 as *const libc::c_char,
    b"ZOOMTUBEEND\0" as *const u8 as *const libc::c_char,
    b"FINISHLINE\0" as *const u8 as *const libc::c_char,
    b"ROPEHANG\0" as *const u8 as *const libc::c_char,
    b"JUMPFLIP\0" as *const u8 as *const libc::c_char,
    b"GRAVITYOVERRIDE\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut SD_LIST: [*const libc::c_char; 12] = [
    b"NONE\0" as *const u8 as *const libc::c_char,
    b"GENERIC\0" as *const u8 as *const libc::c_char,
    b"WATER\0" as *const u8 as *const libc::c_char,
    b"FIRE\0" as *const u8 as *const libc::c_char,
    b"LAVA\0" as *const u8 as *const libc::c_char,
    b"ELECTRIC\0" as *const u8 as *const libc::c_char,
    b"SPIKE\0" as *const u8 as *const libc::c_char,
    b"DEATHPITTILT\0" as *const u8 as *const libc::c_char,
    b"DEATHPITNOTILT\0" as *const u8 as *const libc::c_char,
    b"INSTAKILL\0" as *const u8 as *const libc::c_char,
    b"SPECIALSTAGE\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut TO_LIST: [*const libc::c_char; 4] = [
    b"PLAYER\0" as *const u8 as *const libc::c_char,
    b"ALLPLAYERS\0" as *const u8 as *const libc::c_char,
    b"MOBJ\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut COLOR_ENUMS: [*const libc::c_char; 158] = [
    b"NONE\0" as *const u8 as *const libc::c_char,
    b"WHITE\0" as *const u8 as *const libc::c_char,
    b"BONE\0" as *const u8 as *const libc::c_char,
    b"CLOUDY\0" as *const u8 as *const libc::c_char,
    b"GREY\0" as *const u8 as *const libc::c_char,
    b"SILVER\0" as *const u8 as *const libc::c_char,
    b"CARBON\0" as *const u8 as *const libc::c_char,
    b"JET\0" as *const u8 as *const libc::c_char,
    b"BLACK\0" as *const u8 as *const libc::c_char,
    b"AETHER\0" as *const u8 as *const libc::c_char,
    b"SLATE\0" as *const u8 as *const libc::c_char,
    b"MOONSTONE\0" as *const u8 as *const libc::c_char,
    b"BLUEBELL\0" as *const u8 as *const libc::c_char,
    b"PINK\0" as *const u8 as *const libc::c_char,
    b"ROSEWOOD\0" as *const u8 as *const libc::c_char,
    b"YOGURT\0" as *const u8 as *const libc::c_char,
    b"LATTE\0" as *const u8 as *const libc::c_char,
    b"BROWN\0" as *const u8 as *const libc::c_char,
    b"BOULDER\0" as *const u8 as *const libc::c_char,
    b"BRONZE\0" as *const u8 as *const libc::c_char,
    b"SEPIA\0" as *const u8 as *const libc::c_char,
    b"ECRU\0" as *const u8 as *const libc::c_char,
    b"TAN\0" as *const u8 as *const libc::c_char,
    b"BEIGE\0" as *const u8 as *const libc::c_char,
    b"ROSEBUSH\0" as *const u8 as *const libc::c_char,
    b"MOSS\0" as *const u8 as *const libc::c_char,
    b"AZURE\0" as *const u8 as *const libc::c_char,
    b"EGGPLANT\0" as *const u8 as *const libc::c_char,
    b"LAVENDER\0" as *const u8 as *const libc::c_char,
    b"RUBY\0" as *const u8 as *const libc::c_char,
    b"CHERRY\0" as *const u8 as *const libc::c_char,
    b"SALMON\0" as *const u8 as *const libc::c_char,
    b"PEPPER\0" as *const u8 as *const libc::c_char,
    b"RED\0" as *const u8 as *const libc::c_char,
    b"CRIMSON\0" as *const u8 as *const libc::c_char,
    b"FLAME\0" as *const u8 as *const libc::c_char,
    b"GARNET\0" as *const u8 as *const libc::c_char,
    b"KETCHUP\0" as *const u8 as *const libc::c_char,
    b"PEACHY\0" as *const u8 as *const libc::c_char,
    b"QUAIL\0" as *const u8 as *const libc::c_char,
    b"FOUNDATION\0" as *const u8 as *const libc::c_char,
    b"SUNSET\0" as *const u8 as *const libc::c_char,
    b"COPPER\0" as *const u8 as *const libc::c_char,
    b"APRICOT\0" as *const u8 as *const libc::c_char,
    b"ORANGE\0" as *const u8 as *const libc::c_char,
    b"RUST\0" as *const u8 as *const libc::c_char,
    b"TANGERINE\0" as *const u8 as *const libc::c_char,
    b"TOPAZ\0" as *const u8 as *const libc::c_char,
    b"GOLD\0" as *const u8 as *const libc::c_char,
    b"SANDY\0" as *const u8 as *const libc::c_char,
    b"GOLDENROD\0" as *const u8 as *const libc::c_char,
    b"YELLOW\0" as *const u8 as *const libc::c_char,
    b"OLIVE\0" as *const u8 as *const libc::c_char,
    b"PEAR\0" as *const u8 as *const libc::c_char,
    b"LEMON\0" as *const u8 as *const libc::c_char,
    b"LIME\0" as *const u8 as *const libc::c_char,
    b"PERIDOT\0" as *const u8 as *const libc::c_char,
    b"APPLE\0" as *const u8 as *const libc::c_char,
    b"HEADLIGHT\0" as *const u8 as *const libc::c_char,
    b"CHARTREUSE\0" as *const u8 as *const libc::c_char,
    b"GREEN\0" as *const u8 as *const libc::c_char,
    b"FOREST\0" as *const u8 as *const libc::c_char,
    b"SHAMROCK\0" as *const u8 as *const libc::c_char,
    b"JADE\0" as *const u8 as *const libc::c_char,
    b"MINT\0" as *const u8 as *const libc::c_char,
    b"MASTER\0" as *const u8 as *const libc::c_char,
    b"EMERALD\0" as *const u8 as *const libc::c_char,
    b"SEAFOAM\0" as *const u8 as *const libc::c_char,
    b"ISLAND\0" as *const u8 as *const libc::c_char,
    b"BOTTLE\0" as *const u8 as *const libc::c_char,
    b"AQUA\0" as *const u8 as *const libc::c_char,
    b"TEAL\0" as *const u8 as *const libc::c_char,
    b"OCEAN\0" as *const u8 as *const libc::c_char,
    b"WAVE\0" as *const u8 as *const libc::c_char,
    b"CYAN\0" as *const u8 as *const libc::c_char,
    b"TURQUOISE\0" as *const u8 as *const libc::c_char,
    b"AQUAMARINE\0" as *const u8 as *const libc::c_char,
    b"SKY\0" as *const u8 as *const libc::c_char,
    b"MARINE\0" as *const u8 as *const libc::c_char,
    b"CERULEAN\0" as *const u8 as *const libc::c_char,
    b"DREAM\0" as *const u8 as *const libc::c_char,
    b"ICY\0" as *const u8 as *const libc::c_char,
    b"DAYBREAK\0" as *const u8 as *const libc::c_char,
    b"SAPPHIRE\0" as *const u8 as *const libc::c_char,
    b"ARCTIC\0" as *const u8 as *const libc::c_char,
    b"CORNFLOWER\0" as *const u8 as *const libc::c_char,
    b"BLUE\0" as *const u8 as *const libc::c_char,
    b"COBALT\0" as *const u8 as *const libc::c_char,
    b"MIDNIGHT\0" as *const u8 as *const libc::c_char,
    b"GALAXY\0" as *const u8 as *const libc::c_char,
    b"VAPOR\0" as *const u8 as *const libc::c_char,
    b"DUSK\0" as *const u8 as *const libc::c_char,
    b"MAJESTY\0" as *const u8 as *const libc::c_char,
    b"PASTEL\0" as *const u8 as *const libc::c_char,
    b"PURPLE\0" as *const u8 as *const libc::c_char,
    b"NOBLE\0" as *const u8 as *const libc::c_char,
    b"FUCHSIA\0" as *const u8 as *const libc::c_char,
    b"BUBBLEGUM\0" as *const u8 as *const libc::c_char,
    b"SIBERITE\0" as *const u8 as *const libc::c_char,
    b"MAGENTA\0" as *const u8 as *const libc::c_char,
    b"NEON\0" as *const u8 as *const libc::c_char,
    b"VIOLET\0" as *const u8 as *const libc::c_char,
    b"ROYAL\0" as *const u8 as *const libc::c_char,
    b"LILAC\0" as *const u8 as *const libc::c_char,
    b"MAUVE\0" as *const u8 as *const libc::c_char,
    b"EVENTIDE\0" as *const u8 as *const libc::c_char,
    b"PLUM\0" as *const u8 as *const libc::c_char,
    b"RASPBERRY\0" as *const u8 as *const libc::c_char,
    b"TAFFY\0" as *const u8 as *const libc::c_char,
    b"ROSY\0" as *const u8 as *const libc::c_char,
    b"FANCY\0" as *const u8 as *const libc::c_char,
    b"SANGRIA\0" as *const u8 as *const libc::c_char,
    b"VOLCANIC\0" as *const u8 as *const libc::c_char,
    b"SUPERSILVER1\0" as *const u8 as *const libc::c_char,
    b"SUPERSILVER2\0" as *const u8 as *const libc::c_char,
    b"SUPERSILVER3\0" as *const u8 as *const libc::c_char,
    b"SUPERSILVER4\0" as *const u8 as *const libc::c_char,
    b"SUPERSILVER5\0" as *const u8 as *const libc::c_char,
    b"SUPERRED1\0" as *const u8 as *const libc::c_char,
    b"SUPERRED2\0" as *const u8 as *const libc::c_char,
    b"SUPERRED3\0" as *const u8 as *const libc::c_char,
    b"SUPERRED4\0" as *const u8 as *const libc::c_char,
    b"SUPERRED5\0" as *const u8 as *const libc::c_char,
    b"SUPERORANGE1\0" as *const u8 as *const libc::c_char,
    b"SUPERORANGE2\0" as *const u8 as *const libc::c_char,
    b"SUPERORANGE3\0" as *const u8 as *const libc::c_char,
    b"SUPERORANGE4\0" as *const u8 as *const libc::c_char,
    b"SUPERORANGE5\0" as *const u8 as *const libc::c_char,
    b"SUPERGOLD1\0" as *const u8 as *const libc::c_char,
    b"SUPERGOLD2\0" as *const u8 as *const libc::c_char,
    b"SUPERGOLD3\0" as *const u8 as *const libc::c_char,
    b"SUPERGOLD4\0" as *const u8 as *const libc::c_char,
    b"SUPERGOLD5\0" as *const u8 as *const libc::c_char,
    b"SUPERPERIDOT1\0" as *const u8 as *const libc::c_char,
    b"SUPERPERIDOT2\0" as *const u8 as *const libc::c_char,
    b"SUPERPERIDOT3\0" as *const u8 as *const libc::c_char,
    b"SUPERPERIDOT4\0" as *const u8 as *const libc::c_char,
    b"SUPERPERIDOT5\0" as *const u8 as *const libc::c_char,
    b"SUPERSKY1\0" as *const u8 as *const libc::c_char,
    b"SUPERSKY2\0" as *const u8 as *const libc::c_char,
    b"SUPERSKY3\0" as *const u8 as *const libc::c_char,
    b"SUPERSKY4\0" as *const u8 as *const libc::c_char,
    b"SUPERSKY5\0" as *const u8 as *const libc::c_char,
    b"SUPERPURPLE1\0" as *const u8 as *const libc::c_char,
    b"SUPERPURPLE2\0" as *const u8 as *const libc::c_char,
    b"SUPERPURPLE3\0" as *const u8 as *const libc::c_char,
    b"SUPERPURPLE4\0" as *const u8 as *const libc::c_char,
    b"SUPERPURPLE5\0" as *const u8 as *const libc::c_char,
    b"SUPERRUST1\0" as *const u8 as *const libc::c_char,
    b"SUPERRUST2\0" as *const u8 as *const libc::c_char,
    b"SUPERRUST3\0" as *const u8 as *const libc::c_char,
    b"SUPERRUST4\0" as *const u8 as *const libc::c_char,
    b"SUPERRUST5\0" as *const u8 as *const libc::c_char,
    b"SUPERTAN1\0" as *const u8 as *const libc::c_char,
    b"SUPERTAN2\0" as *const u8 as *const libc::c_char,
    b"SUPERTAN3\0" as *const u8 as *const libc::c_char,
    b"SUPERTAN4\0" as *const u8 as *const libc::c_char,
    b"SUPERTAN5\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut POWERS_LIST: [*const libc::c_char; 30] = [
    b"INVULNERABILITY\0" as *const u8 as *const libc::c_char,
    b"SNEAKERS\0" as *const u8 as *const libc::c_char,
    b"FLASHING\0" as *const u8 as *const libc::c_char,
    b"SHIELD\0" as *const u8 as *const libc::c_char,
    b"CARRY\0" as *const u8 as *const libc::c_char,
    b"TAILSFLY\0" as *const u8 as *const libc::c_char,
    b"UNDERWATER\0" as *const u8 as *const libc::c_char,
    b"SPACETIME\0" as *const u8 as *const libc::c_char,
    b"EXTRALIFE\0" as *const u8 as *const libc::c_char,
    b"PUSHING\0" as *const u8 as *const libc::c_char,
    b"JUSTSPRUNG\0" as *const u8 as *const libc::c_char,
    b"NOAUTOBRAKE\0" as *const u8 as *const libc::c_char,
    b"SUPER\0" as *const u8 as *const libc::c_char,
    b"GRAVITYBOOTS\0" as *const u8 as *const libc::c_char,
    b"INFINITYRING\0" as *const u8 as *const libc::c_char,
    b"AUTOMATICRING\0" as *const u8 as *const libc::c_char,
    b"BOUNCERING\0" as *const u8 as *const libc::c_char,
    b"SCATTERRING\0" as *const u8 as *const libc::c_char,
    b"GRENADERING\0" as *const u8 as *const libc::c_char,
    b"EXPLOSIONRING\0" as *const u8 as *const libc::c_char,
    b"RAILRING\0" as *const u8 as *const libc::c_char,
    b"EMERALDS\0" as *const u8 as *const libc::c_char,
    b"NIGHTS_SUPERLOOP\0" as *const u8 as *const libc::c_char,
    b"NIGHTS_HELPER\0" as *const u8 as *const libc::c_char,
    b"NIGHTS_LINKFREEZE\0" as *const u8 as *const libc::c_char,
    b"NOCONTROL\0" as *const u8 as *const libc::c_char,
    b"DYE\0" as *const u8 as *const libc::c_char,
    b"JUSTLAUNCHED\0" as *const u8 as *const libc::c_char,
    b"IGNORELATCH\0" as *const u8 as *const libc::c_char,
    b"STRONG\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut HUDITEMS_LIST: [*const libc::c_char; 20] = [
    b"LIVES\0" as *const u8 as *const libc::c_char,
    b"RINGS\0" as *const u8 as *const libc::c_char,
    b"RINGSNUM\0" as *const u8 as *const libc::c_char,
    b"RINGSNUMTICS\0" as *const u8 as *const libc::c_char,
    b"SCORE\0" as *const u8 as *const libc::c_char,
    b"SCORENUM\0" as *const u8 as *const libc::c_char,
    b"TIME\0" as *const u8 as *const libc::c_char,
    b"MINUTES\0" as *const u8 as *const libc::c_char,
    b"TIMECOLON\0" as *const u8 as *const libc::c_char,
    b"SECONDS\0" as *const u8 as *const libc::c_char,
    b"TIMETICCOLON\0" as *const u8 as *const libc::c_char,
    b"TICS\0" as *const u8 as *const libc::c_char,
    b"SS_TOTALRINGS\0" as *const u8 as *const libc::c_char,
    b"GETRINGS\0" as *const u8 as *const libc::c_char,
    b"GETRINGSNUM\0" as *const u8 as *const libc::c_char,
    b"TIMELEFT\0" as *const u8 as *const libc::c_char,
    b"TIMELEFTNUM\0" as *const u8 as *const libc::c_char,
    b"TIMEUP\0" as *const u8 as *const libc::c_char,
    b"HUNTPICS\0" as *const u8 as *const libc::c_char,
    b"POWERUPS\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut MENUTYPES_LIST: [*const libc::c_char; 57] = [
    b"NONE\0" as *const u8 as *const libc::c_char,
    b"MAIN\0" as *const u8 as *const libc::c_char,
    b"SP_MAIN\0" as *const u8 as *const libc::c_char,
    b"SP_LOAD\0" as *const u8 as *const libc::c_char,
    b"SP_PLAYER\0" as *const u8 as *const libc::c_char,
    b"SP_LEVELSELECT\0" as *const u8 as *const libc::c_char,
    b"SP_LEVELSTATS\0" as *const u8 as *const libc::c_char,
    b"SP_TIMEATTACK\0" as *const u8 as *const libc::c_char,
    b"SP_TIMEATTACK_LEVELSELECT\0" as *const u8 as *const libc::c_char,
    b"SP_GUESTREPLAY\0" as *const u8 as *const libc::c_char,
    b"SP_REPLAY\0" as *const u8 as *const libc::c_char,
    b"SP_GHOST\0" as *const u8 as *const libc::c_char,
    b"SP_NIGHTSATTACK\0" as *const u8 as *const libc::c_char,
    b"SP_NIGHTS_LEVELSELECT\0" as *const u8 as *const libc::c_char,
    b"SP_NIGHTS_GUESTREPLAY\0" as *const u8 as *const libc::c_char,
    b"SP_NIGHTS_REPLAY\0" as *const u8 as *const libc::c_char,
    b"SP_NIGHTS_GHOST\0" as *const u8 as *const libc::c_char,
    b"MP_MAIN\0" as *const u8 as *const libc::c_char,
    b"MP_SPLITSCREEN\0" as *const u8 as *const libc::c_char,
    b"MP_SERVER\0" as *const u8 as *const libc::c_char,
    b"MP_CONNECT\0" as *const u8 as *const libc::c_char,
    b"MP_ROOM\0" as *const u8 as *const libc::c_char,
    b"MP_PLAYERSETUP\0" as *const u8 as *const libc::c_char,
    b"MP_SERVER_OPTIONS\0" as *const u8 as *const libc::c_char,
    b"OP_MAIN\0" as *const u8 as *const libc::c_char,
    b"OP_P1CONTROLS\0" as *const u8 as *const libc::c_char,
    b"OP_CHANGECONTROLS\0" as *const u8 as *const libc::c_char,
    b"OP_P1MOUSE\0" as *const u8 as *const libc::c_char,
    b"OP_P1JOYSTICK\0" as *const u8 as *const libc::c_char,
    b"OP_JOYSTICKSET\0" as *const u8 as *const libc::c_char,
    b"OP_P1CAMERA\0" as *const u8 as *const libc::c_char,
    b"OP_P2CONTROLS\0" as *const u8 as *const libc::c_char,
    b"OP_P2MOUSE\0" as *const u8 as *const libc::c_char,
    b"OP_P2JOYSTICK\0" as *const u8 as *const libc::c_char,
    b"OP_P2CAMERA\0" as *const u8 as *const libc::c_char,
    b"OP_PLAYSTYLE\0" as *const u8 as *const libc::c_char,
    b"OP_VIDEO\0" as *const u8 as *const libc::c_char,
    b"OP_VIDEOMODE\0" as *const u8 as *const libc::c_char,
    b"OP_COLOR\0" as *const u8 as *const libc::c_char,
    b"OP_OPENGL\0" as *const u8 as *const libc::c_char,
    b"OP_OPENGL_LIGHTING\0" as *const u8 as *const libc::c_char,
    b"OP_SOUND\0" as *const u8 as *const libc::c_char,
    b"OP_SERVER\0" as *const u8 as *const libc::c_char,
    b"OP_MONITORTOGGLE\0" as *const u8 as *const libc::c_char,
    b"OP_DATA\0" as *const u8 as *const libc::c_char,
    b"OP_ADDONS\0" as *const u8 as *const libc::c_char,
    b"OP_SCREENSHOTS\0" as *const u8 as *const libc::c_char,
    b"OP_ERASEDATA\0" as *const u8 as *const libc::c_char,
    b"SR_MAIN\0" as *const u8 as *const libc::c_char,
    b"SR_PANDORA\0" as *const u8 as *const libc::c_char,
    b"SR_LEVELSELECT\0" as *const u8 as *const libc::c_char,
    b"SR_UNLOCKCHECKLIST\0" as *const u8 as *const libc::c_char,
    b"SR_EMBLEMHINT\0" as *const u8 as *const libc::c_char,
    b"SR_PLAYER\0" as *const u8 as *const libc::c_char,
    b"SR_SOUNDTEST\0" as *const u8 as *const libc::c_char,
    b"AD_MAIN\0" as *const u8 as *const libc::c_char,
    b"SPECIAL\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut INT_CONST: [int_const_s; 698] = [
    {
        let mut init = int_const_s {
            n: b"INT8_MIN\0" as *const u8 as *const libc::c_char,
            v: -(128 as libc::c_int),
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"INT16_MIN\0" as *const u8 as *const libc::c_char,
            v: -(32767 as libc::c_int) - 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"INT32_MIN\0" as *const u8 as *const libc::c_char,
            v: -(2147483647 as libc::c_int) - 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"INT8_MAX\0" as *const u8 as *const libc::c_char,
            v: 127 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"INT16_MAX\0" as *const u8 as *const libc::c_char,
            v: 32767 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"INT32_MAX\0" as *const u8 as *const libc::c_char,
            v: 2147483647 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"UINT8_MAX\0" as *const u8 as *const libc::c_char,
            v: 255 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"UINT16_MAX\0" as *const u8 as *const libc::c_char,
            v: 65535 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"UINT32_MAX\0" as *const u8 as *const libc::c_char,
            v: 4294967295 as libc::c_uint as lua_Number,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FRACUNIT\0" as *const u8 as *const libc::c_char,
            v: (1 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FU\0" as *const u8 as *const libc::c_char,
            v: (1 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FRACBITS\0" as *const u8 as *const libc::c_char,
            v: 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"TICRATE\0" as *const u8 as *const libc::c_char,
            v: 35 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MUSICRATE\0" as *const u8 as *const libc::c_char,
            v: 1000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RING_DIST\0" as *const u8 as *const libc::c_char,
            v: 512 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PUSHACCEL\0" as *const u8 as *const libc::c_char,
            v: 2 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MODID\0" as *const u8 as *const libc::c_char,
            v: 18 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MODVERSION\0" as *const u8 as *const libc::c_char,
            v: 54 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CODEBASE\0" as *const u8 as *const libc::c_char,
            v: 220 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"NEWTICRATE\0" as *const u8 as *const libc::c_char,
            v: 35 as libc::c_int * 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"NEWTICRATERATIO\0" as *const u8 as *const libc::c_char,
            v: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LE_PINCHPHASE\0" as *const u8 as *const libc::c_char,
            v: LE_PINCHPHASE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LE_ALLBOSSESDEAD\0" as *const u8 as *const libc::c_char,
            v: LE_ALLBOSSESDEAD as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LE_BOSSDEAD\0" as *const u8 as *const libc::c_char,
            v: LE_BOSSDEAD as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LE_BOSS4DROP\0" as *const u8 as *const libc::c_char,
            v: LE_BOSS4DROP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LE_BRAKVILEATACK\0" as *const u8 as *const libc::c_char,
            v: LE_BRAKVILEATACK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LE_TURRET\0" as *const u8 as *const libc::c_char,
            v: LE_TURRET as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LE_BRAKPLATFORM\0" as *const u8 as *const libc::c_char,
            v: LE_BRAKPLATFORM as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LE_CAPSULE2\0" as *const u8 as *const libc::c_char,
            v: LE_CAPSULE2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LE_CAPSULE1\0" as *const u8 as *const libc::c_char,
            v: LE_CAPSULE1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LE_CAPSULE0\0" as *const u8 as *const libc::c_char,
            v: LE_CAPSULE0 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LE_KOOPA\0" as *const u8 as *const libc::c_char,
            v: LE_KOOPA as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LE_AXE\0" as *const u8 as *const libc::c_char,
            v: LE_AXE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LE_PARAMWIDTH\0" as *const u8 as *const libc::c_char,
            v: LE_PARAMWIDTH as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_FRAMEMASK\0" as *const u8 as *const libc::c_char,
            v: 0xff as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_SPR2SUPER\0" as *const u8 as *const libc::c_char,
            v: 0x80 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_SPR2ENDSTATE\0" as *const u8 as *const libc::c_char,
            v: 0x100 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_SPR2MIDSTART\0" as *const u8 as *const libc::c_char,
            v: 0x200 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_ANIMATE\0" as *const u8 as *const libc::c_char,
            v: 0x10000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_RANDOMANIM\0" as *const u8 as *const libc::c_char,
            v: 0x40000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_GLOBALANIM\0" as *const u8 as *const libc::c_char,
            v: 0x20000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_FULLBRIGHT\0" as *const u8 as *const libc::c_char,
            v: 0x100000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_SEMIBRIGHT\0" as *const u8 as *const libc::c_char,
            v: 0x100000 as libc::c_int | 0x200000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_FULLDARK\0" as *const u8 as *const libc::c_char,
            v: 0x200000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_VERTICALFLIP\0" as *const u8 as *const libc::c_char,
            v: 0x1000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_HORIZONTALFLIP\0" as *const u8 as *const libc::c_char,
            v: 0x2000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_PAPERSPRITE\0" as *const u8 as *const libc::c_char,
            v: 0x400000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_FLOORSPRITE\0" as *const u8 as *const libc::c_char,
            v: 0x800000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_BLENDMASK\0" as *const u8 as *const libc::c_char,
            v: 0x7000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_BLENDSHIFT\0" as *const u8 as *const libc::c_char,
            v: 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_ADD\0" as *const u8 as *const libc::c_char,
            v: (AST_ADD as libc::c_int - 1 as libc::c_int) << 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_SUBTRACT\0" as *const u8 as *const libc::c_char,
            v: (AST_SUBTRACT as libc::c_int - 1 as libc::c_int) << 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_REVERSESUBTRACT\0" as *const u8 as *const libc::c_char,
            v: (AST_REVERSESUBTRACT as libc::c_int - 1 as libc::c_int)
                << 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_MODULATE\0" as *const u8 as *const libc::c_char,
            v: (AST_MODULATE as libc::c_int - 1 as libc::c_int) << 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_TRANSMASK\0" as *const u8 as *const libc::c_char,
            v: 0xf0000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_TRANSSHIFT\0" as *const u8 as *const libc::c_char,
            v: 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_TRANS10\0" as *const u8 as *const libc::c_char,
            v: (tr_trans10 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_TRANS20\0" as *const u8 as *const libc::c_char,
            v: (tr_trans20 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_TRANS30\0" as *const u8 as *const libc::c_char,
            v: (tr_trans30 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_TRANS40\0" as *const u8 as *const libc::c_char,
            v: (tr_trans40 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_TRANS50\0" as *const u8 as *const libc::c_char,
            v: (tr_trans50 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_TRANS60\0" as *const u8 as *const libc::c_char,
            v: (tr_trans60 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_TRANS70\0" as *const u8 as *const libc::c_char,
            v: (tr_trans70 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_TRANS80\0" as *const u8 as *const libc::c_char,
            v: (tr_trans80 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_TRANS90\0" as *const u8 as *const libc::c_char,
            v: (tr_trans90 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"TR_TRANS10\0" as *const u8 as *const libc::c_char,
            v: (tr_trans10 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"TR_TRANS20\0" as *const u8 as *const libc::c_char,
            v: (tr_trans20 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"TR_TRANS30\0" as *const u8 as *const libc::c_char,
            v: (tr_trans30 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"TR_TRANS40\0" as *const u8 as *const libc::c_char,
            v: (tr_trans40 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"TR_TRANS50\0" as *const u8 as *const libc::c_char,
            v: (tr_trans50 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"TR_TRANS60\0" as *const u8 as *const libc::c_char,
            v: (tr_trans60 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"TR_TRANS70\0" as *const u8 as *const libc::c_char,
            v: (tr_trans70 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"TR_TRANS80\0" as *const u8 as *const libc::c_char,
            v: (tr_trans80 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"TR_TRANS90\0" as *const u8 as *const libc::c_char,
            v: (tr_trans90 as libc::c_int) << 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"tr_trans10\0" as *const u8 as *const libc::c_char,
            v: tr_trans10 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"tr_trans20\0" as *const u8 as *const libc::c_char,
            v: tr_trans20 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"tr_trans30\0" as *const u8 as *const libc::c_char,
            v: tr_trans30 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"tr_trans40\0" as *const u8 as *const libc::c_char,
            v: tr_trans40 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"tr_trans50\0" as *const u8 as *const libc::c_char,
            v: tr_trans50 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"tr_trans60\0" as *const u8 as *const libc::c_char,
            v: tr_trans60 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"tr_trans70\0" as *const u8 as *const libc::c_char,
            v: tr_trans70 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"tr_trans80\0" as *const u8 as *const libc::c_char,
            v: tr_trans80 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"tr_trans90\0" as *const u8 as *const libc::c_char,
            v: tr_trans90 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"NUMTRANSMAPS\0" as *const u8 as *const libc::c_char,
            v: NUMTRANSMAPS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"AST_COPY\0" as *const u8 as *const libc::c_char,
            v: AST_COPY as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"AST_TRANSLUCENT\0" as *const u8 as *const libc::c_char,
            v: AST_TRANSLUCENT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"AST_ADD\0" as *const u8 as *const libc::c_char,
            v: AST_ADD as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"AST_SUBTRACT\0" as *const u8 as *const libc::c_char,
            v: AST_SUBTRACT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"AST_REVERSESUBTRACT\0" as *const u8 as *const libc::c_char,
            v: AST_REVERSESUBTRACT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"AST_MODULATE\0" as *const u8 as *const libc::c_char,
            v: AST_MODULATE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"AST_OVERLAY\0" as *const u8 as *const libc::c_char,
            v: AST_OVERLAY as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"AST_FOG\0" as *const u8 as *const libc::c_char,
            v: AST_FOG as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_HORIZONTALFLIP\0" as *const u8 as *const libc::c_char,
            v: RF_HORIZONTALFLIP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_VERTICALFLIP\0" as *const u8 as *const libc::c_char,
            v: RF_VERTICALFLIP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_ABSOLUTEOFFSETS\0" as *const u8 as *const libc::c_char,
            v: RF_ABSOLUTEOFFSETS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_FLIPOFFSETS\0" as *const u8 as *const libc::c_char,
            v: RF_FLIPOFFSETS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_SPLATMASK\0" as *const u8 as *const libc::c_char,
            v: RF_SPLATMASK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_SLOPESPLAT\0" as *const u8 as *const libc::c_char,
            v: RF_SLOPESPLAT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_OBJECTSLOPESPLAT\0" as *const u8 as *const libc::c_char,
            v: RF_OBJECTSLOPESPLAT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_NOSPLATBILLBOARD\0" as *const u8 as *const libc::c_char,
            v: RF_NOSPLATBILLBOARD as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_NOSPLATROLLANGLE\0" as *const u8 as *const libc::c_char,
            v: RF_NOSPLATROLLANGLE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_BRIGHTMASK\0" as *const u8 as *const libc::c_char,
            v: RF_BRIGHTMASK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_FULLBRIGHT\0" as *const u8 as *const libc::c_char,
            v: RF_FULLBRIGHT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_FULLDARK\0" as *const u8 as *const libc::c_char,
            v: RF_FULLDARK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_SEMIBRIGHT\0" as *const u8 as *const libc::c_char,
            v: RF_SEMIBRIGHT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_NOCOLORMAPS\0" as *const u8 as *const libc::c_char,
            v: RF_NOCOLORMAPS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_SPRITETYPEMASK\0" as *const u8 as *const libc::c_char,
            v: RF_SPRITETYPEMASK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_PAPERSPRITE\0" as *const u8 as *const libc::c_char,
            v: RF_PAPERSPRITE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_FLOORSPRITE\0" as *const u8 as *const libc::c_char,
            v: RF_FLOORSPRITE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_SHADOWDRAW\0" as *const u8 as *const libc::c_char,
            v: RF_SHADOWDRAW as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_SHADOWEFFECTS\0" as *const u8 as *const libc::c_char,
            v: RF_SHADOWEFFECTS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RF_DROPSHADOW\0" as *const u8 as *const libc::c_char,
            v: RF_DROPSHADOW as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LF_SCRIPTISFILE\0" as *const u8 as *const libc::c_char,
            v: (1 as libc::c_int) << 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LF_SPEEDMUSIC\0" as *const u8 as *const libc::c_char,
            v: (1 as libc::c_int) << 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LF_NOSSMUSIC\0" as *const u8 as *const libc::c_char,
            v: (1 as libc::c_int) << 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LF_NORELOAD\0" as *const u8 as *const libc::c_char,
            v: (1 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LF_NOZONE\0" as *const u8 as *const libc::c_char,
            v: (1 as libc::c_int) << 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LF_SAVEGAME\0" as *const u8 as *const libc::c_char,
            v: (1 as libc::c_int) << 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LF_MIXNIGHTSCOUNTDOWN\0" as *const u8 as *const libc::c_char,
            v: (1 as libc::c_int) << 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LF_NOTITLECARDFIRST\0" as *const u8 as *const libc::c_char,
            v: (1 as libc::c_int) << 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LF_NOTITLECARDRESPAWN\0" as *const u8 as *const libc::c_char,
            v: (1 as libc::c_int) << 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LF_NOTITLECARDRECORDATTACK\0" as *const u8 as *const libc::c_char,
            v: (1 as libc::c_int) << 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LF_NOTITLECARD\0" as *const u8 as *const libc::c_char,
            v: (1 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int
                | (1 as libc::c_int) << 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LF_WARNINGTITLE\0" as *const u8 as *const libc::c_char,
            v: (1 as libc::c_int) << 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LF2_HIDEINMENU\0" as *const u8 as *const libc::c_char,
            v: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LF2_HIDEINSTATS\0" as *const u8 as *const libc::c_char,
            v: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LF2_RECORDATTACK\0" as *const u8 as *const libc::c_char,
            v: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LF2_NIGHTSATTACK\0" as *const u8 as *const libc::c_char,
            v: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LF2_NOVISITNEEDED\0" as *const u8 as *const libc::c_char,
            v: 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"LF2_WIDEICON\0" as *const u8 as *const libc::c_char,
            v: 32 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"EMERALD1\0" as *const u8 as *const libc::c_char,
            v: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"EMERALD2\0" as *const u8 as *const libc::c_char,
            v: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"EMERALD3\0" as *const u8 as *const libc::c_char,
            v: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"EMERALD4\0" as *const u8 as *const libc::c_char,
            v: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"EMERALD5\0" as *const u8 as *const libc::c_char,
            v: 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"EMERALD6\0" as *const u8 as *const libc::c_char,
            v: 32 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"EMERALD7\0" as *const u8 as *const libc::c_char,
            v: 64 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MAXSKINCOLORS\0" as *const u8 as *const libc::c_char,
            v: MAXSKINCOLORS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FIRSTSUPERCOLOR\0" as *const u8 as *const libc::c_char,
            v: FIRSTSUPERCOLOR as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"NUMSUPERCOLORS\0" as *const u8 as *const libc::c_char,
            v: NUMSUPERCOLORS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PRECIP_NONE\0" as *const u8 as *const libc::c_char,
            v: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PRECIP_STORM\0" as *const u8 as *const libc::c_char,
            v: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PRECIP_SNOW\0" as *const u8 as *const libc::c_char,
            v: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PRECIP_RAIN\0" as *const u8 as *const libc::c_char,
            v: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PRECIP_BLANK\0" as *const u8 as *const libc::c_char,
            v: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PRECIP_STORM_NORAIN\0" as *const u8 as *const libc::c_char,
            v: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PRECIP_STORM_NOSTRIKES\0" as *const u8 as *const libc::c_char,
            v: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_NONE\0" as *const u8 as *const libc::c_char,
            v: SH_NONE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_PROTECTFIRE\0" as *const u8 as *const libc::c_char,
            v: SH_PROTECTFIRE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_PROTECTWATER\0" as *const u8 as *const libc::c_char,
            v: SH_PROTECTWATER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_PROTECTELECTRIC\0" as *const u8 as *const libc::c_char,
            v: SH_PROTECTELECTRIC as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_PROTECTSPIKE\0" as *const u8 as *const libc::c_char,
            v: SH_PROTECTSPIKE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_PITY\0" as *const u8 as *const libc::c_char,
            v: SH_PITY as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_WHIRLWIND\0" as *const u8 as *const libc::c_char,
            v: SH_WHIRLWIND as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_ARMAGEDDON\0" as *const u8 as *const libc::c_char,
            v: SH_ARMAGEDDON as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_PINK\0" as *const u8 as *const libc::c_char,
            v: SH_PINK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_ATTRACT\0" as *const u8 as *const libc::c_char,
            v: SH_ATTRACT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_ELEMENTAL\0" as *const u8 as *const libc::c_char,
            v: SH_ELEMENTAL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_FLAMEAURA\0" as *const u8 as *const libc::c_char,
            v: SH_FLAMEAURA as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_BUBBLEWRAP\0" as *const u8 as *const libc::c_char,
            v: SH_BUBBLEWRAP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_THUNDERCOIN\0" as *const u8 as *const libc::c_char,
            v: SH_THUNDERCOIN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_FORCE\0" as *const u8 as *const libc::c_char,
            v: SH_FORCE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_FORCEHP\0" as *const u8 as *const libc::c_char,
            v: SH_FORCEHP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_FIREFLOWER\0" as *const u8 as *const libc::c_char,
            v: SH_FIREFLOWER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_STACK\0" as *const u8 as *const libc::c_char,
            v: SH_STACK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SH_NOSTACK\0" as *const u8 as *const libc::c_char,
            v: SH_NOSTACK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CR_NONE\0" as *const u8 as *const libc::c_char,
            v: CR_NONE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CR_GENERIC\0" as *const u8 as *const libc::c_char,
            v: CR_GENERIC as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CR_PLAYER\0" as *const u8 as *const libc::c_char,
            v: CR_PLAYER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CR_NIGHTSMODE\0" as *const u8 as *const libc::c_char,
            v: CR_NIGHTSMODE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CR_NIGHTSFALL\0" as *const u8 as *const libc::c_char,
            v: CR_NIGHTSFALL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CR_BRAKGOOP\0" as *const u8 as *const libc::c_char,
            v: CR_BRAKGOOP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CR_ZOOMTUBE\0" as *const u8 as *const libc::c_char,
            v: CR_ZOOMTUBE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CR_ROPEHANG\0" as *const u8 as *const libc::c_char,
            v: CR_ROPEHANG as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CR_MACESPIN\0" as *const u8 as *const libc::c_char,
            v: CR_MACESPIN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CR_MINECART\0" as *const u8 as *const libc::c_char,
            v: CR_MINECART as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CR_ROLLOUT\0" as *const u8 as *const libc::c_char,
            v: CR_ROLLOUT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CR_PTERABYTE\0" as *const u8 as *const libc::c_char,
            v: CR_PTERABYTE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CR_DUSTDEVIL\0" as *const u8 as *const libc::c_char,
            v: CR_DUSTDEVIL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CR_FAN\0" as *const u8 as *const libc::c_char,
            v: CR_FAN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_NONE\0" as *const u8 as *const libc::c_char,
            v: STR_NONE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_ANIM\0" as *const u8 as *const libc::c_char,
            v: STR_ANIM as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_PUNCH\0" as *const u8 as *const libc::c_char,
            v: STR_PUNCH as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_TAIL\0" as *const u8 as *const libc::c_char,
            v: STR_TAIL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_STOMP\0" as *const u8 as *const libc::c_char,
            v: STR_STOMP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_UPPER\0" as *const u8 as *const libc::c_char,
            v: STR_UPPER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_GUARD\0" as *const u8 as *const libc::c_char,
            v: STR_GUARD as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_HEAVY\0" as *const u8 as *const libc::c_char,
            v: STR_HEAVY as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_DASH\0" as *const u8 as *const libc::c_char,
            v: STR_DASH as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_WALL\0" as *const u8 as *const libc::c_char,
            v: STR_WALL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_FLOOR\0" as *const u8 as *const libc::c_char,
            v: STR_FLOOR as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_CEILING\0" as *const u8 as *const libc::c_char,
            v: STR_CEILING as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_SPRING\0" as *const u8 as *const libc::c_char,
            v: STR_SPRING as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_SPIKE\0" as *const u8 as *const libc::c_char,
            v: STR_SPIKE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_ATTACK\0" as *const u8 as *const libc::c_char,
            v: STR_ATTACK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_BUST\0" as *const u8 as *const libc::c_char,
            v: STR_BUST as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_FLY\0" as *const u8 as *const libc::c_char,
            v: STR_FLY as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_GLIDE\0" as *const u8 as *const libc::c_char,
            v: STR_GLIDE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_TWINSPIN\0" as *const u8 as *const libc::c_char,
            v: STR_TWINSPIN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_MELEE\0" as *const u8 as *const libc::c_char,
            v: STR_MELEE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_BOUNCE\0" as *const u8 as *const libc::c_char,
            v: STR_BOUNCE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"STR_METAL\0" as *const u8 as *const libc::c_char,
            v: STR_METAL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RW_AUTO\0" as *const u8 as *const libc::c_char,
            v: RW_AUTO as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RW_BOUNCE\0" as *const u8 as *const libc::c_char,
            v: RW_BOUNCE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RW_SCATTER\0" as *const u8 as *const libc::c_char,
            v: RW_SCATTER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RW_GRENADE\0" as *const u8 as *const libc::c_char,
            v: RW_GRENADE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RW_EXPLODE\0" as *const u8 as *const libc::c_char,
            v: RW_EXPLODE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"RW_RAIL\0" as *const u8 as *const libc::c_char,
            v: RW_RAIL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_SUPER\0" as *const u8 as *const libc::c_char,
            v: SF_SUPER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_NOSUPERSPIN\0" as *const u8 as *const libc::c_char,
            v: SF_NOSUPERSPIN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_NOSPINDASHDUST\0" as *const u8 as *const libc::c_char,
            v: SF_NOSPINDASHDUST as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_HIRES\0" as *const u8 as *const libc::c_char,
            v: SF_HIRES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_NOSKID\0" as *const u8 as *const libc::c_char,
            v: SF_NOSKID as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_NOSPEEDADJUST\0" as *const u8 as *const libc::c_char,
            v: SF_NOSPEEDADJUST as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_RUNONWATER\0" as *const u8 as *const libc::c_char,
            v: SF_RUNONWATER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_NOJUMPSPIN\0" as *const u8 as *const libc::c_char,
            v: SF_NOJUMPSPIN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_NOJUMPDAMAGE\0" as *const u8 as *const libc::c_char,
            v: SF_NOJUMPDAMAGE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_STOMPDAMAGE\0" as *const u8 as *const libc::c_char,
            v: SF_STOMPDAMAGE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_MARIODAMAGE\0" as *const u8 as *const libc::c_char,
            v: SF_MARIODAMAGE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_MACHINE\0" as *const u8 as *const libc::c_char,
            v: SF_MACHINE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_DASHMODE\0" as *const u8 as *const libc::c_char,
            v: SF_DASHMODE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_FASTEDGE\0" as *const u8 as *const libc::c_char,
            v: SF_FASTEDGE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_MULTIABILITY\0" as *const u8 as *const libc::c_char,
            v: SF_MULTIABILITY as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_NONIGHTSROTATION\0" as *const u8 as *const libc::c_char,
            v: SF_NONIGHTSROTATION as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_NONIGHTSSUPER\0" as *const u8 as *const libc::c_char,
            v: SF_NONIGHTSSUPER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_NOSUPERSPRITES\0" as *const u8 as *const libc::c_char,
            v: SF_NOSUPERSPRITES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_NOSUPERJUMPBOOST\0" as *const u8 as *const libc::c_char,
            v: SF_NOSUPERJUMPBOOST as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_CANBUSTWALLS\0" as *const u8 as *const libc::c_char,
            v: SF_CANBUSTWALLS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_NOSHIELDABILITY\0" as *const u8 as *const libc::c_char,
            v: SF_NOSHIELDABILITY as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DASHMODE_THRESHOLD\0" as *const u8 as *const libc::c_char,
            v: 3 as libc::c_int * 35 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DASHMODE_MAX\0" as *const u8 as *const libc::c_char,
            v: 3 as libc::c_int * 35 as libc::c_int + 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA_NONE\0" as *const u8 as *const libc::c_char,
            v: CA_NONE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA_THOK\0" as *const u8 as *const libc::c_char,
            v: CA_THOK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA_FLY\0" as *const u8 as *const libc::c_char,
            v: CA_FLY as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA_GLIDEANDCLIMB\0" as *const u8 as *const libc::c_char,
            v: CA_GLIDEANDCLIMB as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA_HOMINGTHOK\0" as *const u8 as *const libc::c_char,
            v: CA_HOMINGTHOK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA_DOUBLEJUMP\0" as *const u8 as *const libc::c_char,
            v: CA_DOUBLEJUMP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA_FLOAT\0" as *const u8 as *const libc::c_char,
            v: CA_FLOAT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA_SLOWFALL\0" as *const u8 as *const libc::c_char,
            v: CA_SLOWFALL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA_SWIM\0" as *const u8 as *const libc::c_char,
            v: CA_SWIM as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA_TELEKINESIS\0" as *const u8 as *const libc::c_char,
            v: CA_TELEKINESIS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA_FALLSWITCH\0" as *const u8 as *const libc::c_char,
            v: CA_FALLSWITCH as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA_JUMPBOOST\0" as *const u8 as *const libc::c_char,
            v: CA_JUMPBOOST as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA_AIRDRILL\0" as *const u8 as *const libc::c_char,
            v: CA_AIRDRILL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA_JUMPTHOK\0" as *const u8 as *const libc::c_char,
            v: CA_JUMPTHOK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA_BOUNCE\0" as *const u8 as *const libc::c_char,
            v: CA_BOUNCE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA_TWINSPIN\0" as *const u8 as *const libc::c_char,
            v: CA_TWINSPIN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA2_NONE\0" as *const u8 as *const libc::c_char,
            v: CA2_NONE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA2_SPINDASH\0" as *const u8 as *const libc::c_char,
            v: CA2_SPINDASH as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA2_GUNSLINGER\0" as *const u8 as *const libc::c_char,
            v: CA2_GUNSLINGER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CA2_MELEE\0" as *const u8 as *const libc::c_char,
            v: CA2_MELEE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_TOTALLYSINGLE\0" as *const u8 as *const libc::c_char,
            v: SF_TOTALLYSINGLE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_NOMULTIPLESOUND\0" as *const u8 as *const libc::c_char,
            v: SF_NOMULTIPLESOUND as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_OUTSIDESOUND\0" as *const u8 as *const libc::c_char,
            v: SF_OUTSIDESOUND as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_X4AWAYSOUND\0" as *const u8 as *const libc::c_char,
            v: SF_X4AWAYSOUND as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_X8AWAYSOUND\0" as *const u8 as *const libc::c_char,
            v: SF_X8AWAYSOUND as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_NOINTERRUPT\0" as *const u8 as *const libc::c_char,
            v: SF_NOINTERRUPT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SF_X2AWAYSOUND\0" as *const u8 as *const libc::c_char,
            v: SF_X2AWAYSOUND as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GE_NIGHTSPULL\0" as *const u8 as *const libc::c_char,
            v: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GE_NIGHTSITEM\0" as *const u8 as *const libc::c_char,
            v: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ME_ALLEMERALDS\0" as *const u8 as *const libc::c_char,
            v: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ME_ULTIMATE\0" as *const u8 as *const libc::c_char,
            v: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ME_PERFECT\0" as *const u8 as *const libc::c_char,
            v: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FLOATSPEED\0" as *const u8 as *const libc::c_char,
            v: ((1 as libc::c_int) << 16 as libc::c_int) * 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MAXSTEPMOVE\0" as *const u8 as *const libc::c_char,
            v: 24 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"USERANGE\0" as *const u8 as *const libc::c_char,
            v: 64 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MELEERANGE\0" as *const u8 as *const libc::c_char,
            v: 64 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MISSILERANGE\0" as *const u8 as *const libc::c_char,
            v: 32 as libc::c_int * 64 as libc::c_int
                * ((1 as libc::c_int) << 16 as libc::c_int),
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ONFLOORZ\0" as *const u8 as *const libc::c_char,
            v: -(2147483647 as libc::c_int) - 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ONCEILINGZ\0" as *const u8 as *const libc::c_char,
            v: 2147483647 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PAL_WHITE\0" as *const u8 as *const libc::c_char,
            v: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PAL_MIXUP\0" as *const u8 as *const libc::c_char,
            v: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PAL_RECYCLE\0" as *const u8 as *const libc::c_char,
            v: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PAL_NUKE\0" as *const u8 as *const libc::c_char,
            v: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PAL_INVERT\0" as *const u8 as *const libc::c_char,
            v: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DMG_WATER\0" as *const u8 as *const libc::c_char,
            v: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DMG_FIRE\0" as *const u8 as *const libc::c_char,
            v: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DMG_ELECTRIC\0" as *const u8 as *const libc::c_char,
            v: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DMG_SPIKE\0" as *const u8 as *const libc::c_char,
            v: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DMG_NUKE\0" as *const u8 as *const libc::c_char,
            v: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DMG_INSTAKILL\0" as *const u8 as *const libc::c_char,
            v: 0x80 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DMG_DROWNED\0" as *const u8 as *const libc::c_char,
            v: 0x80 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DMG_SPACEDROWN\0" as *const u8 as *const libc::c_char,
            v: 0x80 as libc::c_int + 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DMG_DEATHPIT\0" as *const u8 as *const libc::c_char,
            v: 0x80 as libc::c_int + 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DMG_CRUSHED\0" as *const u8 as *const libc::c_char,
            v: 0x80 as libc::c_int + 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DMG_SPECTATOR\0" as *const u8 as *const libc::c_char,
            v: 0x80 as libc::c_int + 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DMG_CANHURTSELF\0" as *const u8 as *const libc::c_char,
            v: 0x40 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DMG_DEATHMASK\0" as *const u8 as *const libc::c_char,
            v: 0x80 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"int_none\0" as *const u8 as *const libc::c_char,
            v: int_none as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"int_coop\0" as *const u8 as *const libc::c_char,
            v: int_coop as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"int_match\0" as *const u8 as *const libc::c_char,
            v: int_match as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"int_teammatch\0" as *const u8 as *const libc::c_char,
            v: int_teammatch as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"int_ctf\0" as *const u8 as *const libc::c_char,
            v: int_ctf as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"int_spec\0" as *const u8 as *const libc::c_char,
            v: int_spec as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"int_race\0" as *const u8 as *const libc::c_char,
            v: int_race as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"int_comp\0" as *const u8 as *const libc::c_char,
            v: int_comp as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JT_NONE\0" as *const u8 as *const libc::c_char,
            v: JT_NONE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JT_OTHER\0" as *const u8 as *const libc::c_char,
            v: JT_OTHER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JT_MASTER\0" as *const u8 as *const libc::c_char,
            v: JT_MASTER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JT_1UP\0" as *const u8 as *const libc::c_char,
            v: JT_1UP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JT_SHOES\0" as *const u8 as *const libc::c_char,
            v: JT_SHOES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JT_INV\0" as *const u8 as *const libc::c_char,
            v: JT_INV as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JT_MINV\0" as *const u8 as *const libc::c_char,
            v: JT_MINV as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JT_DROWN\0" as *const u8 as *const libc::c_char,
            v: JT_DROWN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JT_SUPER\0" as *const u8 as *const libc::c_char,
            v: JT_SUPER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JT_GOVER\0" as *const u8 as *const libc::c_char,
            v: JT_GOVER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JT_NIGHTSTIMEOUT\0" as *const u8 as *const libc::c_char,
            v: JT_NIGHTSTIMEOUT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JT_SSTIMEOUT\0" as *const u8 as *const libc::c_char,
            v: JT_SSTIMEOUT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PST_LIVE\0" as *const u8 as *const libc::c_char,
            v: PST_LIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PST_DEAD\0" as *const u8 as *const libc::c_char,
            v: PST_DEAD as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PST_REBORN\0" as *const u8 as *const libc::c_char,
            v: PST_REBORN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PA_ETC\0" as *const u8 as *const libc::c_char,
            v: PA_ETC as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PA_IDLE\0" as *const u8 as *const libc::c_char,
            v: PA_IDLE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PA_EDGE\0" as *const u8 as *const libc::c_char,
            v: PA_EDGE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PA_WALK\0" as *const u8 as *const libc::c_char,
            v: PA_WALK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PA_RUN\0" as *const u8 as *const libc::c_char,
            v: PA_RUN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PA_DASH\0" as *const u8 as *const libc::c_char,
            v: PA_DASH as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PA_PAIN\0" as *const u8 as *const libc::c_char,
            v: PA_PAIN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PA_ROLL\0" as *const u8 as *const libc::c_char,
            v: PA_ROLL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PA_JUMP\0" as *const u8 as *const libc::c_char,
            v: PA_JUMP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PA_SPRING\0" as *const u8 as *const libc::c_char,
            v: PA_SPRING as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PA_FALL\0" as *const u8 as *const libc::c_char,
            v: PA_FALL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PA_ABILITY\0" as *const u8 as *const libc::c_char,
            v: PA_ABILITY as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PA_ABILITY2\0" as *const u8 as *const libc::c_char,
            v: PA_ABILITY2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"PA_RIDE\0" as *const u8 as *const libc::c_char,
            v: PA_RIDE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"WEP_AUTO\0" as *const u8 as *const libc::c_char,
            v: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"WEP_BOUNCE\0" as *const u8 as *const libc::c_char,
            v: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"WEP_SCATTER\0" as *const u8 as *const libc::c_char,
            v: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"WEP_GRENADE\0" as *const u8 as *const libc::c_char,
            v: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"WEP_EXPLODE\0" as *const u8 as *const libc::c_char,
            v: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"WEP_RAIL\0" as *const u8 as *const libc::c_char,
            v: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"NUM_WEAPONS\0" as *const u8 as *const libc::c_char,
            v: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"INFLIVES\0" as *const u8 as *const libc::c_char,
            v: 0x7f as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GF_REDFLAG\0" as *const u8 as *const libc::c_char,
            v: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GF_BLUEFLAG\0" as *const u8 as *const libc::c_char,
            v: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BOT_NONE\0" as *const u8 as *const libc::c_char,
            v: BOT_NONE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BOT_2PAI\0" as *const u8 as *const libc::c_char,
            v: BOT_2PAI as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BOT_2PHUMAN\0" as *const u8 as *const libc::c_char,
            v: BOT_2PHUMAN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BOT_MPAI\0" as *const u8 as *const libc::c_char,
            v: BOT_MPAI as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSSPIN\0" as *const u8 as *const libc::c_char,
            v: SKSSPIN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSPUTPUT\0" as *const u8 as *const libc::c_char,
            v: SKSPUTPUT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSPUDPUD\0" as *const u8 as *const libc::c_char,
            v: SKSPUDPUD as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSPLPAN1\0" as *const u8 as *const libc::c_char,
            v: SKSPLPAN1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSPLPAN2\0" as *const u8 as *const libc::c_char,
            v: SKSPLPAN2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSPLPAN3\0" as *const u8 as *const libc::c_char,
            v: SKSPLPAN3 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSPLPAN4\0" as *const u8 as *const libc::c_char,
            v: SKSPLPAN4 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSPLDET1\0" as *const u8 as *const libc::c_char,
            v: SKSPLDET1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSPLDET2\0" as *const u8 as *const libc::c_char,
            v: SKSPLDET2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSPLDET3\0" as *const u8 as *const libc::c_char,
            v: SKSPLDET3 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSPLDET4\0" as *const u8 as *const libc::c_char,
            v: SKSPLDET4 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSPLVCT1\0" as *const u8 as *const libc::c_char,
            v: SKSPLVCT1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSPLVCT2\0" as *const u8 as *const libc::c_char,
            v: SKSPLVCT2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSPLVCT3\0" as *const u8 as *const libc::c_char,
            v: SKSPLVCT3 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSPLVCT4\0" as *const u8 as *const libc::c_char,
            v: SKSPLVCT4 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSTHOK\0" as *const u8 as *const libc::c_char,
            v: SKSTHOK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSSPNDSH\0" as *const u8 as *const libc::c_char,
            v: SKSSPNDSH as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSZOOM\0" as *const u8 as *const libc::c_char,
            v: SKSZOOM as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSSKID\0" as *const u8 as *const libc::c_char,
            v: SKSSKID as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSGASP\0" as *const u8 as *const libc::c_char,
            v: SKSGASP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SKSJUMP\0" as *const u8 as *const libc::c_char,
            v: SKSJUMP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_EXISTS\0" as *const u8 as *const libc::c_char,
            v: FOF_EXISTS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_BLOCKPLAYER\0" as *const u8 as *const libc::c_char,
            v: FOF_BLOCKPLAYER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_BLOCKOTHERS\0" as *const u8 as *const libc::c_char,
            v: FOF_BLOCKOTHERS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_SOLID\0" as *const u8 as *const libc::c_char,
            v: FOF_SOLID as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_RENDERSIDES\0" as *const u8 as *const libc::c_char,
            v: FOF_RENDERSIDES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_RENDERPLANES\0" as *const u8 as *const libc::c_char,
            v: FOF_RENDERPLANES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_RENDERALL\0" as *const u8 as *const libc::c_char,
            v: FOF_RENDERALL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_SWIMMABLE\0" as *const u8 as *const libc::c_char,
            v: FOF_SWIMMABLE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_NOSHADE\0" as *const u8 as *const libc::c_char,
            v: FOF_NOSHADE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_CUTSOLIDS\0" as *const u8 as *const libc::c_char,
            v: FOF_CUTSOLIDS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_CUTEXTRA\0" as *const u8 as *const libc::c_char,
            v: FOF_CUTEXTRA as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_CUTLEVEL\0" as *const u8 as *const libc::c_char,
            v: FOF_CUTLEVEL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_CUTSPRITES\0" as *const u8 as *const libc::c_char,
            v: FOF_CUTSPRITES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_BOTHPLANES\0" as *const u8 as *const libc::c_char,
            v: FOF_BOTHPLANES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_EXTRA\0" as *const u8 as *const libc::c_char,
            v: FOF_EXTRA as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_TRANSLUCENT\0" as *const u8 as *const libc::c_char,
            v: FOF_TRANSLUCENT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_FOG\0" as *const u8 as *const libc::c_char,
            v: FOF_FOG as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_INVERTPLANES\0" as *const u8 as *const libc::c_char,
            v: FOF_INVERTPLANES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_ALLSIDES\0" as *const u8 as *const libc::c_char,
            v: FOF_ALLSIDES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_INVERTSIDES\0" as *const u8 as *const libc::c_char,
            v: FOF_INVERTSIDES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_DOUBLESHADOW\0" as *const u8 as *const libc::c_char,
            v: FOF_DOUBLESHADOW as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_FLOATBOB\0" as *const u8 as *const libc::c_char,
            v: FOF_FLOATBOB as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_NORETURN\0" as *const u8 as *const libc::c_char,
            v: FOF_NORETURN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_CRUMBLE\0" as *const u8 as *const libc::c_char,
            v: FOF_CRUMBLE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_GOOWATER\0" as *const u8 as *const libc::c_char,
            v: FOF_GOOWATER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_MARIO\0" as *const u8 as *const libc::c_char,
            v: FOF_MARIO as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_BUSTUP\0" as *const u8 as *const libc::c_char,
            v: FOF_BUSTUP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_QUICKSAND\0" as *const u8 as *const libc::c_char,
            v: FOF_QUICKSAND as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_PLATFORM\0" as *const u8 as *const libc::c_char,
            v: FOF_PLATFORM as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_REVERSEPLATFORM\0" as *const u8 as *const libc::c_char,
            v: FOF_REVERSEPLATFORM as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_INTANGIBLEFLATS\0" as *const u8 as *const libc::c_char,
            v: FOF_INTANGIBLEFLATS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_RIPPLE\0" as *const u8 as *const libc::c_char,
            v: FOF_RIPPLE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_COLORMAPONLY\0" as *const u8 as *const libc::c_char,
            v: FOF_COLORMAPONLY as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_BOUNCY\0" as *const u8 as *const libc::c_char,
            v: FOF_BOUNCY as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FOF_SPLAT\0" as *const u8 as *const libc::c_char,
            v: FOF_SPLAT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_EXISTS\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_EXISTS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_BLOCKPLAYER\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_BLOCKPLAYER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_BLOCKOTHERS\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_BLOCKOTHERS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_SOLID\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_SOLID as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_RENDERSIDES\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_RENDERSIDES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_RENDERPLANES\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_RENDERPLANES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_RENDERALL\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_RENDERALL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_SWIMMABLE\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_SWIMMABLE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_NOSHADE\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_NOSHADE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_CUTSOLIDS\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_CUTSOLIDS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_CUTEXTRA\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_CUTEXTRA as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_CUTLEVEL\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_CUTLEVEL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_CUTSPRITES\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_CUTSPRITES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_BOTHPLANES\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_BOTHPLANES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_EXTRA\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_EXTRA as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_TRANSLUCENT\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_TRANSLUCENT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_FOG\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_FOG as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_INVERTPLANES\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_INVERTPLANES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_ALLSIDES\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_ALLSIDES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_INVERTSIDES\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_INVERTSIDES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_DOUBLESHADOW\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_DOUBLESHADOW as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_FLOATBOB\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_FLOATBOB as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_NORETURN\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_NORETURN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_CRUMBLE\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_CRUMBLE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_SHATTERBOTTOM\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_SHATTERBOTTOM as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_GOOWATER\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_GOOWATER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_MARIO\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_MARIO as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_BUSTUP\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_BUSTUP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_QUICKSAND\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_QUICKSAND as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_PLATFORM\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_PLATFORM as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_REVERSEPLATFORM\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_REVERSEPLATFORM as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_INTANGIBLEFLATS\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_INTANGIBLEFLATS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_INTANGABLEFLATS\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_INTANGIBLEFLATS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_SHATTER\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_SHATTER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_SPINBUST\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_SPINBUST as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_STRONGBUST\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_STRONGBUST as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_RIPPLE\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_RIPPLE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FF_COLORMAPONLY\0" as *const u8 as *const libc::c_char,
            v: FF_OLD_COLORMAPONLY as libc::c_uint as lua_Number,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FB_PUSHABLES\0" as *const u8 as *const libc::c_char,
            v: FB_PUSHABLES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FB_EXECUTOR\0" as *const u8 as *const libc::c_char,
            v: FB_EXECUTOR as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"FB_ONLYBOTTOM\0" as *const u8 as *const libc::c_char,
            v: FB_ONLYBOTTOM as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BT_TOUCH\0" as *const u8 as *const libc::c_char,
            v: BT_TOUCH as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BT_SPINBUST\0" as *const u8 as *const libc::c_char,
            v: BT_SPINBUST as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BT_REGULAR\0" as *const u8 as *const libc::c_char,
            v: BT_REGULAR as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BT_STRONG\0" as *const u8 as *const libc::c_char,
            v: BT_STRONG as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"POF_CLIPLINES\0" as *const u8 as *const libc::c_char,
            v: POF_CLIPLINES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"POF_CLIPPLANES\0" as *const u8 as *const libc::c_char,
            v: POF_CLIPPLANES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"POF_SOLID\0" as *const u8 as *const libc::c_char,
            v: POF_SOLID as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"POF_TESTHEIGHT\0" as *const u8 as *const libc::c_char,
            v: POF_TESTHEIGHT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"POF_RENDERSIDES\0" as *const u8 as *const libc::c_char,
            v: POF_RENDERSIDES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"POF_RENDERTOP\0" as *const u8 as *const libc::c_char,
            v: POF_RENDERTOP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"POF_RENDERBOTTOM\0" as *const u8 as *const libc::c_char,
            v: POF_RENDERBOTTOM as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"POF_RENDERPLANES\0" as *const u8 as *const libc::c_char,
            v: POF_RENDERPLANES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"POF_RENDERALL\0" as *const u8 as *const libc::c_char,
            v: POF_RENDERALL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"POF_INVERT\0" as *const u8 as *const libc::c_char,
            v: POF_INVERT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"POF_INVERTPLANES\0" as *const u8 as *const libc::c_char,
            v: POF_INVERTPLANES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"POF_INVERTPLANESONLY\0" as *const u8 as *const libc::c_char,
            v: POF_INVERTPLANESONLY as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"POF_PUSHABLESTOP\0" as *const u8 as *const libc::c_char,
            v: POF_PUSHABLESTOP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"POF_LDEXEC\0" as *const u8 as *const libc::c_char,
            v: POF_LDEXEC as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"POF_ONESIDE\0" as *const u8 as *const libc::c_char,
            v: POF_ONESIDE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"POF_NOSPECIALS\0" as *const u8 as *const libc::c_char,
            v: POF_NOSPECIALS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"POF_SPLAT\0" as *const u8 as *const libc::c_char,
            v: POF_SPLAT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SL_NOPHYSICS\0" as *const u8 as *const libc::c_char,
            v: SL_NOPHYSICS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"SL_DYNAMIC\0" as *const u8 as *const libc::c_char,
            v: SL_DYNAMIC as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANG1\0" as *const u8 as *const libc::c_char,
            v: 0xb60b61 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANG2\0" as *const u8 as *const libc::c_char,
            v: 0x16c16c1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANG10\0" as *const u8 as *const libc::c_char,
            v: 0x71c71c7 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANG15\0" as *const u8 as *const libc::c_char,
            v: 0xaaaaaab as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANG20\0" as *const u8 as *const libc::c_char,
            v: 0xe38e38e as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANG30\0" as *const u8 as *const libc::c_char,
            v: 0x15555555 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANG60\0" as *const u8 as *const libc::c_char,
            v: 0x2aaaaaab as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANG64h\0" as *const u8 as *const libc::c_char,
            v: 0x2dddddde as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANG105\0" as *const u8 as *const libc::c_char,
            v: 0x4aaaaaab as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANG210\0" as *const u8 as *const libc::c_char,
            v: 0x95555555 as libc::c_uint as lua_Number,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANG255\0" as *const u8 as *const libc::c_char,
            v: 0xb5555555 as libc::c_uint as lua_Number,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANG340\0" as *const u8 as *const libc::c_char,
            v: 0xf1c71c72 as libc::c_uint as lua_Number,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANG350\0" as *const u8 as *const libc::c_char,
            v: 0xf8e38e39 as libc::c_uint as lua_Number,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANGLE_11hh\0" as *const u8 as *const libc::c_char,
            v: 0x8000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANGLE_22h\0" as *const u8 as *const libc::c_char,
            v: 0x10000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANGLE_45\0" as *const u8 as *const libc::c_char,
            v: 0x20000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANGLE_67h\0" as *const u8 as *const libc::c_char,
            v: 0x30000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANGLE_90\0" as *const u8 as *const libc::c_char,
            v: 0x40000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANGLE_112h\0" as *const u8 as *const libc::c_char,
            v: 0x50000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANGLE_135\0" as *const u8 as *const libc::c_char,
            v: 0x60000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANGLE_157h\0" as *const u8 as *const libc::c_char,
            v: 0x70000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANGLE_180\0" as *const u8 as *const libc::c_char,
            v: 0x80000000 as libc::c_uint as lua_Number,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANGLE_202h\0" as *const u8 as *const libc::c_char,
            v: 0x90000000 as libc::c_uint as lua_Number,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANGLE_225\0" as *const u8 as *const libc::c_char,
            v: 0xa0000000 as libc::c_uint as lua_Number,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANGLE_247h\0" as *const u8 as *const libc::c_char,
            v: 0xb0000000 as libc::c_uint as lua_Number,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANGLE_270\0" as *const u8 as *const libc::c_char,
            v: 0xc0000000 as libc::c_uint as lua_Number,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANGLE_292h\0" as *const u8 as *const libc::c_char,
            v: 0xd0000000 as libc::c_uint as lua_Number,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANGLE_315\0" as *const u8 as *const libc::c_char,
            v: 0xe0000000 as libc::c_uint as lua_Number,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANGLE_337h\0" as *const u8 as *const libc::c_char,
            v: 0xf0000000 as libc::c_uint as lua_Number,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ANGLE_MAX\0" as *const u8 as *const libc::c_char,
            v: 0xffffffff as libc::c_uint as lua_Number,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DI_NODIR\0" as *const u8 as *const libc::c_char,
            v: DI_NODIR as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DI_EAST\0" as *const u8 as *const libc::c_char,
            v: DI_EAST as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DI_NORTHEAST\0" as *const u8 as *const libc::c_char,
            v: DI_NORTHEAST as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DI_NORTH\0" as *const u8 as *const libc::c_char,
            v: DI_NORTH as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DI_NORTHWEST\0" as *const u8 as *const libc::c_char,
            v: DI_NORTHWEST as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DI_WEST\0" as *const u8 as *const libc::c_char,
            v: DI_WEST as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DI_SOUTHWEST\0" as *const u8 as *const libc::c_char,
            v: DI_SOUTHWEST as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DI_SOUTH\0" as *const u8 as *const libc::c_char,
            v: DI_SOUTH as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"DI_SOUTHEAST\0" as *const u8 as *const libc::c_char,
            v: DI_SOUTHEAST as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"NUMDIRS\0" as *const u8 as *const libc::c_char,
            v: NUMDIRS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ROTAXIS_X\0" as *const u8 as *const libc::c_char,
            v: ROTAXIS_X as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ROTAXIS_Y\0" as *const u8 as *const libc::c_char,
            v: ROTAXIS_Y as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"ROTAXIS_Z\0" as *const u8 as *const libc::c_char,
            v: ROTAXIS_Z as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BT_WEAPONMASK\0" as *const u8 as *const libc::c_char,
            v: BT_WEAPONMASK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BT_WEAPONNEXT\0" as *const u8 as *const libc::c_char,
            v: BT_WEAPONNEXT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BT_WEAPONPREV\0" as *const u8 as *const libc::c_char,
            v: BT_WEAPONPREV as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BT_ATTACK\0" as *const u8 as *const libc::c_char,
            v: BT_ATTACK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BT_SPIN\0" as *const u8 as *const libc::c_char,
            v: BT_SPIN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BT_CAMLEFT\0" as *const u8 as *const libc::c_char,
            v: BT_CAMLEFT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BT_CAMRIGHT\0" as *const u8 as *const libc::c_char,
            v: BT_CAMRIGHT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BT_TOSSFLAG\0" as *const u8 as *const libc::c_char,
            v: BT_TOSSFLAG as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BT_JUMP\0" as *const u8 as *const libc::c_char,
            v: BT_JUMP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BT_FIRENORMAL\0" as *const u8 as *const libc::c_char,
            v: BT_FIRENORMAL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BT_CUSTOM1\0" as *const u8 as *const libc::c_char,
            v: BT_CUSTOM1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BT_CUSTOM2\0" as *const u8 as *const libc::c_char,
            v: BT_CUSTOM2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"BT_CUSTOM3\0" as *const u8 as *const libc::c_char,
            v: BT_CUSTOM3 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"COM_ADMIN\0" as *const u8 as *const libc::c_char,
            v: COM_ADMIN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"COM_SPLITSCREEN\0" as *const u8 as *const libc::c_char,
            v: COM_SPLITSCREEN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"COM_LOCAL\0" as *const u8 as *const libc::c_char,
            v: COM_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CV_SAVE\0" as *const u8 as *const libc::c_char,
            v: CV_SAVE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CV_CALL\0" as *const u8 as *const libc::c_char,
            v: CV_CALL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CV_NETVAR\0" as *const u8 as *const libc::c_char,
            v: CV_NETVAR as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CV_NOINIT\0" as *const u8 as *const libc::c_char,
            v: CV_NOINIT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CV_FLOAT\0" as *const u8 as *const libc::c_char,
            v: CV_FLOAT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CV_NOTINNET\0" as *const u8 as *const libc::c_char,
            v: CV_NOTINNET as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CV_MODIFIED\0" as *const u8 as *const libc::c_char,
            v: CV_MODIFIED as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CV_SHOWMODIF\0" as *const u8 as *const libc::c_char,
            v: CV_SHOWMODIF as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CV_SHOWMODIFONETIME\0" as *const u8 as *const libc::c_char,
            v: CV_SHOWMODIFONETIME as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CV_NOSHOWHELP\0" as *const u8 as *const libc::c_char,
            v: CV_NOSHOWHELP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CV_HIDEN\0" as *const u8 as *const libc::c_char,
            v: CV_HIDEN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CV_HIDDEN\0" as *const u8 as *const libc::c_char,
            v: CV_HIDEN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CV_CHEAT\0" as *const u8 as *const libc::c_char,
            v: CV_CHEAT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"CV_ALLOWLUA\0" as *const u8 as *const libc::c_char,
            v: CV_ALLOWLUA as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_NOSCALEPATCH\0" as *const u8 as *const libc::c_char,
            v: 0x100 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_SMALLSCALEPATCH\0" as *const u8 as *const libc::c_char,
            v: 0x200 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_MEDSCALEPATCH\0" as *const u8 as *const libc::c_char,
            v: 0x300 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_6WIDTHSPACE\0" as *const u8 as *const libc::c_char,
            v: 0x400 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_OLDSPACING\0" as *const u8 as *const libc::c_char,
            v: 0x800 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_MONOSPACE\0" as *const u8 as *const libc::c_char,
            v: 0xc00 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_MAGENTAMAP\0" as *const u8 as *const libc::c_char,
            v: 0x1000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_YELLOWMAP\0" as *const u8 as *const libc::c_char,
            v: 0x2000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_GREENMAP\0" as *const u8 as *const libc::c_char,
            v: 0x3000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_BLUEMAP\0" as *const u8 as *const libc::c_char,
            v: 0x4000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_REDMAP\0" as *const u8 as *const libc::c_char,
            v: 0x5000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_GRAYMAP\0" as *const u8 as *const libc::c_char,
            v: 0x6000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_ORANGEMAP\0" as *const u8 as *const libc::c_char,
            v: 0x7000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_SKYMAP\0" as *const u8 as *const libc::c_char,
            v: 0x8000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_PURPLEMAP\0" as *const u8 as *const libc::c_char,
            v: 0x9000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_AQUAMAP\0" as *const u8 as *const libc::c_char,
            v: 0xa000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_PERIDOTMAP\0" as *const u8 as *const libc::c_char,
            v: 0xb000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_AZUREMAP\0" as *const u8 as *const libc::c_char,
            v: 0xc000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_BROWNMAP\0" as *const u8 as *const libc::c_char,
            v: 0xd000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_ROSYMAP\0" as *const u8 as *const libc::c_char,
            v: 0xe000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_INVERTMAP\0" as *const u8 as *const libc::c_char,
            v: 0xf000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_TRANSLUCENT\0" as *const u8 as *const libc::c_char,
            v: 0x50000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_10TRANS\0" as *const u8 as *const libc::c_char,
            v: 0x10000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_20TRANS\0" as *const u8 as *const libc::c_char,
            v: 0x20000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_30TRANS\0" as *const u8 as *const libc::c_char,
            v: 0x30000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_40TRANS\0" as *const u8 as *const libc::c_char,
            v: 0x40000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_50TRANS\0" as *const u8 as *const libc::c_char,
            v: 0x50000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_60TRANS\0" as *const u8 as *const libc::c_char,
            v: 0x60000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_70TRANS\0" as *const u8 as *const libc::c_char,
            v: 0x70000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_80TRANS\0" as *const u8 as *const libc::c_char,
            v: 0x80000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_90TRANS\0" as *const u8 as *const libc::c_char,
            v: 0x90000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_HUDTRANSHALF\0" as *const u8 as *const libc::c_char,
            v: 0xa0000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_HUDTRANS\0" as *const u8 as *const libc::c_char,
            v: 0xb0000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_HUDTRANSDOUBLE\0" as *const u8 as *const libc::c_char,
            v: 0xc0000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_BLENDSHIFT\0" as *const u8 as *const libc::c_char,
            v: 20 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_BLENDMASK\0" as *const u8 as *const libc::c_char,
            v: 0x700000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_ADD\0" as *const u8 as *const libc::c_char,
            v: (AST_ADD as libc::c_int - 1 as libc::c_int) << 20 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_SUBTRACT\0" as *const u8 as *const libc::c_char,
            v: (AST_SUBTRACT as libc::c_int - 1 as libc::c_int) << 20 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_REVERSESUBTRACT\0" as *const u8 as *const libc::c_char,
            v: (AST_REVERSESUBTRACT as libc::c_int - 1 as libc::c_int)
                << 20 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_MODULATE\0" as *const u8 as *const libc::c_char,
            v: (AST_MODULATE as libc::c_int - 1 as libc::c_int) << 20 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_ALLOWLOWERCASE\0" as *const u8 as *const libc::c_char,
            v: 0x800000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_FLIP\0" as *const u8 as *const libc::c_char,
            v: 0x800000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_CENTERNAMETAG\0" as *const u8 as *const libc::c_char,
            v: 0x800000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_SNAPTOTOP\0" as *const u8 as *const libc::c_char,
            v: 0x1000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_SNAPTOBOTTOM\0" as *const u8 as *const libc::c_char,
            v: 0x2000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_SNAPTOLEFT\0" as *const u8 as *const libc::c_char,
            v: 0x4000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_SNAPTORIGHT\0" as *const u8 as *const libc::c_char,
            v: 0x8000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_AUTOFADEOUT\0" as *const u8 as *const libc::c_char,
            v: 0x10000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_RETURN8\0" as *const u8 as *const libc::c_char,
            v: 0x20000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_NOSCALESTART\0" as *const u8 as *const libc::c_char,
            v: 0x40000000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_PERPLAYER\0" as *const u8 as *const libc::c_char,
            v: 0x80000000 as libc::c_uint as lua_Number,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_PARAMMASK\0" as *const u8 as *const libc::c_char,
            v: 0xff as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_SCALEPATCHMASK\0" as *const u8 as *const libc::c_char,
            v: 0x300 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_SPACINGMASK\0" as *const u8 as *const libc::c_char,
            v: 0xc00 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_CHARCOLORMASK\0" as *const u8 as *const libc::c_char,
            v: 0xf000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_ALPHAMASK\0" as *const u8 as *const libc::c_char,
            v: 0xf0000 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_CHARCOLORSHIFT\0" as *const u8 as *const libc::c_char,
            v: 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"V_ALPHASHIFT\0" as *const u8 as *const libc::c_char,
            v: 16 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"KR_KICK\0" as *const u8 as *const libc::c_char,
            v: KR_KICK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"KR_PINGLIMIT\0" as *const u8 as *const libc::c_char,
            v: KR_PINGLIMIT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"KR_SYNCH\0" as *const u8 as *const libc::c_char,
            v: KR_SYNCH as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"KR_TIMEOUT\0" as *const u8 as *const libc::c_char,
            v: KR_TIMEOUT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"KR_BAN\0" as *const u8 as *const libc::c_char,
            v: KR_BAN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"KR_LEAVE\0" as *const u8 as *const libc::c_char,
            v: KR_LEAVE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"TC_DEFAULT\0" as *const u8 as *const libc::c_char,
            v: TC_DEFAULT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"TC_BOSS\0" as *const u8 as *const libc::c_char,
            v: TC_BOSS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"TC_METALSONIC\0" as *const u8 as *const libc::c_char,
            v: TC_METALSONIC as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"TC_ALLWHITE\0" as *const u8 as *const libc::c_char,
            v: TC_ALLWHITE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"TC_RAINBOW\0" as *const u8 as *const libc::c_char,
            v: TC_RAINBOW as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"TC_BLINK\0" as *const u8 as *const libc::c_char,
            v: TC_BLINK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"TC_DASHMODE\0" as *const u8 as *const libc::c_char,
            v: TC_DASHMODE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MA_INIT\0" as *const u8 as *const libc::c_char,
            v: MA_INIT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MA_RUNNING\0" as *const u8 as *const libc::c_char,
            v: MA_RUNNING as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MA_NOCUTSCENES\0" as *const u8 as *const libc::c_char,
            v: MA_NOCUTSCENES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MA_INGAME\0" as *const u8 as *const libc::c_char,
            v: MA_INGAME as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MU_NONE\0" as *const u8 as *const libc::c_char,
            v: MU_NONE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MU_WAV\0" as *const u8 as *const libc::c_char,
            v: MU_WAV as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MU_MOD\0" as *const u8 as *const libc::c_char,
            v: MU_MOD as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MU_MID\0" as *const u8 as *const libc::c_char,
            v: MU_MID as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MU_OGG\0" as *const u8 as *const libc::c_char,
            v: MU_OGG as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MU_MP3\0" as *const u8 as *const libc::c_char,
            v: MU_MP3 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MU_FLAC\0" as *const u8 as *const libc::c_char,
            v: MU_FLAC as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MU_GME\0" as *const u8 as *const libc::c_char,
            v: MU_GME as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MU_MOD_EX\0" as *const u8 as *const libc::c_char,
            v: MU_MOD_EX as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MU_MID_EX\0" as *const u8 as *const libc::c_char,
            v: MU_MID_EX as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GS_NULL\0" as *const u8 as *const libc::c_char,
            v: GS_NULL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GS_LEVEL\0" as *const u8 as *const libc::c_char,
            v: GS_LEVEL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GS_INTERMISSION\0" as *const u8 as *const libc::c_char,
            v: GS_INTERMISSION as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GS_CONTINUING\0" as *const u8 as *const libc::c_char,
            v: GS_CONTINUING as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GS_TITLESCREEN\0" as *const u8 as *const libc::c_char,
            v: GS_TITLESCREEN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GS_TIMEATTACK\0" as *const u8 as *const libc::c_char,
            v: GS_TIMEATTACK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GS_CREDITS\0" as *const u8 as *const libc::c_char,
            v: GS_CREDITS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GS_EVALUATION\0" as *const u8 as *const libc::c_char,
            v: GS_EVALUATION as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GS_GAMEEND\0" as *const u8 as *const libc::c_char,
            v: GS_GAMEEND as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GS_INTRO\0" as *const u8 as *const libc::c_char,
            v: GS_INTRO as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GS_ENDING\0" as *const u8 as *const libc::c_char,
            v: GS_ENDING as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GS_CUTSCENE\0" as *const u8 as *const libc::c_char,
            v: GS_CUTSCENE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GS_DEDICATEDSERVER\0" as *const u8 as *const libc::c_char,
            v: GS_DEDICATEDSERVER as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GS_WAITINGPLAYERS\0" as *const u8 as *const libc::c_char,
            v: GS_WAITINGPLAYERS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JA_NONE\0" as *const u8 as *const libc::c_char,
            v: JA_NONE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JA_TURN\0" as *const u8 as *const libc::c_char,
            v: JA_TURN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JA_MOVE\0" as *const u8 as *const libc::c_char,
            v: JA_MOVE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JA_LOOK\0" as *const u8 as *const libc::c_char,
            v: JA_LOOK as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JA_STRAFE\0" as *const u8 as *const libc::c_char,
            v: JA_STRAFE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JA_DIGITAL\0" as *const u8 as *const libc::c_char,
            v: JA_DIGITAL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JA_JUMP\0" as *const u8 as *const libc::c_char,
            v: JA_JUMP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JA_SPIN\0" as *const u8 as *const libc::c_char,
            v: JA_SPIN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JA_FIRE\0" as *const u8 as *const libc::c_char,
            v: JA_FIRE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JA_FIRENORMAL\0" as *const u8 as *const libc::c_char,
            v: JA_FIRENORMAL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"JOYAXISRANGE\0" as *const u8 as *const libc::c_char,
            v: 1023 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_NULL\0" as *const u8 as *const libc::c_char,
            v: GC_NULL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_FORWARD\0" as *const u8 as *const libc::c_char,
            v: GC_FORWARD as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_BACKWARD\0" as *const u8 as *const libc::c_char,
            v: GC_BACKWARD as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_STRAFELEFT\0" as *const u8 as *const libc::c_char,
            v: GC_STRAFELEFT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_STRAFERIGHT\0" as *const u8 as *const libc::c_char,
            v: GC_STRAFERIGHT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_TURNLEFT\0" as *const u8 as *const libc::c_char,
            v: GC_TURNLEFT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_TURNRIGHT\0" as *const u8 as *const libc::c_char,
            v: GC_TURNRIGHT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_WEAPONNEXT\0" as *const u8 as *const libc::c_char,
            v: GC_WEAPONNEXT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_WEAPONPREV\0" as *const u8 as *const libc::c_char,
            v: GC_WEAPONPREV as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_WEPSLOT1\0" as *const u8 as *const libc::c_char,
            v: GC_WEPSLOT1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_WEPSLOT2\0" as *const u8 as *const libc::c_char,
            v: GC_WEPSLOT2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_WEPSLOT3\0" as *const u8 as *const libc::c_char,
            v: GC_WEPSLOT3 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_WEPSLOT4\0" as *const u8 as *const libc::c_char,
            v: GC_WEPSLOT4 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_WEPSLOT5\0" as *const u8 as *const libc::c_char,
            v: GC_WEPSLOT5 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_WEPSLOT6\0" as *const u8 as *const libc::c_char,
            v: GC_WEPSLOT6 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_WEPSLOT7\0" as *const u8 as *const libc::c_char,
            v: GC_WEPSLOT7 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_WEPSLOT8\0" as *const u8 as *const libc::c_char,
            v: GC_WEPSLOT8 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_WEPSLOT9\0" as *const u8 as *const libc::c_char,
            v: GC_WEPSLOT9 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_WEPSLOT10\0" as *const u8 as *const libc::c_char,
            v: GC_WEPSLOT10 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_FIRE\0" as *const u8 as *const libc::c_char,
            v: GC_FIRE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_FIRENORMAL\0" as *const u8 as *const libc::c_char,
            v: GC_FIRENORMAL as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_TOSSFLAG\0" as *const u8 as *const libc::c_char,
            v: GC_TOSSFLAG as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_SPIN\0" as *const u8 as *const libc::c_char,
            v: GC_SPIN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_CAMTOGGLE\0" as *const u8 as *const libc::c_char,
            v: GC_CAMTOGGLE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_CAMRESET\0" as *const u8 as *const libc::c_char,
            v: GC_CAMRESET as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_LOOKUP\0" as *const u8 as *const libc::c_char,
            v: GC_LOOKUP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_LOOKDOWN\0" as *const u8 as *const libc::c_char,
            v: GC_LOOKDOWN as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_CENTERVIEW\0" as *const u8 as *const libc::c_char,
            v: GC_CENTERVIEW as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_MOUSEAIMING\0" as *const u8 as *const libc::c_char,
            v: GC_MOUSEAIMING as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_TALKKEY\0" as *const u8 as *const libc::c_char,
            v: GC_TALKKEY as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_TEAMKEY\0" as *const u8 as *const libc::c_char,
            v: GC_TEAMKEY as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_SCORES\0" as *const u8 as *const libc::c_char,
            v: GC_SCORES as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_JUMP\0" as *const u8 as *const libc::c_char,
            v: GC_JUMP as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_CONSOLE\0" as *const u8 as *const libc::c_char,
            v: GC_CONSOLE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_PAUSE\0" as *const u8 as *const libc::c_char,
            v: GC_PAUSE as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_SYSTEMMENU\0" as *const u8 as *const libc::c_char,
            v: GC_SYSTEMMENU as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_SCREENSHOT\0" as *const u8 as *const libc::c_char,
            v: GC_SCREENSHOT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_RECORDGIF\0" as *const u8 as *const libc::c_char,
            v: GC_RECORDGIF as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_VIEWPOINTNEXT\0" as *const u8 as *const libc::c_char,
            v: GC_VIEWPOINTNEXT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_VIEWPOINT\0" as *const u8 as *const libc::c_char,
            v: GC_VIEWPOINTNEXT as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_VIEWPOINTPREV\0" as *const u8 as *const libc::c_char,
            v: GC_VIEWPOINTPREV as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_CUSTOM1\0" as *const u8 as *const libc::c_char,
            v: GC_CUSTOM1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_CUSTOM2\0" as *const u8 as *const libc::c_char,
            v: GC_CUSTOM2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"GC_CUSTOM3\0" as *const u8 as *const libc::c_char,
            v: GC_CUSTOM3 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"NUM_GAMECONTROLS\0" as *const u8 as *const libc::c_char,
            v: NUM_GAMECONTROLS as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MB_BUTTON1\0" as *const u8 as *const libc::c_char,
            v: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MB_BUTTON2\0" as *const u8 as *const libc::c_char,
            v: 0x2 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MB_BUTTON3\0" as *const u8 as *const libc::c_char,
            v: 0x4 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MB_BUTTON4\0" as *const u8 as *const libc::c_char,
            v: 0x8 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MB_BUTTON5\0" as *const u8 as *const libc::c_char,
            v: 0x10 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MB_BUTTON6\0" as *const u8 as *const libc::c_char,
            v: 0x20 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MB_BUTTON7\0" as *const u8 as *const libc::c_char,
            v: 0x40 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MB_BUTTON8\0" as *const u8 as *const libc::c_char,
            v: 0x80 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MB_SCROLLUP\0" as *const u8 as *const libc::c_char,
            v: 0x100 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: b"MB_SCROLLDOWN\0" as *const u8 as *const libc::c_char,
            v: 0x200 as libc::c_int,
        };
        init
    },
    {
        let mut init = int_const_s {
            n: 0 as *const libc::c_char,
            v: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn DEH_TableCheck() {}
