cmake_minimum_required(VERSION 3.10)

# Project name and version
project(main VERSION 1.0)

# Specify the C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Include directories
include_directories(include/)

# Link directories
link_directories(../target/debug/)

# Add the executable
add_executable(test src/main.cpp)

# Link the static library
target_link_libraries(test rust)

# Link system libraries
if (WIN32)
    target_link_libraries(test Ws2_32)
    target_link_libraries(test Advapi32)
    target_link_libraries(test Crypt32)
    target_link_libraries(test Userenv)
    target_link_libraries(test Ntdll)
    target_link_libraries(test Bcrypt)
endif()