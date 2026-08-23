#ifndef DLL_POINTERS_H
#define DLL_POINTERS_H
#include <stdint.h>

typedef void (*voidchar)(char const*);
typedef struct
{
  uint16_t *emeralds;
} DLLpointers;

typedef struct
{
  int HandleChat;
};

#endif
