project(wwww)

cmake_minimum_required(VERSION 2.8.6)

set(wwww_VERSION_MAJOR 0)
set(wwww_VERSION_MINOR 0)
set(wwww_VERSION_PATCH 1)

set(wwww_VERSION
  "${wwww_VERSION_MAJOR}.${wwww_VERSION_MINOR}.${wwww_VERSION_PATCH}"
)

# include specific modules
set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")
####################################
# compile commands  
####################################
set(CMAKE_EXPORT_COMPILE_COMMANDS ON) # works for Unix MakeFiles (also ninja)
set(BUILD_SHARED_LIBS OFF)
set(CMAKE_ERROR_DEPRECATED ON)
set(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION ON)
set(EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin")
####################################
# pre-compiled headers support
####################################
include(cotire)

####################################
# user specified build options
####################################
if(NOT CMAKE_CONFIGURATION_TYPES AND NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE "Release")
endif()

option(ENABLE_PACKAGING "Create build scripts for creating wwww packages" OFF)
option(ENABLE_TEST "Enable the tests" ON)

####################################
# bootstrap
####################################

include(TestCXXAcceptsFlag)
include(GNUInstallDirs)

set(LIB_DESTINATION
  "${CMAKE_INSTALL_FULL_LIBDIR}" CACHE STRING "Define lib output directory name"
)


####################################
# platform specific options
####################################
if(WIN32 OR WIN64)
  set(CMAKE_DEBUG_POSTFIX "d")
endif()

if(MSVC)
  # enable exceptions, see http://msdn.microsoft.com/en-us/library/1deeycx5.aspx
  add_definitions(-EHsc)
endif()

if(NOT WIN32 AND NOT CMAKE_SYSTEM MATCHES "SunOS-5*.")
  add_definitions(-fPIC)
endif()

if(CYGWIN)
  add_definitions(-D__LARGE64_FILES)
endif()

if(APPLE)
  set(CMAKE_MACOSX_RPATH ON)
  set(CMAKE_SKIP_BUILD_RPATH FALSE)
  set(CMAKE_BUILD_WITH_INSTALL_RPATH FALSE)
  set(CMAKE_INSTALL_RPATH "${CMAKE_INSTALL_PREFIX}/lib")
  set(CMAKE_INSTALL_RPATH_USE_LINK_PATH TRUE)
  list(FIND CMAKE_PLATFORM_IMPLICIT_LINK_DIRECTORIES "${CMAKE_INSTALL_PREFIX}/lib" isSystemDir)
  if("${isSystemDir}" STREQUAL "-1")
    set(CMAKE_INSTALL_RPATH "${CMAKE_INSTALL_PREFIX}/lib")
  endif()
endif()

#################################
# generate Config.h
#################################
configure_file(
  "${CMAKE_CURRENT_SOURCE_DIR}/Config.h.cmake"
  "${CMAKE_CURRENT_BINARY_DIR}/include/config.h" @ONLY
)

include_directories(
  "${CMAKE_CURRENT_BINARY_DIR}/include"
)
#################################
# compile flags
#################################
include_directories("${PROJECT_SOURCE_DIR}/include/")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS}  -Wall -Wextra -Wno-unused-parameter ")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -W -Wall -Wextra -Wunused-parameter -Wno-system-headers -Wno-deprecated")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wwrite-strings -Wundef -D_FORTIFY_SOURCE=2")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wuninitialized -Wparentheses")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wfloat-equal -Wstrict-overflow -Wstrict-overflow=5 -Wredundant-decls")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS}  -pedantic -pedantic-errors -std=c++14")
set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -O0 -g -ggdb ${COVERAGE_FLAGS}")

##Target configs (add static link lines to eahc toolchain file speerately)
set(CMAKE_FIND_LIBRARY_SUFFIXES ".a")
set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -O2")




#################################
# install pkg-config file
#################################
if(NOT WIN32)
  configure_file(
    "${CMAKE_CURRENT_SOURCE_DIR}/libwwww.pc.cmake"
    "${CMAKE_CURRENT_BINARY_DIR}/libwwww.pc" @ONLY)
  install(
    FILES
    "${CMAKE_CURRENT_BINARY_DIR}/libwwww.pc"
    DESTINATION "${LIB_DESTINATION}/pkgconfig")
endif()

#################################
# install Config.h
#################################
install(
  FILES
  "${CMAKE_CURRENT_BINARY_DIR}/include/config.h"
  DESTINATION include/wwww)

####################################
# custom targets
####################################
configure_file(
  "${CMAKE_MODULE_PATH}/cmake_uninstall.cmake.in"
  "${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake"
  IMMEDIATE @ONLY
)

add_custom_target(
  uninstall
  "${CMAKE_COMMAND}" -P "${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake"
  VERBATIM
)

if(ENABLE_PACKAGING)
  include(CreatewwwwPackages)
endif()

if(ENABLE_TEST)
  enable_testing()
endif()
add_subdirectory(src)

add_executable(wwww ${CMAKE_SOURCE_DIR}/src/wwww.cc)
set_target_properties(wwww PROPERTIES LINK_SEARCH_START_STATIC 1)
set_target_properties(wwww PROPERTIES LINK_SEARCH_END_STATIC 1)
cotire(wwww)


message("*********************** Build Summary *********************")
message("  Version:              ${wwww_VERSION}")
message("  Prefix:               ${CMAKE_INSTALL_PREFIX}")
message("  Build Type:           ${CMAKE_BUILD_TYPE}")
message("  Our Architecture:     ${CMAKE_HOST_SYSTEM_PROCESSOR}")
message("  Target system:        ${CMAKE_SYSTEM_NAME}")
message("  c Compiler:           ${CMAKE_C_COMPILER}")
message("  c++ Compiler:         ${CMAKE_CXX_COMPILER}")

message("***********************************************************")
message("Run ccmake or cmake-gui for available options.")
