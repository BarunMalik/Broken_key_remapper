#include <stdio.h>
#include <windows.h>
#include "keyboard.h"

void key_handler(int key, int state)
{
    printf("key %d state %d\n", key, state);
}

int main()
{
    register_key_callback(key_handler);

    printf("Starting listener...\n");
    toggle_listener(1);

    Sleep(5000);

    printf("Stopping listener...\n");
    toggle_listener(0);

    printf("Testing key simulation...\n");
    tap_key('A');

    printf("Done\n");

    return 0;
}
