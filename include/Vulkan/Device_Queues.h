#pragma once

#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <optional>
#include <vector>

struct QueueFamilyIndices {
  std::optional<uint32_t> graphicsFamily;
  std::optional<uint32_t> presentFamily;

  bool isComplete();
};

QueueFamilyIndices findQueueFamilies(VkPhysicalDevice device, VkSurfaceKHR surface);

bool isDeviceSuitable(VkPhysicalDevice device, VkSurfaceKHR surface);

VkPhysicalDevice pickPhysicalDevice(VkInstance instance, VkSurfaceKHR surface);

VkDevice createLogicalDevice(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, VkQueue& graphicsQueue, VkQueue& presentQueue, std::vector<const char*> validationLayers, bool enableValidationLayers);
