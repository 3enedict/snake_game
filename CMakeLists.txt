cmake_minimum_required(VERSION 3.0)

project(Snake)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

file(GLOB_RECURSE source RELATIVE ${CMAKE_CURRENT_SOURCE_DIR}
  src/*.cpp
  include/*.h)

add_executable(Snake ${source})
target_include_directories(Snake PUBLIC include/)

find_package(Vulkan EXACT REQUIRED)

target_link_libraries(Snake PUBLIC glfw Vulkan::Vulkan)
