#include "Vulkan.h"

#include <cstdint>
#include <algorithm>
#include <iostream>

void querySwapChainSupport(VkPhysicalDevice device, Vulkan& vulkan) {
  vkGetPhysicalDeviceSurfaceCapabilitiesKHR(device, vulkan.surface, &vulkan.swapchainDetails.capabilities);

  uint32_t formatCount;
  vkGetPhysicalDeviceSurfaceFormatsKHR(device, vulkan.surface, &formatCount, nullptr);

  if (formatCount != 0) {
    vulkan.swapchainDetails.formats.resize(formatCount);
    vkGetPhysicalDeviceSurfaceFormatsKHR(device, vulkan.surface, &formatCount, vulkan.swapchainDetails.formats.data());
  }

  uint32_t presentModeCount;
  vkGetPhysicalDeviceSurfacePresentModesKHR(device, vulkan.surface, &presentModeCount, nullptr);

  if (presentModeCount != 0) {
    vulkan.swapchainDetails.presentModes.resize(presentModeCount);
    vkGetPhysicalDeviceSurfacePresentModesKHR(device, vulkan.surface, &presentModeCount, vulkan.swapchainDetails.presentModes.data());
  }
}

VkSurfaceFormatKHR chooseSwapSurfaceFormat(const std::vector<VkSurfaceFormatKHR>& availableFormats) {
  for (const auto& availableFormat : availableFormats) {
    if (availableFormat.format == VK_FORMAT_B8G8R8A8_SRGB && availableFormat.colorSpace == VK_COLOR_SPACE_SRGB_NONLINEAR_KHR) {
      return availableFormat;
    }
  }

  return availableFormats[0];
}

VkPresentModeKHR chooseSwapPresentMode(const std::vector<VkPresentModeKHR>& availablePresentModes) {
  for (const auto& availablePresentMode : availablePresentModes) {
    if (availablePresentMode == VK_PRESENT_MODE_MAILBOX_KHR) {
      return availablePresentMode;
    }
  }

  return VK_PRESENT_MODE_FIFO_KHR;
}

VkExtent2D chooseSwapExtent(const VkSurfaceCapabilitiesKHR& capabilities, Vulkan& vulkan) {
  if (capabilities.currentExtent.width != UINT32_MAX) {
    return capabilities.currentExtent;
  } else {
    int width, height;
    glfwGetFramebufferSize(vulkan.window, &width, &height);

    VkExtent2D actualExtent = {
      static_cast<uint32_t>(width),
      static_cast<uint32_t>(height)
    };

    actualExtent.width = std::clamp(actualExtent.width, capabilities.minImageExtent.width, capabilities.maxImageExtent.width);
    actualExtent.height = std::clamp(actualExtent.height, capabilities.minImageExtent.height, capabilities.maxImageExtent.height);

    return actualExtent;
  }
}

void createSwapChain(Vulkan& vulkan) {
  querySwapChainSupport(vulkan.physicalDevice, vulkan);

  VkSurfaceFormatKHR surfaceFormat = chooseSwapSurfaceFormat(vulkan.swapchainDetails.formats);
  VkPresentModeKHR presentMode = chooseSwapPresentMode(vulkan.swapchainDetails.presentModes);
  VkExtent2D extent = chooseSwapExtent(vulkan.swapchainDetails.capabilities, vulkan);

  uint32_t imageCount = vulkan.swapchainDetails.capabilities.minImageCount + 2;
  if (vulkan.swapchainDetails.capabilities.maxImageCount > 0 && imageCount > vulkan.swapchainDetails.capabilities.maxImageCount)
    imageCount = vulkan.swapchainDetails.capabilities.maxImageCount;

  VkSwapchainCreateInfoKHR createInfo{};
  createInfo.sType = VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;
  createInfo.surface = vulkan.surface;
  createInfo.minImageCount = imageCount;
  createInfo.imageFormat = surfaceFormat.format;
  createInfo.imageColorSpace = surfaceFormat.colorSpace;
  createInfo.imageExtent = extent;
  createInfo.imageArrayLayers = 1;
  createInfo.imageUsage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;

  findQueueFamilies(vulkan.physicalDevice, vulkan);
  uint32_t queueFamilyIndices[] = {vulkan.indices.graphicsFamily.value(), vulkan.indices.presentFamily.value()};

  if (vulkan.indices.graphicsFamily != vulkan.indices.presentFamily) {
    createInfo.imageSharingMode = VK_SHARING_MODE_CONCURRENT;
    createInfo.queueFamilyIndexCount = 2;
    createInfo.pQueueFamilyIndices = queueFamilyIndices;
  } else {
    createInfo.imageSharingMode = VK_SHARING_MODE_EXCLUSIVE;
    createInfo.queueFamilyIndexCount = 0;
    createInfo.pQueueFamilyIndices = nullptr;
  }

  createInfo.preTransform = vulkan.swapchainDetails.capabilities.currentTransform;

  createInfo.compositeAlpha = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;

  createInfo.presentMode = presentMode;
  createInfo.clipped = VK_TRUE;

  createInfo.oldSwapchain = VK_NULL_HANDLE;
  if (vkCreateSwapchainKHR(vulkan.device, &createInfo, nullptr, &vulkan.swapChain) != VK_SUCCESS) {
    std::cout << "Error: Failed to create swap chain" << std::endl;
    exit(EXIT_FAILURE);
  }

}
