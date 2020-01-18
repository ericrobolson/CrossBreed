#include <stdio.h>
#include <stdbool.h>

#include "SDL2/SDL.h"
#include "std.h"
#include "window.h"

#define SCREEN_WIDTH 640
#define SCREEN_HEIGHT 480

int main(int argc, char *argv[])
{

    struct SdlContext *windowContext = SdlContext_New(640, 480);

    if (!windowContext)
    {
        return 1;
    }

    windowContext->surface = SDL_GetWindowSurface(windowContext->window);
    SDL_FillRect(windowContext->surface, NULL, SDL_MapRGB(windowContext->surface->format, 0xFF, 0xFF, 0xFF));
    SDL_UpdateWindowSurface(windowContext->window);
    SDL_Delay(2000);

    // Cleanup code
    SDL_DestroyWindow(windowContext->window);
    SDL_Quit();

    return 0;
}