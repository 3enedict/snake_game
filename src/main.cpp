#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include "Vulkan.h"

#include <iostream>
#include <vector>
#include <cstring>
#include <cstdlib>

#ifdef NDEBUG
const bool debug = false;
#else
const bool debug = true;
#endif

int main() {
  Vulkan vulkan;

  vulkan.width = 1280;
  vulkan.height = 720;
  vulkan.name = "Snake";

  vulkan.enableValidationLayers = debug;
  vulkan.validationLayers = {
    "VK_LAYER_KHRONOS_validation"
  };

  vulkan.deviceExtensions = {
    VK_KHR_SWAPCHAIN_EXTENSION_NAME
  };

  initWindow(vulkan);
  initInstance(vulkan);
  createSurface(vulkan);
  pickPhysicalDevice(vulkan);
  createLogicalDevice(vulkan);

  VkPhysicalDeviceProperties properties;
  vkGetPhysicalDeviceProperties(vulkan.physicalDevice, &properties);
  std::cout << "Selected GPU name : " << properties.deviceName << std::endl;

  while (!glfwWindowShouldClose(vulkan.window)) {
    glfwPollEvents();
  }

  cleanup(vulkan);
}
