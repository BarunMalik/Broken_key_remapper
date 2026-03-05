#ifndef KEYBOARD_H
#define KEYBOARD_H

void start_keyboard_hook();
void stop_keyboard_hook();

void press_key(int vk);
void release_key(int vk);
void tap_key(int vk);

#endif
