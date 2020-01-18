#include "SDL2/SDL.h"
#include <stdio.h>

// Wrapping of SDL types
struct SdlContext
{
    SDL_Window *window;
    SDL_Surface *surface;
};

// Initialize a new SDL context
struct SdlContext *SdlContext_New(int widthInPixels, int heightInPixels)
{
    SDL_Window *window = NULL;
    SDL_Surface *screenSurface = NULL;
    if (SDL_Init(SDL_INIT_VIDEO) < 0)
    {
        fprintf(stderr, "could not initialize sdl2: %s\n", SDL_GetError());
        return NULL;
    }
    window = SDL_CreateWindow(
        "hello_sdl2",
        SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED,
        widthInPixels, heightInPixels,
        SDL_WINDOW_SHOWN);
    if (!window)
    {
        fprintf(stderr, "could not create window: %s\n", SDL_GetError());
        return NULL;
    }
}