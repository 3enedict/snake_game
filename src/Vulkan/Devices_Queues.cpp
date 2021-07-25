#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include "Vulkan.h"

#include <cstdlib>
#include <optional>
#include <iostream>
#include <vector>
#include <set>

/* Physical devices and queue families */

bool QueueFamilyIndices::isComplete() {
  return graphicsFamily.has_value() && presentFamily.has_value();
}

void findQueueFamilies(VkPhysicalDevice device, Vulkan& vulkan) {
  if (vulkan.indices.isComplete())
    return;

  uint32_t queueFamilyCount = 0;
  vkGetPhysicalDeviceQueueFamilyProperties(device, &queueFamilyCount, nullptr);

  std::vector<VkQueueFamilyProperties> queueFamilies(queueFamilyCount);
  vkGetPhysicalDeviceQueueFamilyProperties(device, &queueFamilyCount, queueFamilies.data());

  int i = 0;
  for (const auto& queueFamily : queueFamilies) {
    if (queueFamily.queueFlags & VK_QUEUE_GRAPHICS_BIT)
      vulkan.indices.graphicsFamily = i;

    VkBool32 presentSupport = false;
    vkGetPhysicalDeviceSurfaceSupportKHR(device, i, vulkan.surface, &presentSupport);

    if (presentSupport) 
      vulkan.indices.presentFamily = i;

    if (vulkan.indices.isComplete())
      break;

    i++;
  }
}

bool isDeviceSuitable(VkPhysicalDevice device, Vulkan& vulkan) {
  findQueueFamilies(device, vulkan);

  uint32_t extensionCount;
  vkEnumerateDeviceExtensionProperties(device, nullptr, &extensionCount, nullptr);

  std::vector<VkExtensionProperties> availableExtensions(extensionCount);
  vkEnumerateDeviceExtensionProperties(device, nullptr, &extensionCount, availableExtensions.data());

  std::set<std::string> requiredExtensions(vulkan.deviceExtensions.begin(), vulkan.deviceExtensions.end());

  for (const auto& extension : availableExtensions) {
    requiredExtensions.erase(extension.extensionName);
  }

  return vulkan.indices.isComplete() && requiredExtensions.empty();
}

void pickPhysicalDevice(Vulkan& vulkan) {
  uint32_t deviceCount = 0;
  vkEnumeratePhysicalDevices(vulkan.instance, &deviceCount, nullptr);

  if (deviceCount == 0) {
    std::cerr << "Error : Failed to find GPUs with Vulkan support!" << std::endl;
    exit(EXIT_FAILURE);
  }

  std::vector<VkPhysicalDevice> devices(deviceCount);
  vkEnumeratePhysicalDevices(vulkan.instance, &deviceCount, devices.data());

  for (const auto& device : devices) {
    if (isDeviceSuitable(device, vulkan)) {
      vulkan.physicalDevice = device;
      return;
    }
  }

  std::cerr << "Error : Failed to find a suitable GPU!" << std::endl;
  exit(EXIT_FAILURE);
}

/* Logical device and queues */

void createLogicalDevice(Vulkan& vulkan) {
  findQueueFamilies(vulkan.physicalDevice, vulkan);

  std::vector<VkDeviceQueueCreateInfo> queueCreateInfos;
  std::set<uint32_t> uniqueQueueFamilies = {vulkan.indices.graphicsFamily.value(), vulkan.indices.presentFamily.value()};

  float queuePriority = 1.0f;
  for (uint32_t queueFamily : uniqueQueueFamilies) {
    VkDeviceQueueCreateInfo queueCreateInfo{};
    queueCreateInfo.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
    queueCreateInfo.queueFamilyIndex = queueFamily;
    queueCreateInfo.queueCount = 1;
    queueCreateInfo.pQueuePriorities = &queuePriority;
    queueCreateInfos.push_back(queueCreateInfo);
  }

  VkDeviceQueueCreateInfo queueCreateInfo{};
  queueCreateInfo.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
  queueCreateInfo.queueFamilyIndex = vulkan.indices.graphicsFamily.value();
  queueCreateInfo.queueCount = 1;

  queueCreateInfo.pQueuePriorities = &queuePriority;

  VkPhysicalDeviceFeatures deviceFeatures{};

  VkDeviceCreateInfo createInfo{};
  createInfo.sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;

  createInfo.queueCreateInfoCount = static_cast<uint32_t>(queueCreateInfos.size());
  createInfo.pQueueCreateInfos = queueCreateInfos.data();

  createInfo.pEnabledFeatures = &deviceFeatures;

  createInfo.enabledExtensionCount = 0;

  if (vulkan.enableValidationLayers) {
    createInfo.enabledLayerCount = static_cast<uint32_t>(vulkan.validationLayers.size());
    createInfo.ppEnabledLayerNames = vulkan.validationLayers.data();
  } else {
    createInfo.enabledLayerCount = 0;
  }

  VkDevice device;

  if (vkCreateDevice(vulkan.physicalDevice, &createInfo, nullptr, &device) != VK_SUCCESS) {
    std::cerr << "Error : Failed to create logical device!" << std::endl;
    exit(EXIT_FAILURE);
  }

  vkGetDeviceQueue(device, vulkan.indices.graphicsFamily.value(), 0, &vulkan.graphicsQueue);
  vkGetDeviceQueue(device, vulkan.indices.presentFamily.value(), 0, &vulkan.presentQueue);

  vulkan.device = device;
}
