////////////////////////////////////////////////////////////
// Dll testing                                            //
////////////////////////////////////////////////////////////
// Sample dll for patching                                //
////////////////////////////////////////////////////////////

#include <stdio.h>
#include <stdbool.h>
#include <pthread.h>
#include <string.h>
#include <time.h>
#include "../src/dll/dll_api.h"

int SRB2Entry(DLLpointers functions) {
    printf("hey i wonder if this will fucking work\n");
    *(functions.emeralds) = 7;
    return 10;
}
