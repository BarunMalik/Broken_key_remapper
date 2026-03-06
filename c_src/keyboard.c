#include <windows.h>
#include "keyboard.h"

static HHOOK keyboard_hook = NULL;
static key_callback callback = NULL;
static volatile LONG suppress_injected_events = 1;

static HANDLE hook_thread = NULL;
static DWORD hook_thread_id = 0;

void register_key_callback(key_callback cb)
{
    callback = cb;
}

LRESULT CALLBACK keyboard_proc(int nCode, WPARAM wParam, LPARAM lParam)
{
    if (nCode == HC_ACTION && callback != NULL)
    {
        KBDLLHOOKSTRUCT *p = (KBDLLHOOKSTRUCT *)lParam;

        if (suppress_injected_events && (p->flags & LLKHF_INJECTED))
            return CallNextHookEx(keyboard_hook, nCode, wParam, lParam);

        if (wParam == WM_KEYDOWN || wParam == WM_SYSKEYDOWN)
        {
            int should_block = callback((int)p->vkCode, 1);
            if (should_block)
                return 1;
        }

        if (wParam == WM_KEYUP || wParam == WM_SYSKEYUP)
        {
            int should_block = callback((int)p->vkCode, 0);
            if (should_block)
                return 1;
        }
    }

    return CallNextHookEx(keyboard_hook, nCode, wParam, lParam);
}

DWORD WINAPI hook_thread_func(LPVOID param)
{
    keyboard_hook = SetWindowsHookEx(
        WH_KEYBOARD_LL,
        keyboard_proc,
        NULL,
        0
    );

    MSG msg;

    while (GetMessage(&msg, NULL, 0, 0))
    {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }

    return 0;
}

void start_listener()
{
    if (hook_thread != NULL)
        return;

    hook_thread = CreateThread(
        NULL,
        0,
        hook_thread_func,
        NULL,
        0,
        &hook_thread_id
    );
}

void stop_listener()
{
    if (hook_thread == NULL)
        return;

    PostThreadMessage(hook_thread_id, WM_QUIT, 0, 0);

    WaitForSingleObject(hook_thread, INFINITE);

    CloseHandle(hook_thread);
    hook_thread = NULL;

    if (keyboard_hook)
    {
        UnhookWindowsHookEx(keyboard_hook);
        keyboard_hook = NULL;
    }
}

void toggle_listener(int enabled)
{
    if (enabled)
        start_listener();
    else
        stop_listener();
}

void press_key(int vk)
{
    INPUT input = {0};

    input.type = INPUT_KEYBOARD;
    input.ki.wVk = vk;

    SendInput(1, &input, sizeof(INPUT));
}

void release_key(int vk)
{
    INPUT input = {0};

    input.type = INPUT_KEYBOARD;
    input.ki.wVk = vk;
    input.ki.dwFlags = KEYEVENTF_KEYUP;

    SendInput(1, &input, sizeof(INPUT));
}

void tap_key(int vk)
{
    press_key(vk);
    Sleep(10);
    release_key(vk);
}
