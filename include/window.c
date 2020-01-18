#include "SDL2/SDL.h"
#include "window.h"
#include <stdio.h>

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
        SDL_WINDOW_OPENGL | SDL_WINDOW_SHOWN);
    if (!window)
    {
        fprintf(stderr, "could not create window: %s\n", SDL_GetError());
        return NULL;
    }

    SDL_GLContext glcontext = SDL_GL_CreateContext(window);

    struct SdlContext *ctx = malloc(sizeof(struct SdlContext));

    ctx->surface = screenSurface;
    ctx->window = window;
    ctx->glContext = glcontext;

    return ctx;
}

void SdlContext_Destroy(struct SdlContext *ctx)
{
    if (!ctx)
    {
        return;
    }

    SDL_GL_DeleteContext(ctx->glContext);
    SDL_DestroyWindow(ctx->window);
    SDL_Quit();

    free(ctx);
}