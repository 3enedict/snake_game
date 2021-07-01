#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <Device_Queues.h>

#include <optional>
#include <iostream>
#include <vector>

bool QueueFamilyIndices::isComplete() {
  return graphicsFamily.has_value();
}

QueueFamilyIndices findQueueFamilies(VkPhysicalDevice device) {
  QueueFamilyIndices indices;

  uint32_t queueFamilyCount = 0;
  vkGetPhysicalDeviceQueueFamilyProperties(device, &queueFamilyCount, nullptr);

  std::vector<VkQueueFamilyProperties> queueFamilies(queueFamilyCount);
  vkGetPhysicalDeviceQueueFamilyProperties(device, &queueFamilyCount, queueFamilies.data());

  int i = 0;
  for (const auto& queueFamily : queueFamilies) {
    if (queueFamily.queueFlags & VK_QUEUE_GRAPHICS_BIT)
      indices.graphicsFamily = i;

    if (indices.isComplete())
      break;

    i++;
  }

  return indices;
}

bool isDeviceSuitable(VkPhysicalDevice device) {
  QueueFamilyIndices indices = findQueueFamilies(device);

  return indices.isComplete();
}

VkPhysicalDevice pickPhysicalDevice(VkInstance instance) {
  uint32_t deviceCount = 0;
  vkEnumeratePhysicalDevices(instance, &deviceCount, nullptr);

  if (deviceCount == 0) {
    std::cerr << "Error : Failed to find GPUs with Vulkan support!" << std::endl;
    exit(EXIT_FAILURE);
  }

  std::vector<VkPhysicalDevice> devices(deviceCount);
  vkEnumeratePhysicalDevices(instance, &deviceCount, devices.data());

  for (const auto& device : devices) {
    if (isDeviceSuitable(device)) {
      return device;
    }
  }

  std::cerr << "Error : Failed to find a suitable GPU!" << std::endl;
  exit(EXIT_FAILURE);
}
