# Install script for directory: /home/siebencorgie/Scripts/Rust/engine/ori_engine/lib/vulkano/glsl-to-spirv/glslang/SPIRV

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/siebencorgie/Scripts/Rust/engine/ori_engine/examples/simple_scene/target/debug/build/glsl-to-spirv-fb92950b86bfc92b/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Debug")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "0")
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/home/siebencorgie/Scripts/Rust/engine/ori_engine/examples/simple_scene/target/debug/build/glsl-to-spirv-fb92950b86bfc92b/out/build/SPIRV/libSPIRV.a")
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/home/siebencorgie/Scripts/Rust/engine/ori_engine/examples/simple_scene/target/debug/build/glsl-to-spirv-fb92950b86bfc92b/out/build/SPIRV/libSPVRemapper.a")
endif()

if("${CMAKE_INSTALL_COMPONENT}" STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/SPIRV" TYPE FILE FILES
    "/home/siebencorgie/Scripts/Rust/engine/ori_engine/lib/vulkano/glsl-to-spirv/glslang/SPIRV/bitutils.h"
    "/home/siebencorgie/Scripts/Rust/engine/ori_engine/lib/vulkano/glsl-to-spirv/glslang/SPIRV/spirv.hpp"
    "/home/siebencorgie/Scripts/Rust/engine/ori_engine/lib/vulkano/glsl-to-spirv/glslang/SPIRV/GLSL.std.450.h"
    "/home/siebencorgie/Scripts/Rust/engine/ori_engine/lib/vulkano/glsl-to-spirv/glslang/SPIRV/GLSL.ext.KHR.h"
    "/home/siebencorgie/Scripts/Rust/engine/ori_engine/lib/vulkano/glsl-to-spirv/glslang/SPIRV/GlslangToSpv.h"
    "/home/siebencorgie/Scripts/Rust/engine/ori_engine/lib/vulkano/glsl-to-spirv/glslang/SPIRV/hex_float.h"
    "/home/siebencorgie/Scripts/Rust/engine/ori_engine/lib/vulkano/glsl-to-spirv/glslang/SPIRV/Logger.h"
    "/home/siebencorgie/Scripts/Rust/engine/ori_engine/lib/vulkano/glsl-to-spirv/glslang/SPIRV/SpvBuilder.h"
    "/home/siebencorgie/Scripts/Rust/engine/ori_engine/lib/vulkano/glsl-to-spirv/glslang/SPIRV/spvIR.h"
    "/home/siebencorgie/Scripts/Rust/engine/ori_engine/lib/vulkano/glsl-to-spirv/glslang/SPIRV/doc.h"
    "/home/siebencorgie/Scripts/Rust/engine/ori_engine/lib/vulkano/glsl-to-spirv/glslang/SPIRV/disassemble.h"
    "/home/siebencorgie/Scripts/Rust/engine/ori_engine/lib/vulkano/glsl-to-spirv/glslang/SPIRV/GLSL.ext.AMD.h"
    "/home/siebencorgie/Scripts/Rust/engine/ori_engine/lib/vulkano/glsl-to-spirv/glslang/SPIRV/GLSL.ext.NV.h"
    "/home/siebencorgie/Scripts/Rust/engine/ori_engine/lib/vulkano/glsl-to-spirv/glslang/SPIRV/SPVRemapper.h"
    "/home/siebencorgie/Scripts/Rust/engine/ori_engine/lib/vulkano/glsl-to-spirv/glslang/SPIRV/doc.h"
    )
endif()

