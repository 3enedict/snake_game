#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <Instance.h>
#include <ValidationLayers.h>

#include <iostream>
#include <vector>

VkInstance initVulkan(bool enableValidationLayers, std::vector<const char*> validationLayers, VkDebugUtilsMessengerEXT* debugMessenger) {
  /* Create vulkan instance */

  if (enableValidationLayers && !checkValidationLayerSupport(validationLayers)) {
    std::cerr << "Warning : Validation layers requested, but not available!" << std::endl;
    enableValidationLayers = false;
  }

  VkApplicationInfo appInfo{};
  appInfo.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
  appInfo.pApplicationName = "Hello Triangle";
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

  if (enableValidationLayers) {
    extensions.push_back(VK_EXT_DEBUG_UTILS_EXTENSION_NAME);
  }

  createInfo.enabledExtensionCount = static_cast<uint32_t>(extensions.size());
  createInfo.ppEnabledExtensionNames = extensions.data();

  VkDebugUtilsMessengerCreateInfoEXT debugCreateInfo{};
  if (enableValidationLayers) {
    createInfo.enabledLayerCount = static_cast<uint32_t>(validationLayers.size());
    createInfo.ppEnabledLayerNames = validationLayers.data();

    populateDebugMessengerCreateInfo(debugCreateInfo);
    createInfo.pNext = (VkDebugUtilsMessengerCreateInfoEXT*) &debugCreateInfo;
  } else {
    createInfo.enabledLayerCount = 0;

    createInfo.pNext = nullptr;
  }

  VkInstance instance;

  if (vkCreateInstance(&createInfo, nullptr, &instance) != VK_SUCCESS) {
    std::cerr << "Error : Failed to create instance!" << std::endl;
    exit(EXIT_FAILURE);
  }

  /* Init validation layers */

  if (!enableValidationLayers) return instance;

  VkDebugUtilsMessengerCreateInfoEXT layercreateInfo;
  populateDebugMessengerCreateInfo(layercreateInfo);

  if (CreateDebugUtilsMessengerEXT(instance, &layercreateInfo, nullptr, debugMessenger) != VK_SUCCESS) {
    std::cerr << "Warning : failed to set up debug messenger!" << std::endl;
  }

  return instance;
}
