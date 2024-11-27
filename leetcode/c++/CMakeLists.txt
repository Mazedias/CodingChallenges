cmake_minimum_required(VERSION 3.14)
project(leetcodeTests)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(GTest REQUIRED)

enable_testing()

file(GLOB SOURCES "easy/*.cpp")

add_executable(${PROJECT_NAME} ${SOURCES})
target_link_libraries(${PROJECT_NAME} GTest::gtest GTest::gtest_main)

include(GoogleTest)
gtest_discover_tests(${PROJECT_NAME})