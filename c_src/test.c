#include <windows.h>
#include "keyboard.h"

int main()
{
    // simulate pressing A
    tap_key('A');

    // start global keyboard capture
    start_keyboard_hook();

    return 0;
}
