#include "SDL2/SDL.h"
#include <stdio.h>

// Wrapping of SDL types
struct SdlContext
{
    SDL_Window *window;
    SDL_Surface *surface;
    SDL_GLContext glContext;
};

// Initialize a new SDL context
struct SdlContext *SdlContext_New(int widthInPixels, int heightInPixels);

// Destroy and deallocate a SDL context
void SdlContext_Destroy(struct SdlContext *ctx);