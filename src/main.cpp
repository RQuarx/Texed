#define SDL_MAIN_USE_CALLBACKS

#include <SDL3_ttf/SDL_ttf.h>
#include <SDL3/SDL_render.h>
#include <SDL3/SDL_hints.h>
#include <SDL3/SDL_main.h>
#include <SDL3/SDL.h>

#include "../inc/input_handler.hpp"
#include "../inc/decoration.hpp"
#include "../inc/arg_parse.hpp"
#include "../inc/app_data.hpp"
#include "../inc/movement.hpp"
#include "../inc/editor.hpp"
#include "../inc/utils.hpp"

#include "../config.hpp"

#include <fstream>

static const float ONE_SECOND_MS = 1000.0f;

static const char *APP_NAME = "texed";
static const char *APP_VERSION = "0.1.0";
static const char *APP_IDENTITY = "Simple Text Editor";

AppData AppData;


void
SDL_AppQuit(void *appstate, SDL_AppResult result)
{
    TTF_CloseFont(AppData.font);
    TTF_Quit();
    SDL_DestroyWindow(AppData.window);
    SDL_DestroyRenderer(AppData.renderer);
    SDL_QuitSubSystem(SDL_INIT_VIDEO);
}


SDL_AppResult
SDL_AppEvent(void *appstate, SDL_Event *event)
{
    switch (event->type) {
    case SDL_EVENT_QUIT:
        return SDL_APP_SUCCESS;

    case SDL_EVENT_TEXT_INPUT:
        AppData.EditorData.file_content[AppData.EditorData.cursor.y]
            .insert(AppData.EditorData.cursor.x, event->text.text);
        AppData.EditorData.cursor.x += SDL_strlen(event->text.text);
        AppData.changed = true;
        break;

    case SDL_EVENT_WINDOW_RESIZED:
        AppData.changed = true;
        break;

    case SDL_EVENT_WINDOW_MOUSE_ENTER:
        AppData.focused = true;
        AppData.changed = true;
        break;

    case SDL_EVENT_WINDOW_MOUSE_LEAVE:
        AppData.focused = false;
        AppData.changed = true;
        break;

    case SDL_EVENT_KEY_DOWN:
        AppData.changed = InputHandler::Handle(
            &AppData.EditorData,
            event->key.scancode
        );
        break;

    case SDL_EVENT_MOUSE_WHEEL:
        {
            int64_t buffer =
                AppData.EditorData.scroll_offset - (event->wheel.y * scroll_multiplier);
            if (buffer >= (int64_t)AppData.EditorData.file_content.size())
                buffer = AppData.EditorData.file_content.size() - 1;
            else if (buffer < 0)
                buffer = 0;

            AppData.EditorData.scroll_offset = buffer;
        }
        AppData.changed = true;
        break;

    default:
        break;
    }
    return SDL_APP_CONTINUE;
}


SDL_AppResult
SDL_AppIterate(void *appstate)
{
    if (!AppData.changed) return SDL_APP_CONTINUE;

    SDL_DisplayID *displays = SDL_GetDisplays(NULL);
    if (!displays) {
        Log_Err("Failed to get displays");
        return SDL_APP_FAILURE;
    }
    const SDL_DisplayMode *display_mode =
        SDL_GetCurrentDisplayMode(*displays);

    if (
        !SDL_SetRenderDrawColor(
            AppData.renderer,
            background.r,
            background.g,
            background.b,
            background.a
        )
    ) {
        Log_Err("Failed to set render color");
        return SDL_APP_FAILURE;
    }

    if (!SDL_RenderClear(AppData.renderer)) {
        Log_Err("Failed to clear renderer");
        return SDL_APP_FAILURE;
    }

    Offset offset = { 0, 0 };

    if (!Decoration::Draw_Decoration(&AppData, &offset))
        return SDL_APP_FAILURE;

    if (!Editor::Render_Loop(&AppData, offset))
        return SDL_APP_FAILURE;

    if (!SDL_RenderPresent(AppData.renderer)) {
        Log_Err("Failed to update renderer");
        return SDL_APP_FAILURE;
    }

    AppData.changed = false;

    SDL_Delay(ONE_SECOND_MS / display_mode->refresh_rate);

    return SDL_APP_CONTINUE;
}


SDL_AppResult
SDL_InitSDL(struct AppData &AppData)
{
    if (AppData.verbose) Log_Debug("Initialising video subsystem");
    if (!SDL_InitSubSystem(SDL_INIT_VIDEO)) {
        Log_Err("Video subsystem failed to init");
        return SDL_APP_FAILURE;
    }

    if (AppData.verbose) Log_Debug("Setting app metadata");
    if (!SDL_SetAppMetadata(APP_NAME, APP_VERSION, APP_IDENTITY)) {
        Log_Err("Failed to set metadata");
        return SDL_APP_FAILURE;
    }

    if (AppData.verbose) Log_Debug("Initialising SDL_TTF");
    if (!TTF_Init()) {
        Log_Err("Failed to init SDL_TTF");
        return SDL_APP_FAILURE;
    }

    if (AppData.verbose) Log_Debug("Loading fonts");
    AppData.font = TTF_OpenFont(font_file, 24);

    if (!AppData.font) {
        Log_Err("Failed to load font");
        return SDL_APP_FAILURE;
    }

    if (AppData.verbose) Log_Debug("Creating window and renderer");

    SDL_CreateWindowAndRenderer(
        APP_NAME,
        800,
        600,
        SDL_WINDOW_HIGH_PIXEL_DENSITY | SDL_WINDOW_RESIZABLE,
        &AppData.window,
        &AppData.renderer
    );

    if (!AppData.window) {
        Log_Err("Failed to create window");
        return SDL_APP_FAILURE;
    }

    if (!AppData.renderer) {
        Log_Err("Failed to create renderer");
        return SDL_APP_FAILURE;
    }

    SDL_SetRenderVSync(AppData.renderer, 1);
    return SDL_APP_CONTINUE;
}


SDL_AppResult
SDL_AppInit(void **appstate, int argc, char **argv)
{
    /* CLI Args */

    ArgParse arg_parse(argc, argv);

    if (arg_parse.Arg("-h","--help")) {
        std::printf(HELP_MSG);
        return SDL_APP_SUCCESS;
    }

    if (arg_parse.Arg("-v", "--version")) {
        std::printf("%s-%s", APP_NAME, APP_VERSION);
        return SDL_APP_SUCCESS;
    }

    AppData.verbose = arg_parse.Arg("-V", "--verbose");

    /* SDL Init */

    if (SDL_InitSDL(AppData) != SDL_APP_CONTINUE)
        return SDL_APP_FAILURE;

    /* File handling */

    if (AppData.verbose) Log_Debug("Fetching file path");
    auto file = arg_parse.Get_File_Path();

    if (!file) {
        file = "new_file.txt";
        std::ofstream text_file("new_file.txt");
        text_file << " ";
        text_file.close();
    }

    if (AppData.verbose) Log_Debug("Creating editor");
    AppData.EditorData = *Editor::Init_Editor(*file);
    AppData.changed = true;

    Log_Info("Initialisation complete");

    return SDL_APP_CONTINUE;
}