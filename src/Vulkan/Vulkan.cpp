#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include "Vulkan.h"

#include <iostream>
#include <vector>
#include <cstring>
#include <optional>

void initWindow(Vulkan& vulkan) {
  glfwInit();

  glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
  glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE);

  vulkan.window = glfwCreateWindow(vulkan.width, vulkan.height, vulkan.name, nullptr, nullptr);
}

void cleanup(Vulkan& vulkan) {
  if (vulkan.enableValidationLayers)
    DestroyDebugUtilsMessengerEXT(vulkan.instance, vulkan.debugMessenger, nullptr);

  vkDestroyDevice(vulkan.device, nullptr);

  vkDestroySurfaceKHR(vulkan.instance, vulkan.surface, nullptr);

  vkDestroyInstance(vulkan.instance, nullptr);

  glfwDestroyWindow(vulkan.window);

  glfwTerminate();
}
