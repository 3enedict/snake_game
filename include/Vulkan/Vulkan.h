#pragma once

#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <vector>


GLFWwindow* initWindow(uint32_t width, uint32_t height);
VkInstance initVulkan(bool enableValidationLayers, std::vector<const char*> validationLayers, VkDebugUtilsMessengerEXT* debugMessenger);
VkPhysicalDevice pickPhysicalDevice(VkInstance instance);
void cleanup(bool enableValidationLayers, VkInstance instance, GLFWwindow* window, VkDebugUtilsMessengerEXT debugMessenger);
