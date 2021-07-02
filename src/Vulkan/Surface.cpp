#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <Surface.h>

#include <iostream>

VkSurfaceKHR createSurface(VkInstance instance, GLFWwindow* window) {
  VkSurfaceKHR surface;

  if (glfwCreateWindowSurface(instance, window, nullptr, &surface) != VK_SUCCESS) {
    std::cerr << "Error : Failed to create window surface!" << std::endl;
    exit(EXIT_FAILURE);
  }

  return surface;
}
