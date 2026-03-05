#include <windows.h>
#include <stdio.h>
#include "keyboard.h"

static HHOOK keyboard_hook;

LRESULT CALLBACK keyboard_proc(int nCode, WPARAM wParam, LPARAM lParam)
{
    if (nCode == HC_ACTION)
    {
        KBDLLHOOKSTRUCT *p = (KBDLLHOOKSTRUCT *)lParam;

        if (wParam == WM_KEYDOWN)
        {
            printf("Key pressed: %d\n", p->vkCode);
        }

        if (wParam == WM_KEYUP)
        {
            printf("Key released: %d\n", p->vkCode);
        }
    }

    return CallNextHookEx(keyboard_hook, nCode, wParam, lParam);
}

void start_keyboard_hook()
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
}

void stop_keyboard_hook()
{
    UnhookWindowsHookEx(keyboard_hook);
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
