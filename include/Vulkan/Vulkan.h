#pragma once

#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <vector>


GLFWwindow* initWindow(uint32_t width, uint32_t height);
void cleanup(bool enableValidationLayers, VkInstance instance, GLFWwindow* window, VkDebugUtilsMessengerEXT debugMessenger, VkDevice device, VkSurfaceKHR surface);
