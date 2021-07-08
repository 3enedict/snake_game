#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include "Vulkan.h"

#include <iostream>

void createSurface(Vulkan& vulkan) {
  if (glfwCreateWindowSurface(vulkan.instance, vulkan.window, nullptr, &vulkan.surface) != VK_SUCCESS) {
    std::cerr << "Error : Failed to create window surface!" << std::endl;
    exit(EXIT_FAILURE);
  }
}
