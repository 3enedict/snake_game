#pragma once

#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <optional>
#include <vector>

struct QueueFamilyIndices {
  std::optional<uint32_t> graphicsFamily;

  bool isComplete();
};

QueueFamilyIndices findQueueFamilies(VkPhysicalDevice device);

bool isDeviceSuitable(VkPhysicalDevice device);

VkPhysicalDevice pickPhysicalDevice(VkInstance instance);

VkDevice createLogicalDevice(VkPhysicalDevice physicalDevice, std::vector<const char*> validationLayers, bool enableValidationLayers);
