#include "../inc/cursor.hpp"
#include "../inc/utils.hpp"

#include "../config.hpp"


SDL_FRect
RenderCursor::Get_Cursor_Rect(
    SDL_Renderer *renderer,
    SDL_Color color,
    float x,
    float y,
    float w,
    float h
)
{
    SDL_SetRenderDrawColor(
        renderer,
        color.r,
        color.g,
        color.b,
        color.a
    );

    return {
        x,
        y,
        (float)w,
        (float)h,
    };
}


bool
RenderCursor::Hollow(SDL_Color color, AppData *AppData, Offset Offset)
{
    int32_t h;
    int32_t w;
    Get_Font_Size(AppData->font, &w, &h);

    float y = Offset.y + (AppData->EditorData.cursor.y * h);
    float x = Offset.x + (AppData->EditorData.cursor.x * w);

    SDL_FRect cursor = Get_Cursor_Rect(AppData->renderer, color, x, y, w, h);
    if (!SDL_RenderRect(AppData->renderer, &cursor)) {
        Log_Err("Failed to fill cursor surface");
        return false;
    }

    return true;
}


bool
RenderCursor::Box(SDL_Color color, AppData *AppData, Offset Offset)
{
    int32_t h;
    int32_t w;
    Get_Font_Size(AppData->font, &w, &h);

    float y = Offset.y + (AppData->EditorData.cursor.y * h);
    float x = Offset.x + (AppData->EditorData.cursor.x * w);

    SDL_FRect cursor = Get_Cursor_Rect(AppData->renderer, color, x, y, w, h);

    SDL_Surface *surface = SDL_CreateSurface(w, h, SDL_PIXELFORMAT_XRGB32);
    if (!surface) {
        Log_Err("Failed to create cursor surface");
        return false;
    }

    if (
        !SDL_FillSurfaceRect(
            surface,
            NULL,
            SDL_MapSurfaceRGBA(surface, color.r, color.g, color.b, color.a)
        )
    ) {
        Log_Err("Failed to fill cursor surface");
        return false;
    }

    SDL_Texture *texture = SDL_CreateTextureFromSurface(AppData->renderer, surface);
    SDL_DestroySurface(surface);
    if (!texture) {
        Log_Err("Failed to create texture");
        return false;
    }

    if (!SDL_RenderTexture(AppData->renderer, texture, NULL, &cursor)) {
        Log_Err("Failed to render cursor");
        SDL_DestroyTexture(texture);
        return false;
    }

    SDL_DestroyTexture(texture);
    return true;
}


bool
RenderCursor::Beam(SDL_Color color, AppData *AppData, Offset Offset)
{
    int32_t h;
    int32_t w;
    if (!Get_Font_Size(AppData->font, &w, &h)) return false;

    float y = Offset.y + (AppData->EditorData.cursor.y * h);
    float x = Offset.x + (AppData->EditorData.cursor.x * w) + w;

    SDL_FRect cursor = Get_Cursor_Rect(AppData->renderer, color, x, y, cursor_width, h);

    if (!SDL_RenderRect(AppData->renderer, &cursor)) {
        Log_Err("Failed to render cursor rect");
        return false;
    }
    return true;
}


bool
RenderCursor::Line(SDL_Color color, AppData *AppData, Offset Offset)
{
    int32_t h;
    int32_t w;
    if (!Get_Font_Size(AppData->font, &w, &h)) return false;

    float y = Offset.y + (AppData->EditorData.cursor.y * h) + h;
    float x = Offset.x + (AppData->EditorData.cursor.x * w);

    h = 1;

    SDL_FRect cursor = Get_Cursor_Rect(AppData->renderer, color, x, y, w, h);

    if (!SDL_RenderRect(AppData->renderer, &cursor)) {
        Log_Err("Failed to render cursor rect");
        return false;
    }
    return true;
}