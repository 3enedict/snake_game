#include "Vulkan.h"
#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <iostream>
#include <vector>
#include <cstring>
#include <cstdlib>

#ifdef NDEBUG
const bool enableValidationLayers = false;
#else
const bool enableValidationLayers = true;
#endif

int main() {
  uint32_t width = 1280;
  uint32_t height = 720;

  std::vector<const char*> validationLayers = {
    "VK_LAYER_KHRONOS_validation"
  };

  VkDebugUtilsMessengerEXT debugMessenger;

  GLFWwindow* window = initWindow(width, height);
  VkInstance instance = initVulkan(enableValidationLayers, validationLayers, &debugMessenger);

  while (!glfwWindowShouldClose(window)) {
    glfwPollEvents();
  }

  cleanup(enableValidationLayers, instance, window, debugMessenger);
}
