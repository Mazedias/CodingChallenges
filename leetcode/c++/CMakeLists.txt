cmake_minimum_required(VERSION 3.14)
project(LeetcodeSolutions)

# Set C++ standard
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(GTest REQUIRED)

enable_testing()

# Include directories
include_directories(${CMAKE_SOURCE_DIR}/util)

# Find all source files in each difficulty folder
file(GLOB EASY_SOURCES "easy/*.cpp")
file(GLOB MEDIUM_SOURCES "medium/*.cpp")
file(GLOB HARD_SOURCES "hard/*.cpp")

# Create executables for each difficulty level
add_executable(${PROJECT_NAME}_easy ${EASY_SOURCES})
add_executable(${PROJECT_NAME}_medium ${MEDIUM_SOURCES})
add_executable(${PROJECT_NAME}_hard ${HARD_SOURCES})

target_link_libraries(${PROJECT_NAME}_easy GTest::gtest GTest::gtest_main)
target_link_libraries(${PROJECT_NAME}_medium GTest::gtest GTest::gtest_main)
target_link_libraries(${PROJECT_NAME}_hard GTest::gtest GTest::gtest_main)

include(GoogleTest)
gtest_discover_tests(${PROJECT_NAME}_easy)
gtest_discover_tests(${PROJECT_NAME}_medium)
gtest_discover_tests(${PROJECT_NAME}_hard)