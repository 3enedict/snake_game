#pragma once

#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <vector>

VkInstance initVulkan(bool enableValidationLayers, std::vector<const char*> validationLayers, VkDebugUtilsMessengerEXT* debugMessenger);
