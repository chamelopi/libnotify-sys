Wrapper library for `libnotify` on Linux.

Install required dependencies:

```
sudo apt install libgtk2.0-dev libgdk-pixbuf2.0-dev libnotify-dev
```

To generate bindings:

```
bindgen /usr/include/libnotify/notify.h -o src/bindings.rs -- -I/usr/include/glib-2.0 -I/usr/lib/x86_64-linux-gnu/glib-2.0/include -I/usr/include/gdk-pixbuf-2.0
```
