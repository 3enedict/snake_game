You know how people LOVE over engineering stuff ? Well, this is that, but in programming. Let me explain. This project is essentially the famous Snake Game : every game developer's "Hello World" program. But... the twist is : Vulkan, which, if you don't know yet is a low level graphics API that rocks. The result of all that is kind of like when you see someone becoming a bodybuilder just because he or she wants to be able to type more easily on one of those clicky keyboards. Insane right ? Anyways, hope you enjoy this lovely and over engineered game.

# IMPORTANT

THIS PROJECT IS STILL IN DEVELOPMENT, AND MAY BE SO FOR A BIT !! Currently it's only able to create a triangle (and it ain't perfect yet, so...)

# Downloading and building

This program depends on a library that I'm still creating, namely, vgl. As a consequence, I have decided to not fully integrate into cargo because it is still in it's infancy. So for now, just download it separately as shown here : 

```bash
git clone https://github.com/BPN06/vgl
git clone https://github.com/BPN06/snake_game
cd snake_game
cargo run
```
<!-- 
# Testing

```bash
cargo test -- --test-threads=1
```

The test-threads option needed to be added because winit only supports being run in the main thread. 
-->
