#pragma once

#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <vector>
#include <cstdint>
#include <optional>

/* The QueueFamilyIndices structure is part of the Device and Queues section */

struct QueueFamilyIndices {
  std::optional<uint32_t> graphicsFamily;
  std::optional<uint32_t> presentFamily;

  bool isComplete();
};

struct Vulkan {
  uint32_t width;
  uint32_t height;
  const char* name;

  std::vector<const char*> validationLayers;
  bool enableValidationLayers;
  VkDebugUtilsMessengerEXT debugMessenger;

  GLFWwindow* window;
  VkInstance instance;
  VkSurfaceKHR surface;
  VkPhysicalDevice physicalDevice;
  VkDevice device;

  QueueFamilyIndices indices;
  VkQueue graphicsQueue;
  VkQueue presentQueue;
};

void initWindow(Vulkan& vulkan);
void cleanup(Vulkan& vulkan);

/* Instance */

void initInstance(Vulkan& vulkan);

/* Validation layers */

VkResult CreateDebugUtilsMessengerEXT(VkInstance instance, const VkDebugUtilsMessengerCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDebugUtilsMessengerEXT* pDebugMessenger);

void DestroyDebugUtilsMessengerEXT(VkInstance instance, VkDebugUtilsMessengerEXT debugMessenger, const VkAllocationCallbacks* pAllocator);

void populateDebugMessengerCreateInfo(VkDebugUtilsMessengerCreateInfoEXT& createInfo);

bool checkValidationLayerSupport(Vulkan& vulkan);

/* Surface */

void createSurface(Vulkan& vulkan);

/* Device and Queues */

void findQueueFamilies(VkPhysicalDevice device, Vulkan& vulkan);
bool isDeviceSuitable(VkPhysicalDevice device, Vulkan& vulkan);
void pickPhysicalDevice(Vulkan& vulkan);
void createLogicalDevice(Vulkan& vulkan);
