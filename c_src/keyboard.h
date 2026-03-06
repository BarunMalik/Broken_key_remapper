#ifndef KEYBOARD_H
#define KEYBOARD_H

#ifdef __cplusplus
extern "C" {
#endif

typedef int (*key_callback)(int key, int state);

void register_key_callback(key_callback cb);

void start_listener();
void stop_listener();
void toggle_listener(int enabled);

void press_key(int vk);
void release_key(int vk);
void tap_key(int vk);

#ifdef __cplusplus
}
#endif

#endif
