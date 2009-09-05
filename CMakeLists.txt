cmake_minimum_required(VERSION 2.6)
project (wwww)
#this needs to be at the top of the file
MESSAGE ("##############################################")
IF(NOT CMAKE_GEN)
  MESSAGE("")
  IF(UNIX)
    MESSAGE("No generator specified.  Use e.g. -G\"CodeBlocks - Unix Makefiles\"")
  ELSE(UNIX)
    MESSAGE("No generator specified.  Use e.g. -G\"CodeBlocks - MinGW Makefiles\"")
  ENDIF(UNIX)
  MESSAGE("")
ENDIF(NOT CMAKE_GEN)
SET(CMAKE_GEN ${CMAKE_GENERATOR} CACHE INTERNAL "" FORCE)

IF(NOT CMAKE_BUILD_TYPE)
   SET(CMAKE_BUILD_TYPE "Release" CACHE INTERNAL "Choose the type of build, options are: Debug Release RelWithDebInfo MinSizeRel." FORCE)
ENDIF(NOT CMAKE_BUILD_TYPE)


