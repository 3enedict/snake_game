#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include "Vulkan.h"

#include <iostream>
#include <vector>

void initInstance(Vulkan& vulkan) {
  /* Create vulkan instance */

  if (vulkan.enableValidationLayers && !checkValidationLayerSupport(vulkan)) {
    std::cerr << "Warning : Validation layers requested, but not available!" << std::endl;
    vulkan.enableValidationLayers = false;
  }

  VkApplicationInfo appInfo{};
  appInfo.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
  appInfo.pApplicationName = vulkan.name;
  appInfo.applicationVersion = VK_MAKE_VERSION(1, 0, 0);
  appInfo.pEngineName = "No Engine";
  appInfo.engineVersion = VK_MAKE_VERSION(1, 0, 0);
  appInfo.apiVersion = VK_API_VERSION_1_0;

  VkInstanceCreateInfo createInfo{};
  createInfo.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
  createInfo.pApplicationInfo = &appInfo;

  uint32_t glfwExtensionCount = 0;
  const char** glfwExtensions;
  glfwExtensions = glfwGetRequiredInstanceExtensions(&glfwExtensionCount);

  std::vector<const char*> extensions(glfwExtensions, glfwExtensions + glfwExtensionCount);

  if (vulkan.enableValidationLayers) {
    extensions.push_back(VK_EXT_DEBUG_UTILS_EXTENSION_NAME);
  }

  createInfo.enabledExtensionCount = static_cast<uint32_t>(extensions.size());
  createInfo.ppEnabledExtensionNames = extensions.data();

  VkDebugUtilsMessengerCreateInfoEXT debugCreateInfo{};
  if (vulkan.enableValidationLayers) {
    createInfo.enabledLayerCount = static_cast<uint32_t>(vulkan.validationLayers.size());
    createInfo.ppEnabledLayerNames = vulkan.validationLayers.data();

    populateDebugMessengerCreateInfo(debugCreateInfo);
    createInfo.pNext = (VkDebugUtilsMessengerCreateInfoEXT*) &debugCreateInfo;
  } else {
    createInfo.enabledLayerCount = 0;

    createInfo.pNext = nullptr;
  }

  if (vkCreateInstance(&createInfo, nullptr, &vulkan.instance) != VK_SUCCESS) {
    std::cerr << "Error: Failed to create instance!" << std::endl;
    exit(EXIT_FAILURE);
  }

  /* Init validation layers */

  if (!vulkan.enableValidationLayers) return;

  VkDebugUtilsMessengerCreateInfoEXT layercreateInfo;
  populateDebugMessengerCreateInfo(layercreateInfo);

  if (CreateDebugUtilsMessengerEXT(vulkan.instance, &layercreateInfo, nullptr, &vulkan.debugMessenger) != VK_SUCCESS) {
    std::cerr << "Warning : failed to set up debug messenger!" << std::endl;
  }
}
