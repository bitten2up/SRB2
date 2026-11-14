// Emacs style mode select   -*- C++ -*-
//-----------------------------------------------------------------------------
//
// Copyright (C) 1993-1996 by id Software, Inc.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// The source is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//-----------------------------------------------------------------------------
/// \file
/// \brief NDS interface for sound

// uses vorbisidec to decode OGG files
// obviously because of that this only works on OGG files (most mods already use them)
#ifdef __3DS__
#include <tremor/ivorbisfile.h>
#else
#include <vorbis/vorbisfile.h>
#endif
#include <SDL2/SDL.h>
#include "../i_sound.h"
#include "../s_sound.h"

OggVorbis_File vorbis;
vorbis_info *info = NULL;

SDL_AudioDeviceID audio_device = 0;

UINT8 sound_started = 0;
boolean music_loaded = false;
typedef struct
{
    unsigned char *data;
    size_t size;
    size_t pos;
} MemFile;


size_t ov_read_func(void *ptr, size_t size, size_t nmemb, void *datasource)
{
    MemFile *file = datasource;

    size_t bytes = size * nmemb;

    if (file->pos + bytes > file->size)
        bytes = file->size - file->pos;

    memcpy(ptr, file->data + file->pos, bytes);

    file->pos += bytes;

    return bytes / size;
}


int ov_seek_func(void *datasource, ogg_int64_t offset, int whence)
{
    MemFile *file = datasource;

    size_t newpos;

    if (whence == SEEK_SET)
        newpos = offset;
    else if (whence == SEEK_CUR)
        newpos = file->pos + offset;
    else
        newpos = file->size + offset;


    if (newpos > file->size)
        return -1;

    file->pos = newpos;

    return 0;
}


int ov_close_func(void *datasource)
{
    return 0;
}


long ov_tell_func(void *datasource)
{
    MemFile *file = datasource;
    return file->pos;
}
void *I_GetSfx(sfxinfo_t *sfx)
{
	(void)sfx;
	return NULL;
}

void I_FreeSfx(sfxinfo_t *sfx)
{
	(void)sfx;
}

void I_StartupSound(void){}

void I_ShutdownSound(void){}

void I_UpdateSound(void){};

//
//  SFX I/O
//

INT32 I_StartSound(sfxenum_t id, UINT8 vol, UINT8 sep, UINT8 pitch, UINT8 priority, INT32 channel)
{
	(void)id;
	(void)vol;
	(void)sep;
	(void)pitch;
	(void)priority;
	(void)channel;
	return -1;
}

void I_StopSound(INT32 handle)
{
	(void)handle;
}

boolean I_SoundIsPlaying(INT32 handle)
{
	(void)handle;
	return false;
}

void I_UpdateSoundParams(INT32 handle, UINT8 vol, UINT8 sep, UINT8 pitch)
{
	(void)handle;
	(void)vol;
	(void)sep;
	(void)pitch;
}

void I_SetSfxVolume(UINT8 volume)
{
	(void)volume;
}

/// ------------------------
//  MUSIC SYSTEM
/// ------------------------

void I_InitMusic(void)
{
    SDL_Init(SDL_INIT_AUDIO);

    SDL_AudioSpec spec;

    SDL_zero(spec);

    spec.freq = 44100;
    spec.format = AUDIO_S16SYS;
    spec.channels = 2;
    spec.samples = 2048;

    audio_device = SDL_OpenAudioDevice(
        NULL,
        0,
        &spec,
        NULL,
        0
    );

    if (!audio_device)
        printf("SDL audio failed: %s\n", SDL_GetError());
}

void I_ShutdownMusic(void){}

/// ------------------------
//  MUSIC PROPERTIES
/// ------------------------

musictype_t I_SongType(void)
{
	return MU_NONE;
}

boolean I_SongPlaying(void)
{
	return false;
}

boolean I_SongPaused(void)
{
	return false;
}

/// ------------------------
//  MUSIC EFFECTS
/// ------------------------

boolean I_SetSongSpeed(float speed)
{
	(void)speed;
	return false;
}

/// ------------------------
//  MUSIC SEEKING
/// ------------------------

UINT32 I_GetSongLength(void)
{
	return 0;
}

boolean I_SetSongLoopPoint(UINT32 looppoint)
{
        (void)looppoint;
        return false;
}

UINT32 I_GetSongLoopPoint(void)
{
	return 0;
}

boolean I_SetSongPosition(UINT32 position)
{
    (void)position;
    return false;
}

UINT32 I_GetSongPosition(void)
{
    return 0;
}

/// ------------------------
//  MUSIC PLAYBACK
/// ------------------------

MemFile music_file;


boolean I_LoadSong(char *data, size_t len)
{
    if (!cv_gamedigimusic.value)
        return true;


    music_file.data = (unsigned char *)data;
    music_file.size = len;
    music_file.pos = 0;


    ov_callbacks callbacks;

    callbacks.read_func = ov_read_func;
    callbacks.seek_func = ov_seek_func;
    callbacks.close_func = ov_close_func;
    callbacks.tell_func = ov_tell_func;


    if (ov_open_callbacks(
        &music_file,
        &vorbis,
        NULL,
        0,
        callbacks) < 0)
    {
        return false;
    }


    info = ov_info(&vorbis, -1);

    music_loaded = true;

    return true;
}


void I_UnloadSong(void)
{
    if (!cv_gamedigimusic.value)
        return;


    I_StopSong();

    if (music_loaded)
    {
        ov_clear(&vorbis);
        music_loaded = false;
    }
}

boolean I_PlaySong(boolean looping)
{
    (void)looping;


    if (!music_loaded)
        return false;


    I_StopSong();


    SDL_PauseAudioDevice(audio_device, 0);


    char buffer[4096];

    int bitstream;


    while (1)
    {
        long bytes = ov_read(
            &vorbis,
            buffer,
            sizeof(buffer),
						#ifndef __3DS__
						0,
						2,
						1,
						#endif
            &bitstream
        );


        if (bytes <= 0)
            break;


        SDL_QueueAudio(
            audio_device,
            buffer,
            bytes
        );
    }


    return true;
}

void I_StopSong(void)
{
    if (audio_device)
        SDL_ClearQueuedAudio(audio_device);
}


void I_PauseSong(void)
{
}

void I_ResumeSong(void)
{
}

void I_SetMusicVolume(UINT8 volume)
{
	(void)volume;
}

boolean I_SetSongTrack(INT32 track)
{
	(void)track;
	return false;
}

/// ------------------------
//  MUSIC FADING
/// ------------------------

void I_SetInternalMusicVolume(UINT8 volume)
{
	(void)volume;
}

void I_StopFadingSong(void)
{
}

boolean I_FadeSongFromVolume(UINT8 target_volume, UINT8 source_volume, UINT32 ms, void (*callback)(void))
{
	(void)target_volume;
	(void)source_volume;
	(void)ms;
	(void)callback;
	return false;
}

boolean I_FadeSong(UINT8 target_volume, UINT32 ms, void (*callback)(void))
{
	(void)target_volume;
	(void)ms;
	(void)callback;
	return false;
}

boolean I_FadeOutStopSong(UINT32 ms)
{
	(void)ms;
	return false;
}

boolean I_FadeInPlaySong(UINT32 ms, boolean looping)
{
        (void)ms;
        (void)looping;
        return false;
}
