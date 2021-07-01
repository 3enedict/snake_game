#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <Vulkan.h>
#include <ValidationLayers.h>

#include <iostream>
#include <vector>
#include <cstring>
#include <optional>

GLFWwindow* initWindow(uint32_t width, uint32_t height) {
  glfwInit();

  glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
  glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE);

  GLFWwindow* window = glfwCreateWindow(width, height, "Snake", nullptr, nullptr);

  return window;
}

void cleanup(bool enableValidationLayers, VkInstance instance, GLFWwindow* window, VkDebugUtilsMessengerEXT debugMessenger) {
  if (enableValidationLayers) {
    DestroyDebugUtilsMessengerEXT(instance, debugMessenger, nullptr);
  }

  vkDestroyInstance(instance, nullptr);

  glfwDestroyWindow(window);

  glfwTerminate();
}
